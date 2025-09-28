#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use std::{fs::File, io::Read, iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum SExp {
    Symbol(Symbol),
    Form(Form),
    String(String),
    Immediate(String), // TODO convert to u64 value for easier handling
}

#[derive(Debug)]
pub enum Symbol {
    Keyword(String), // : prefix
    Section(String), // . prefix
    Label(String),   // ' prefix
    Sym(String),     //a-zA-Z only
}

#[derive(Debug)]
pub struct Form {
    pub op: Symbol,
    pub exps: Vec<SExp>,
}

/*
S => SExp*
SExp => (SExp) | Symbol
Symbol => Ascii-char*
*/
pub fn parse_file(file: &mut File) -> Result<Form, String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e| e.to_string())?;
    parse(&mut buf.chars().peekable())
}

fn parse(chars: &mut Peekable<Chars>) -> Result<Form, String> {
    let mut forms = Vec::new();
    loop {
        if skip_whitespace_and_comment(chars)? {
            break;
        }
        forms.push(SExp::Form(parse_form(chars)?));
    }

    Ok(Form {
        op: Symbol::Sym("progn".to_string()),
        exps: forms,
    })
}

fn parse_form(chars: &mut Peekable<Chars>) -> Result<Form, String> {
    expect(chars, '(')?;

    if skip_whitespace_and_comment(chars)? {
        return Err("unexpected form end".to_string());
    }

    let op = parse_symbol(chars)?;

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

    if exps.is_empty() {
        return Err("empty form".to_string());
    }

    Ok(Form { op, exps })
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Result<Symbol, String> {
    let mut sym = String::new();

    let may_first_char = chars.next();
    let first_char = match may_first_char {
        None => return Err("unexpected end of symbol".to_string()),
        Some(ch) => ch,
    };
    sym.push(first_char);

    loop {
        let la = {
            let p = chars.peek();
            match p {
                None => break,
                Some(ch) => *ch,
            }
        };

        if la.is_alphabetic() || la == '-' {
            chars.advance_by(1).map_err(|e| e.to_string())?;
            sym.push(la);
        } else {
            break;
        }
    }

    if sym.is_empty() {
        return Err("error: empty symbol".to_string());
    }

    Ok(Symbol::Sym(sym))
}

fn parse_immediate(chars: &mut Peekable<Chars>) -> Result<String, String> {
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

    let may_second_num = chars.next();
    let mut is_hex = false;
    match may_second_num {
        None => return Ok(immediate),
        Some(ch) => match ch {
            'x' => {
                is_hex = true;
                immediate.push('x')
            }
            other => immediate.push(other),
        },
    }

    loop {
        let la = {
            let p = chars.peek();
            match p {
                None => break,
                Some(ch) => *ch,
            }
        };

        if la.is_numeric() || (is_hex && la.is_ascii_hexdigit()) {
            chars.advance_by(1).map_err(|e| e.to_string())?;
            immediate.push(la);
        } else {
            break;
        }
    }

    Ok(immediate)
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
