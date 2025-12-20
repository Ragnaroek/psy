#[cfg(test)]
#[path = "./assembler_test.rs"]
mod assembler_test;

use crate::arch::sm83::{
    self, INSTR_DEC_A, INSTR_DEC_B, INSTR_DEC_DE, INSTR_DEC_HL, INSTR_INC_A, INSTR_INC_DE,
    INSTR_INC_HL, INSTR_LD_TO_A_FROM_DEREF_HL, INSTR_LD_TO_B_FROM_IMMEDIATE,
    INSTR_LD_TO_DE_FROM_LABEL, INSTR_LD_TO_DEREF_DE_FROM_A, INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE,
    INSTR_LD_TO_HL_FROM_LABEL,
};
use crate::asm::parser::{Address, Form, Label, SExp, Symbol, TopLevel, parse_from_file};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Options {
    pub flat: bool,
    pub out: PathBuf,
}

#[derive(Debug)]
struct Memory {
    pub mem: Vec<u8>,
    pub mem_ptr: usize, // where the assembler is currently defining memory, always points an the next free mem
}

impl Memory {
    fn push_u8(&mut self, v: u8) {
        self.mem[self.mem_ptr] = v;
        self.mem_ptr += 1;
    }

    fn push_u16(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self.push_u8(bytes[0]);
        self.push_u8(bytes[1]);
    }
}

#[derive(Debug)]
struct Section {
    name: String,
    offset: Address,
    length: Option<u64>,
    label_only: bool,
    memory: Memory,
}

#[derive(Debug)]
struct State {
    sections: Vec<Section>,
    current_section_name: Option<String>,
    current_section_address: Address,
    label_addresses: HashMap<Label, Address>,
}

#[derive(Debug)]
struct UnresolvedLabel {
    relative_from: Option<Address>, // None means absolute jump
    label: Label,
    check: fn(i32) -> Result<(), String>,
    sec_name: String,
    patch_index: usize,
    patch_width: usize, // in bytes
}

impl State {
    fn new() -> State {
        State {
            sections: Vec::new(),
            current_section_name: None,
            current_section_address: Address(0),
            label_addresses: HashMap::new(),
        }
    }

    fn lookup_section(&self, name: &str) -> Option<&Section> {
        for section in &self.sections {
            if section.name == name {
                return Some(section);
            }
        }
        None
    }

    fn lookup_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        for section in &mut self.sections {
            if section.name == name {
                return Some(section);
            }
        }
        None
    }
}

pub fn assemble(pasm: TopLevel, options: Options) -> Result<(), String> {
    let mut state = State::new();
    assemble_in_state(pasm, &mut state)?;

    if options.flat {
        state_to_flat(&mut state, &options.out)
    } else {
        Err("object file assembly currently not supported".to_string())
    }
}

fn state_to_flat(state: &mut State, out: &Path) -> Result<(), String> {
    state.sections.sort_by_key(|section| section.offset);

    let mut out_file = File::create(out).map_err(|e| e.to_string())?;

    let mut last_written = 0;
    for sec in &state.sections {
        if sec.label_only {
            continue;
        }

        let fill_length = sec.offset.0 - last_written;
        if fill_length > 0 {
            let fill = vec![0; fill_length as usize];
            out_file.write(&fill).map_err(|e| e.to_string())?;
        }
        out_file.write(&sec.memory.mem).map_err(|e| e.to_string())?;
        last_written = sec.offset.0
            + sec
                .length
                .expect("flat assembly needs sections with specified length");
    }

    Ok(())
}

