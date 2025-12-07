#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use core::iter::Iterator;
use std::{fs::File, io::Read, iter::Peekable, str::Chars};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Label(String);

impl Label {
    pub fn from_string(str: String) -> Label {
        Label(str)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Address(pub u64);

impl Address {
    pub fn add_bytes(&mut self, n: u64) {
        self.0 += n * 8;
    }
}

#[derive(Debug, PartialEq)]
pub enum SExp {
    TopLevel(TopLevel),
    Symbol(Symbol),
    Form(Form),
    String(String),
    Immediate(u64),
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Keyword(String), // : prefix
    Section(String), // . prefix
    Reg(String),     // % prefix
    Label(Label),    // ' prefix
    Flag(String),    // # prefix
    Sym(String),     //a-zA-Z only
}

#[derive(Debug, PartialEq)]
pub struct Form {
    pub label: Option<Label>,
    pub op: Symbol,
    pub exps: Vec<SExp>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel {
    pub forms: Vec<Form>,
}

/*
S => SExp*
SExp => (SExp) | Symbol
Symbol => Ascii-char*
*/
pub fn parse_from_file(file: &mut File) -> Result<TopLevel, String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e| e.to_string())?;
    parse(&mut buf.chars().peekable())
}

pub fn parse_from_string(str: &str) -> Result<TopLevel, String> {
    parse(&mut str.chars().peekable())
}

fn parse(chars: &mut Peekable<Chars>) -> Result<TopLevel, String> {
    let mut forms = Vec::new();
    loop {
        if skip_whitespace_and_comment(chars)? {
            break;
        }

        let may_form = parse_form(chars);
        if may_form.is_ok() {
            forms.push(may_form.unwrap());
        } else {
            return Err(format!(
                "error parsing form #{}: {:?}",
                forms.len() + 1,
                may_form.err()
            ));
        }
    }
    Ok(TopLevel { forms })
}

fn parse_form(chars: &mut Peekable<Chars>) -> Result<Form, String> {
    expect(chars, '(')?;

    if skip_whitespace_and_comment(chars)? {
        return Err("unexpected form end".to_string());
    }

    let mut label = None;
    let mut op = parse_symbol(chars)?;
    if is_label(&op) {
        label = Some(sym_get_label(op)?);
        skip_whitespace_and_comment(chars)?;
        op = parse_symbol(chars)?
    }

    let mut exps = Vec::new();
    'parse: loop {
        if skip_whitespace_and_comment(chars)? {
            break;
        }
        let la = chars.peek();
        match la {
            None => return Err("unexpected form end".to_string()),
            Some('(') => exps.push(SExp::Form(parse_form(chars)?)),
            Some('"') => exps.push(SExp::String(parse_string(chars)?)),
            Some(')') => {
                chars.advance_by(1).map_err(|e| e.to_string())?;
                break 'parse;
            }
            Some(';') => skip_line_comment(chars)?,
            Some(ch) => {
                if ch.is_numeric() {
                    exps.push(SExp::Immediate(parse_immediate(chars)?))
                } else {
                    exps.push(SExp::Symbol(parse_symbol(chars)?))
                }
            }
        };
    }

