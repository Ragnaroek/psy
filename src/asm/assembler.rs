use crate::asm::parser::{Address, Form, Label, SExp, Symbol, TopLevel, parse_file};
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
    for form in &pasm.forms {
        match &form.op {
            Symbol::Sym(sym_name) => {
                if sym_name == "include" {
                    include(state, form)?;
                } else if sym_name == "def-section" {
                    def_section(state, form)?;
                } else if sym_name == "section" {
                    section(state, form)?;
                } else if sym_name == "db" {
                    db(state, form)?;
                } else if sym_name == "label" {
                    label(state);
                } else if sym_name == "sub-section" {
                    sub_section(state);
                // the following forms are tempoarily handled here. Plan is
                // to convert this to macros that emits bytes with low-level primitives
                } else if sym_name == "ld" {
                    //machine specific, should not be handled here
                    ld(state);
                } else if sym_name == "jp" {
                    jp(state, form)?;
                } else if sym_name == "inc" {
                    inc(state);
                } else if sym_name == "dec" {
                    dec(state);
                } else if sym_name == "jr" {
                    jr(state);
                } else if sym_name == "nop" {
                    nop(state)?;
                } else {
                    return Err(format!("unknown top-level: {:?}", sym_name));
                }
            }
            sym => return Err(format!("illegal top-level form: {:?}", sym)),
        }
    }
    Ok(())
}

fn include(state: &mut State, form: &Form) -> Result<(), String> {
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
    let tl = parse_file(&mut file)?;
    assemble_in_state(tl, state)?;
    Ok(())
}

fn def_section(state: &mut State, form: &Form) -> Result<(), String> {
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
    Ok(())
}

fn section(state: &mut State, form: &Form) -> Result<(), String> {
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
        Ok(())
    } else {
        Err(format!("no such section: {}", name))
    }
}

fn db(state: &mut State, db: &Form) -> Result<(), String> {
    expect_in_section(state)?;

    if let Some(label) = &db.label {
        let lbl = label.clone(); // TODO is the clone free possible?
        if state.label_addresses.contains_key(&lbl) {
            return Err(format!("duplicate label definition: '{}", lbl.name()));
        }
        state
            .label_addresses
            .insert(lbl, state.current_section_address);
    }

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

    Ok(())
}

fn label(state: &mut State) {
    println!("!label");
}

fn sub_section(state: &mut State) {
    println!("!sub-section");
}

// non-primitive forms, temporarily implemented in Rust directly

fn nop(state: &mut State) -> Result<(), String> {
    let sec = expect_in_w_sec(state)?;
    sec.memory.push_u8(0);
    Ok(())
}

fn ld(state: &mut State) {
    println!("!ld");
}

fn jp(state: &mut State, form: &Form) -> Result<(), String> {
    let lbl = expect_label_name(&form.exps[0])?;
    let lbl_address = if let Some(address) = state.label_addresses.get(&lbl) {
        address.0 as u16
    } else {
        return Err(format!("no address for label '{}", lbl.name()));
    };

    let sec = expect_in_w_sec(state)?;
    sec.memory.push_u8(0xC3);
    sec.memory.push_u16(lbl_address);
    Ok(())
}

fn jr(state: &mut State) {
    println!("!jr");
}

fn inc(state: &mut State) {
    println!("!inc");
}

fn dec(state: &mut State) {
    println!("!dec");
}

// interpret helper

static FALSE_SYM_NAME: &'static str = "false";
static TRUE_SYM_NAME: &'static str = "true";

fn sym_false() -> Symbol {
    Symbol::Sym(FALSE_SYM_NAME.to_string())
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