fn assemble_in_state(pasm: TopLevel, state: &mut State) -> Result<(), String> {
    let mut unresolved_refs: Vec<UnresolvedLabel> = Vec::new();

    for form in &pasm.forms {
        // define the label with an adresse if the form is labeled
        if let Some(lbl) = &form.label {
            define_label(state, lbl.clone())?;
        }

        let unresolved = match &form.op {
            Symbol::Sym(sym_name) => {
                if sym_name == "include" {
                    include(state, form)?
                } else if sym_name == "def-section" {
                    def_section(state, form)?
                } else if sym_name == "section" {
                    section(state, form)?
                } else if sym_name == "db" {
                    db(state, form)?
                } else if sym_name == "ds" {
                    ds(state, form)?
                } else if sym_name == "label" {
                    label(state, form)?
                } else if sym_name == "sub-section" {
                    sub_section(state)?
                // the following forms are tempoarily handled here. Plan is
                // to convert this to macros that emits bytes with low-level primitives
                } else if sym_name == "ld" {
                    //machine specific, should not be handled here
                    ld(state, form)?
                } else if sym_name == "jp" {
                    jp(state, form)?
                } else if sym_name == "inc" {
                    inc(state, form)?
                } else if sym_name == "dec" {
                    dec(state, form)?
                } else if sym_name == "jr" {
                    jr(state, form)?
                } else if sym_name == "nop" {
                    nop(state)?
                } else {
                    return Err(format!("unknown top-level: {:?}", sym_name));
                }
            }
            sym => return Err(format!("illegal top-level form: {:?}", sym)),
        };

        if let Some(unresolved) = unresolved {
            unresolved_refs.push(unresolved);
        }
    }

    try_resolve(&unresolved_refs, state)?;

    Ok(())
}

fn define_label(state: &mut State, label: Label) -> Result<(), String> {
    if state.label_addresses.contains_key(&label) {
        return Err(format!("duplicate label definition: '{}", label.name()));
    }

    state
        .label_addresses
        .insert(label, state.current_section_address);

    Ok(())
}

fn try_resolve(unresolved_refs: &Vec<UnresolvedLabel>, state: &mut State) -> Result<(), String> {
    for unresolved in unresolved_refs {
        let lbl_address = expect_label_address(state, &unresolved.label)?;

        let dist = if let Some(rel_dist) = unresolved.relative_from {
            (lbl_address as i32 - rel_dist.0 as i32) / 8
        } else {
            lbl_address as i32
        };
        (unresolved.check)(dist)?;
        let sec = state
            .lookup_section_mut(&unresolved.sec_name)
            .expect("source section not found");
        match unresolved.patch_width {
            1 => sec.memory.mem[unresolved.patch_index] = dist as u8,
            2 => {
                let bytes = dist.to_le_bytes();
                sec.memory.mem[unresolved.patch_index] = bytes[0];
                sec.memory.mem[unresolved.patch_index + 1] = bytes[1];
            }
            _ => {
                return Err(format!(
                    "unsupported patch_width: {}",
                    unresolved.patch_width
                ));
            }
        }
    }
    Ok(())
}

fn include(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() < 2 {
        return Err("include must at least provide file to include".to_string());
    }

    let file_name = if is_keyword(&form.exps[0], "std") {
        expect_has_sexp_at(&form.exps, 1, "std include path required")?;
        let std_file = expect_string(&form.exps[1])?;
        format!("stdlib/{}.asm", std_file)
    } else {
        expect_has_sexp_at(&form.exps, 0, "include path required")?;
        let file = expect_string(&form.exps[0])?;
        format!("{}.asm", file)
    };

    let mut file = File::open(file_name).map_err(|e| e.to_string())?;
    let tl = parse_from_file(&mut file)?;
    assemble_in_state(tl, state)?;
    Ok(None)
}

fn def_section(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() == 0 {
        return Err("illegal def-section".to_string());
    }

    let name = expect_section_name(&form.exps[0])?;
    let offset = Address(expect_immediate_value_or(
        key_value(&form.exps, "offset")?,
        0,
    )?);

    let may_length = key_value(&form.exps, "length")?;
    let length = if let Some(exp) = may_length {
        Some(expect_immediate(exp)?)
    } else {
        None
    };

    let false_default = &sym_false();
    let label_only_sym = expect_symbol_or(key_value(&form.exps, "label-only")?, false_default)?;
    let label_only = expect_bool_sym(label_only_sym)?;

    let memory = if let Some(len) = length {
        Memory {
            mem: vec![0; len as usize],
            mem_ptr: 0,
        }
    } else {
        Memory {
            mem: Vec::new(),
            mem_ptr: 0,
        }
    };

    state.sections.push(Section {
        name,
        offset,
        length,
        label_only,
        memory,
    });
    Ok(None)
}