    Ok(Form { label, op, exps })
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Result<Symbol, String> {
    let mut sym = String::new();

    let may_first_char = chars.next();
    let first_char = match may_first_char {
        None => return Err("unexpected end of symbol".to_string()),
        Some(ch) => ch,
    };
    if first_char != ':'
        && first_char != '.'
        && first_char != '\''
        && first_char != '%'
        && first_char != '#'
    {
        sym.push(first_char);
    }

    loop {
        let la = {
            let p = chars.peek();
            match p {
                None => break,
                Some(ch) => *ch,
            }
        };

        if la.is_alphanumeric() || la == '-' {
            chars.advance_by(1).map_err(|e| e.to_string())?;
            sym.push(la);
        } else {
            break;
        }
    }

    if sym.is_empty() {
        return Err("error: empty symbol".to_string());
    }

    match first_char {
        ':' => Ok(Symbol::Keyword(sym)),
        '.' => Ok(Symbol::Section(sym)),
        '\'' => Ok(Symbol::Label(Label(sym))),
        '%' => Ok(Symbol::Reg(sym)),
        '#' => Ok(Symbol::Flag(sym)),
        _ => Ok(Symbol::Sym(sym)),
    }
}

#[derive(PartialEq)]
enum ImmediateType {
    Decimal,
    Hex,
    Binary,
}

fn parse_immediate(chars: &mut Peekable<Chars>) -> Result<u64, String> {
    let mut immediate = String::new();

    let may_first_num = chars.next();
    match may_first_num {
        None => return Err("unexpected end of immediate value".to_string()),
        Some(ch) => {
            if !ch.is_numeric() {
                return Err("illegal immediate value".to_string());
            }
            immediate.push(ch);
        }
    }

    let may_second_num = chars.peek();
    let mut immediate_type = ImmediateType::Decimal;
    match may_second_num {
        None => return Ok(parse_number_value(&immediate, ImmediateType::Decimal)?),
        Some(ch) => {
            match *ch {
                'x' => {
                    immediate_type = ImmediateType::Hex;
                    chars.next(); //consume lookahead char
                    immediate.clear();
                }
                'b' => {
                    immediate_type = ImmediateType::Binary;
                    chars.next(); //consume lookahead char
                    immediate.clear();
                }
                _ => { /* stay with Decimal */ }
            }
        }
    }

    loop {
        let la = {
            let p = chars.peek();
            match p {
                None => break,
                Some(ch) => *ch,
            }
        };

        if la.is_numeric() || (immediate_type == ImmediateType::Hex && la.is_ascii_hexdigit()) {
            chars.advance_by(1).map_err(|e| e.to_string())?;
            immediate.push(la);
        } else {
            break;
        }
    }

    parse_number_value(&immediate, immediate_type)
}

fn parse_number_value(str: &str, immediate_type: ImmediateType) -> Result<u64, String> {
    match immediate_type {
        ImmediateType::Decimal => {
            let u64_val: u64 = str.parse::<u64>().map_err(|e| e.to_string())?;
            Ok(u64_val)
        }
        ImmediateType::Hex => {
            let hex_u64 = u64::from_str_radix(str, 16);
            if let Ok(val_u64) = hex_u64 {
                Ok(val_u64)
            } else {
                Err(format!("invalid hex immediate: {}", str))
            }
        }
        ImmediateType::Binary => {
            let binary_u64 = u64::from_str_radix(str, 2);
            if let Ok(val_u64) = binary_u64 {
                Ok(val_u64)
            } else {
                Err(format!("invalid binary immediate: {}", str))
            }
        }
    }
}

fn parse_string(chars: &mut Peekable<Chars>) -> Result<String, String> {
    expect(chars, '"')?;
    let mut literal = String::new();
    loop {
        let next = chars.next();
        match next {
            None => return Err("unexpected end of string".to_string()),
            Some('"') => break,
            Some(ch) => literal.push(ch),
        }
    }

    Ok(literal)
}

// helper

fn expect(chars: &mut Peekable<Chars>, ch: char) -> Result<(), String> {
    if let Some(next_char) = chars.next() {
        if next_char == ch {
            return Ok(());
        }
        return Err(format!("expected {}, but got {}", ch, next_char));
    }
    Err("unexpected end".to_string())
}

/// Returns true if EOF is reached
fn skip_whitespace_and_comment(chars: &mut Peekable<Chars>) -> Result<bool, String> {
    loop {
        let la = chars.peek();
        match la {
            Some(ch) => {
                if ch.is_whitespace() {
                    chars.advance_by(1).map_err(|e| e.to_string())?;
                } else if *ch == ';' {
                    skip_line_comment(chars)?;
                } else {
                    return Ok(false);
                }
            }
            None => return Ok(true),
        }
    }
}

fn skip_line_comment(chars: &mut Peekable<Chars>) -> Result<(), String> {
    expect(chars, ';')?;
    loop {
        let next = chars.next();
        match next {
            None | Some('\n') => break,
            _ => { /* skip next char */ }
        };
    }
    Ok(())
}

fn is_label(sym: &Symbol) -> bool {
    match sym {
        Symbol::Label(_) => true,
        _ => false,
    }
}

fn sym_get_label(sym: Symbol) -> Result<Label, String> {
    match sym {
        Symbol::Label(lbl) => Ok(lbl),
        _ => Err("symbol not a label".to_string()),
    }
}
