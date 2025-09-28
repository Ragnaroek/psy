use crate::pasm::parser::{Symbol, TopLevel};

struct Section {
    offset: usize,
    length: usize,
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
        println!("### form = {:?}", form);
        match &form.op {
            Symbol::Sym(sym_name) => {
                if sym_name == "include" {
                    include(&mut state);
                } else if sym_name == "def-section" {
                    def_section(&mut state);
                } else if sym_name == "section" {
                    section(&mut state);
                } else if sym_name == "db" {
                    db(&mut state);
                } else {
                    return Err(format!("unknown top-level: {:?}", sym_name));
                }
            }
            sym => return Err(format!("illegal top-level form: {:?}", sym)),
        }
    }
    Ok(())
}

fn include(state: &mut State) {
    // TODO read file and interpret this one first!
    println!("!include todo");
}

fn def_section(state: &mut State) {
    // TODO add section definition to state!
    println!("!def_section todo");
}

fn section(state: &mut State) {
    // TODO update the current working section in the state!
    println!("!section");
}

fn db(state: &mut State) {
    // TODO link the label with this address in the state (and check that the label is not used already)
    println!("!db");
}