fn section(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() != 1 {
        return Err("illegal section".to_string());
    }

    let name = expect_section_name(&form.exps[0])?;
    let may_section = state.lookup_section(&name);
    if let Some(section) = may_section {
        let addr = section.offset;
        let name = section.name.clone();
        state.current_section_address = addr;
        state.current_section_name = Some(name);
        Ok(None)
    } else {
        Err(format!("no such section: {}", name))
    }
}

fn db(state: &mut State, db: &Form) -> Result<Option<UnresolvedLabel>, String> {
    expect_in_section(state)?;
    if !db.exps.is_empty() {
        state
            .current_section_address
            .add_bytes(db.exps.len() as u64);
        let sec = expect_in_w_sec(state)?;
        for exp in &db.exps {
            let v = expect_immediate(exp)?;
            sec.memory.push_u8(v as u8);
        }
    }
    Ok(None)
}

fn ds(state: &mut State, ds: &Form) -> Result<Option<UnresolvedLabel>, String> {
    expect_in_section(state)?;

    if ds.exps.is_empty() {
        return Err("ds needs at least a len".to_string());
    }

    let len = expect_immediate(&ds.exps[0])?;
    state.current_section_address.add_bytes(len);
    let sec = expect_in_w_sec(state)?;
    for _ in 0..len {
        sec.memory.push_u8(0);
    }

    Ok(None)
}

fn label(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() < 1 {
        return Err("label: needs at one argument".to_string());
    }

    let may_label = is_label(&form.exps[0]);
    if let Some(lbl) = may_label {
        define_label(state, lbl.clone())?;
        Ok(None)
    } else {
        Err("label: need a label as argument".to_string())
    }
}

fn sub_section(state: &mut State) -> Result<Option<UnresolvedLabel>, String> {
    println!("!sub-section");
    Ok(None)
}

// non-primitive forms, temporarily implemented in Rust directly

fn nop(state: &mut State) -> Result<Option<UnresolvedLabel>, String> {
    state.current_section_address.add_bytes(1);
    let sec = expect_in_w_sec(state)?;
    sec.memory.push_u8(0);
    Ok(None)
}

