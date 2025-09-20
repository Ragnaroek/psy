use crate::pasm::parser::{Form, SExp, Symbol, TopLevel};

#[derive(Debug)]
struct Section {
    name: String,
    offset: u64,
    length: u64,
    label_only: bool,
}

struct State {
    sections: Vec<Section>,
}

impl State {
    fn new() -> State {
        State {
            sections: Vec::new(),
        }
    }
}

pub fn interpret(pasm: TopLevel) -> Result<(), String> {
    let mut state = State::new();

    for form in &pasm.forms {
        match &form.op {
            Symbol::Sym(sym_name) => {
                if sym_name == "include" {
                    include(&mut state);
                } else if sym_name == "def-section" {
                    def_section(&mut state, form)?;
                } else if sym_name == "section" {
                    section(&mut state);
                } else if sym_name == "db" {
                    db(&mut state);
                } else if sym_name == "label" {
                    label(&mut state);
                } else if sym_name == "ld" {
                    ld(&mut state);
                } else if sym_name == "sub-section" {
                    sub_section(&mut state);
                } else {
                    return Err(format!("unknown top-level: {:?}", sym_name));
                }
            }
            sym => return Err(format!("illegal top-level form: {:?}", sym)),
        }
    }

    //debug
    println!("sections = {:?}", state.sections);

    Ok(())
}

fn include(state: &mut State) {
    // TODO read file and interpret this one first!
    println!("!include todo");
}

fn def_section(state: &mut State, form: &Form) -> Result<(), String> {
    if form.exps.len() == 0 {
        return Err("illegal def-section".to_string());
    }

    let name = expect_label_name(&form.exps[0])?;
    let offset = expect_immediate_value_or(key_value(&form.exps, "offset")?, 0)?;
    let length = expect_immediate_value_or(key_value(&form.exps, "length")?, 0)?;
    let false_default = &sym_false();
    let label_only_sym = expect_symbol_or(key_value(&form.exps, "label-only")?, false_default)?;
    let label_only = expect_bool_sym(label_only_sym)?;

    state.sections.push(Section {
        name,
        offset,
        length,
        label_only,
    });
    Ok(())
}

fn section(state: &mut State) {
    // TODO update the current working section in the state!
    println!("!section");
}

fn db(state: &mut State) {
    // TODO link the label with this address in the state (and check that the label is not used already)
    println!("!db");
}

fn ld(state: &mut State) {
    println!("!ld");
}

fn label(state: &mut State) {
    println!("!label");
}

fn sub_section(state: &mut State) {
    println!("!sub-section");
}

// interpret helper

static FALSE_SYM_NAME: &'static str = "false";
static TRUE_SYM_NAME: &'static str = "true";

fn sym_false() -> Symbol {
    Symbol::Sym(FALSE_SYM_NAME.to_string())
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

fn expect_label_name(exp: &SExp) -> Result<String, String> {
    match exp {
        SExp::Symbol(Symbol::Label(name)) => Ok(name.clone()),
        _ => Err("label expected".to_string()),
    }
}

// If the Sexp is not a immediate value it will return an error.
// If the Option is None it will use the or value.
fn expect_immediate_value_or(exp: Option<&SExp>, or: u64) -> Result<u64, String> {
    if let Some(exp) = exp {
        match exp {
            SExp::Immediate(val) => Ok(*val),
            _ => Err("not an immediate value".to_string()),
        }
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
