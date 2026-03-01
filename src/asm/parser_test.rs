use std::{fs::File, iter::Peekable, str::Chars};

use crate::asm::parser::{Label, SExp, Symbol, parse, parse_from_file, parse_symbol};

#[test]
fn test_parse_test_file() -> Result<(), String> {
    let mut f = File::open("testdata/asm/test.asm").map_err(|e| e.to_string())?;
    let tl = parse_from_file(&mut f)?;

    assert_eq!(tl.forms.len(), 13);
    Ok(())
}

#[test]
fn test_parse_label_form() -> Result<(), String> {
    let tl = parse(&mut chars("('value1 db)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, Some(Label("value1".to_string())));
    assert_eq!(tl.forms[0].op, Symbol::Sym("db".to_string()));
    assert!(tl.forms[0].exps.is_empty());
    Ok(())
}

/// Special case form, same as the explicit (label 'lbl).
/// The only form allowed with an operator.
#[test]
fn test_parse_label_only_form() -> Result<(), String> {
    let tl = parse(&mut chars("('lbl)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, Some(Label("lbl".to_string())));
    assert_eq!(tl.forms[0].op, Symbol::Sym("".to_string()));
    assert!(tl.forms[0].exps.is_empty());
    Ok(())
}

#[test]
fn test_parse_immediate_values() -> Result<(), String> {
    // 'db' is a bit arbitrary, but the easiest for an immediate value test
    let cases = [
        ("(db 42)", 42),
        ("(db 0x42)", 66),
        ("(db 0b1010011010", 666),
    ];

    for (exp, val) in cases {
        let tl = parse(&mut chars(exp))?;
        assert_eq!(tl.forms.len(), 1);
        assert_eq!(tl.forms[0].op, Symbol::Sym("db".to_string()));
        assert_eq!(tl.forms[0].exps.len(), 1);
        assert_eq!(tl.forms[0].exps[0], SExp::Immediate(val));
    }

    Ok(())
}

#[test]
fn test_parse_include() -> Result<(), String> {
    let tl = parse(&mut chars("(include \"gb_dma\")"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Sym("include".to_string()));
    assert_eq!(tl.forms[0].exps.len(), 1);
    assert_eq!(tl.forms[0].exps[0], SExp::String("gb_dma".to_string()));
    Ok(())
}

#[test]
fn test_parse_def_constant() -> Result<(), String> {
    let tl = parse(&mut chars("(def-constant +const+ 1)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Sym("def-constant".to_string()));
    assert_eq!(tl.forms[0].exps.len(), 2);
    assert_eq!(
        tl.forms[0].exps[0],
        SExp::Symbol(Symbol::Sym("+const+".to_string()))
    );
    assert_eq!(tl.forms[0].exps[1], SExp::Immediate(1));
    Ok(())
}

#[test]
fn test_parse_shift_left() -> Result<(), String> {
    let tl = parse(&mut chars("(<< 1 +const+)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Sym("<<".to_string()));
    assert_eq!(tl.forms[0].exps.len(), 2);
    assert_eq!(tl.forms[0].exps[0], SExp::Immediate(1));
    assert_eq!(
        tl.forms[0].exps[1],
        SExp::Symbol(Symbol::Sym("+const+".to_string()))
    );

    Ok(())
}

#[test]
fn test_parse_deref_reg() -> Result<(), String> {
    let tl = parse(&mut chars("(%hl)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Reg("hl".to_string()));
    Ok(())
}

#[test]
fn test_parse_ld_deref_immediate() -> Result<(), String> {
    let tl = parse(&mut chars("(ld (%hl) 1)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Sym("ld".to_string()));
    Ok(())
}

#[test]
fn test_parse_jr_conditional_nz() -> Result<(), String> {
    let tl = parse(&mut chars("(jr #nz 'lbl)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, None);
    assert_eq!(tl.forms[0].op, Symbol::Sym("jr".to_string()));
    assert_eq!(tl.forms[0].exps.len(), 2);
    assert_eq!(
        tl.forms[0].exps[0],
        SExp::Symbol(Symbol::Flag("nz".to_string()))
    );
    assert_eq!(
        tl.forms[0].exps[1],
        SExp::Symbol(Symbol::Label(Label("lbl".to_string())))
    );
    Ok(())
}

#[test]
fn test_parse_symbol() -> Result<(), String> {
    let cases = [
        ("", Symbol::Sym("".to_string())), //empty symbol is a special case, but allowed in places
        (")", Symbol::Sym("".to_string())),
        ("a", Symbol::Sym("a".to_string())),
        ("a)", Symbol::Sym("a".to_string())), // test symbol boundaries
        ("+", Symbol::Sym("+".to_string())),
        ("+xxx+", Symbol::Sym("+xxx+".to_string())),
        ("<", Symbol::Sym("<".to_string())),
        (">", Symbol::Sym(">".to_string())),
        ("<<", Symbol::Sym("<<".to_string())),
        (">>", Symbol::Sym(">>".to_string())),
    ];

    for (exp, symbol) in cases {
        let parsed_symbol = parse_symbol(&mut chars(exp))?;
        assert_eq!(parsed_symbol, symbol, "exp: {}", exp);
    }

    Ok(())
}

// helper

fn chars(str: &'static str) -> Peekable<Chars<'static>> {
    str.chars().peekable()
}