fn ld(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() < 2 {
        return Err("ld: needs at least two arguments".to_string());
    }
    match (&form.exps[0], &form.exps[1]) {
        (SExp::Symbol(Symbol::Reg(reg)), SExp::Symbol(Symbol::Label(lbl))) => {
            let op = match reg.as_str() {
                sm83::REG_HL => INSTR_LD_TO_HL_FROM_LABEL.op_code,
                sm83::REG_DE => INSTR_LD_TO_DE_FROM_LABEL.op_code,
                _ => return Err(format!("ld: unknown source register: {}", reg)),
            };

            state.current_section_address.add_bytes(3);

            let may_address = state.label_addresses.get(&lbl);
            if let Some(lbl_address) = may_address {
                let from_address = lbl_address.0 as u16;
                let sec = expect_in_w_sec(state)?;
                sec.memory.push_u8(op);
                sec.memory.push_u16(from_address);
                Ok(None)
            } else {
                let sec = expect_in_w_sec(state)?;
                sec.memory.push_u8(op);
                sec.memory.push_u16(0);
                Ok(Some(UnresolvedLabel {
                    relative_from: None,
                    label: lbl.clone(),
                    check: check_16_bit_address_range,
                    sec_name: sec.name.clone(),
                    patch_index: sec.memory.mem_ptr - 2,
                    patch_width: 2,
                }))
            }
        }
        (SExp::Symbol(Symbol::Reg(reg)), SExp::Immediate(im_value)) => {
            let op = match reg.as_str() {
                sm83::REG_B => INSTR_LD_TO_B_FROM_IMMEDIATE.op_code,
                _ => return Err(format!("ld: unknown source register: {}", reg)),
            };

            state.current_section_address.add_bytes(2);

            // TODO check range of immediate value!
            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
            sec.memory.push_u8(*im_value as u8);
            Ok(None)
        }
        (SExp::Symbol(Symbol::Reg(dst_reg)), SExp::Form(deref)) => {
            let src_reg = expect_deref_reg(&deref)?;
            let op = match (dst_reg.as_str(), src_reg) {
                (sm83::REG_A, sm83::REG_HL) => INSTR_LD_TO_A_FROM_DEREF_HL.op_code,
                _ => {
                    return Err(format!(
                        "ld: unknown source register: {}, {:?}",
                        dst_reg, form
                    ));
                }
            };

            state.current_section_address.add_bytes(1);

            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
            Ok(None)
        }
        (SExp::Form(form), SExp::Immediate(im_value)) => {
            let op = match &form.op {
                Symbol::Reg(reg) => match reg.as_str() {
                    sm83::REG_HL => INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE.op_code,
                    _ => return Err(format!("ld: unknown deref register: {}", reg)),
                },
                illegal => return Err(format!("ld: illegal deref: {:?}", illegal)),
            };

            state.current_section_address.add_bytes(2);

            // TODO check range of immediate value!
            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
            sec.memory.push_u8(*im_value as u8);

            Ok(None)
        }
        (SExp::Form(form), SExp::Symbol(Symbol::Reg(src_reg))) => {
            let dst_reg = expect_deref_reg(&form)?;
            let op = match (dst_reg, src_reg.as_str()) {
                (sm83::REG_DE, sm83::REG_A) => INSTR_LD_TO_DEREF_DE_FROM_A.op_code,
                illegal => return Err(format!("ld: illegal deref from reg: {:?}", illegal)),
            };

            state.current_section_address.add_bytes(1);

            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
            Ok(None)
        }
        (a1, a2) => return Err(format!("ld: illegal parameters: {:?} {:?}", a1, a2)),
    }
}

fn expect_deref_reg(form: &Form) -> Result<&str, String> {
    let dst_reg = match &form.op {
        Symbol::Reg(reg) => reg,
        illegal => return Err(format!("illegal deref: {:?}", illegal)),
    };
    Ok(dst_reg.as_str())
}

fn jp(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    state.current_section_address.add_bytes(3);

    let lbl = expect_label_name(&form.exps[0])?;
    let may_address = state.label_addresses.get(&lbl);
    if let Some(lbl_address) = may_address {
        let to_address = lbl_address.0 as i32;
        check_16_bit_address_range(to_address)?;
        let sec = expect_in_w_sec(state)?;
        sec.memory.push_u8(sm83::INSTR_JP.op_code);
        sec.memory.push_u16(to_address as u16);

        Ok(None)
    } else {
        let sec = expect_in_w_sec(state)?;
        sec.memory.push_u8(sm83::INSTR_JP.op_code);
        sec.memory.push_u16(0);

        Ok(Some(UnresolvedLabel {
            relative_from: None,
            label: lbl.clone(),
            check: check_16_bit_address_range,
            sec_name: sec.name.clone(),
            patch_index: sec.memory.mem_ptr - 2,
            patch_width: 2,
        }))
    }
}

fn check_16_bit_address_range(dist: i32) -> Result<(), String> {
    if dist < u16::MIN as i32 {
        return Err(format!("jp: max {} jumps back, was {}", u16::MIN, dist));
    }
    if dist > u16::MAX as i32 {
        return Err(format!("jp: max {} jumps forward, was {}", u16::MAX, dist));
    }
    Ok(())
}

/// jump relative
fn jr(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.is_empty() {
        return Err("jr: needs at least one argument".to_string());
    }

    let (flag, lbl_ix) = if let Some(flg) = is_flag(&form.exps[0]) {
        (Some(flg), 1)
    } else {
        (None, 0)
    };

    if let Some(lbl) = is_label(&form.exps[lbl_ix]) {
        state.current_section_address.add_bytes(2);

        let may_address = state.label_addresses.get(&lbl);
        if let Some(lbl_address) = may_address {
            // resolve and check it immeditately
            let rel_dist = lbl_address.0 as i32 - state.current_section_address.0 as i32;
            check_jr_jump(rel_dist)?;
            let sec = expect_in_w_sec(state)?;
            write_jr_instr(sec, flag, rel_dist as u8)?;
            Ok(None)
        } else {
            let curr_address = state.current_section_address;
            let sec = expect_in_w_sec(state)?;
            write_jr_instr(sec, flag, 0)?;
            Ok(Some(UnresolvedLabel {
                relative_from: Some(curr_address),
                label: lbl.clone(),
                check: check_jr_jump,
                sec_name: sec.name.clone(),
                patch_index: sec.memory.mem_ptr - 1,
                patch_width: 1,
            }))
        }
    } else {
        Err(format!(
            "jr currently only supports labels, was: {:?}",
            &form.exps[0]
        ))
    }
}

fn write_jr_instr(sec: &mut Section, flag: Option<&String>, rel_dist: u8) -> Result<(), String> {
    match flag {
        None => sec.memory.push_u8(sm83::INSTR_JR.op_code),
        Some(flag) => match flag.as_str() {
            "nz" => sec.memory.push_u8(sm83::INSTR_JR_NZ.op_code),
            _ => return Err(format!("jr: unknown flag '{}'", flag)),
        },
    }

    sec.memory.push_u8(rel_dist);
    Ok(())
}

fn check_jr_jump(rel_dist: i32) -> Result<(), String> {
    if rel_dist < -128 {
        return Err(format!("jr: max -128 jumps back, was {}", rel_dist));
    }
    if rel_dist > 127 {
        return Err(format!("jr: max 127 jumps forward, was {}", rel_dist));
    }
    Ok(())
}

fn inc(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() != 1 {
        return Err("inc: needs exactly one argument".to_string());
    }

    match &form.exps[0] {
        SExp::Symbol(Symbol::Reg(reg)) => {
            let op = match reg.as_str() {
                sm83::REG_A => INSTR_INC_A.op_code,
                sm83::REG_DE => INSTR_INC_DE.op_code,
                sm83::REG_HL => INSTR_INC_HL.op_code,
                illegal_reg => return Err(format!("inc: unknow register: {}", illegal_reg)),
            };

            state.current_section_address.add_bytes(1);

            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
        }
        illegal => return Err(format!("inc: illegal argument: {:?}", illegal)),
    }
    Ok(None)
}

fn dec(state: &mut State, form: &Form) -> Result<Option<UnresolvedLabel>, String> {
    if form.exps.len() != 1 {
        return Err("dec: needs exactly one argument".to_string());
    }

    match &form.exps[0] {
        SExp::Symbol(Symbol::Reg(reg)) => {
            let op = match reg.as_str() {
                sm83::REG_A => INSTR_DEC_A.op_code,
                sm83::REG_B => INSTR_DEC_B.op_code,
                sm83::REG_DE => INSTR_DEC_DE.op_code,
                sm83::REG_HL => INSTR_DEC_HL.op_code,
                illegal_reg => return Err(format!("dec: unknow register: {}", illegal_reg)),
            };

            state.current_section_address.add_bytes(1);

            let sec = expect_in_w_sec(state)?;
            sec.memory.push_u8(op);
        }
        illegal => return Err(format!("dec: illegal argument: {:?}", illegal)),
    }
    Ok(None)
}

// interpret/assemble helper

static FALSE_SYM_NAME: &'static str = "false";
static TRUE_SYM_NAME: &'static str = "true";

fn sym_false() -> Symbol {
    Symbol::Sym(FALSE_SYM_NAME.to_string())
}

fn expect_label_address(state: &State, lbl: &Label) -> Result<u16, String> {
    if let Some(address) = state.label_addresses.get(&lbl) {
        Ok(address.0 as u16)
    } else {
        Err(format!("no address for label '{}", lbl.name()))
    }
}

fn expect_in_section(state: &State) -> Result<(), String> {
    if state.current_section_name.is_none() {
        Err("not in a section".to_string())
    } else {
        Ok(())
    }
}

fn expect_bool_sym(sym: &Symbol) -> Result<bool, String> {
    match sym {
        Symbol::Sym(name) => {
            if name == FALSE_SYM_NAME {
                Ok(false)
            } else if name == TRUE_SYM_NAME {
                Ok(true)
            } else {
                Err("symbol but not true|false".to_string())
            }
        }
        _ => Err("not a bool symbol".to_string()),
    }
}

fn expect_string(exp: &SExp) -> Result<String, String> {
    match exp {
        SExp::String(str) => Ok(str.clone()),
        _ => Err("string expected".to_string()),
    }
}

fn expect_section_name(exp: &SExp) -> Result<String, String> {
    match exp {
        SExp::Symbol(Symbol::Section(name)) => Ok(name.clone()),
        _ => Err("section name expected".to_string()),
    }
}

fn expect_label_name(exp: &SExp) -> Result<Label, String> {
    match exp {
        SExp::Symbol(Symbol::Label(lbl)) => Ok(lbl.clone()),
        _ => Err("label expected".to_string()),
    }
}

fn expect_immediate(exp: &SExp) -> Result<u64, String> {
    match exp {
        SExp::Immediate(val) => Ok(*val),
        _ => Err(format!("not an immediate value: {:?}", exp)),
    }
}

// If the Sexp is not a immediate value it will return an error.
// If the Option is None it will use the or value.
fn expect_immediate_value_or(exp: Option<&SExp>, or: u64) -> Result<u64, String> {
    if let Some(exp) = exp {
        expect_immediate(exp)
    } else {
        Ok(or)
    }
}

fn expect_symbol_or<'a>(exp: Option<&'a SExp>, or: &'a Symbol) -> Result<&'a Symbol, String> {
    if let Some(exp) = exp {
        match exp {
            SExp::Symbol(sym) => Ok(sym),
            _ => Err("not a symbol".to_string()),
        }
    } else {
        Ok(or)
    }
}

fn expect_has_sexp_at(exps: &[SExp], i: usize, err: &str) -> Result<(), String> {
    if exps.len() <= i {
        return Err(format!("expected a '{}' but got nothing", err));
    }
    Ok(())
}

fn expect_in_w_sec<'a>(state: &'a mut State) -> Result<&'a mut Section, String> {
    let curr_name = state.current_section_name.clone();
    if let Some(sec_name) = curr_name {
        let sec = state
            .lookup_section_mut(&sec_name)
            .expect("a current section");
        if sec.label_only {
            Err(format!("section {} is label-only", sec_name))
        } else {
            Ok(sec)
        }
    } else {
        Err("not in a section".to_string())
    }
}

fn is_keyword(exp: &SExp, name: &str) -> bool {
    match exp {
        SExp::Symbol(Symbol::Keyword(keyword_name)) => keyword_name == name,
        _ => false,
    }
}

fn is_label(exp: &SExp) -> Option<&Label> {
    match exp {
        SExp::Symbol(Symbol::Label(lbl)) => Some(lbl),
        _ => None,
    }
}

fn is_flag(exp: &SExp) -> Option<&String> {
    match exp {
        SExp::Symbol(Symbol::Flag(flg)) => Some(flg),
        _ => None,
    }
}

fn key_value<'a>(exps: &'a [SExp], name: &str) -> Result<Option<&'a SExp>, String> {
    let mut i = 0;
    while i < exps.len() {
        if is_keyword(&exps[i], name) {
            if i + 1 < exps.len() {
                return Ok(Some(&exps[i + 1]));
            } else {
                return Err("no value for keyword".to_string());
            }
        }
        i += 1;
    }
    Ok(None)
}
