use std::{fs::File, iter::Peekable, str::Chars};

use crate::asm::parser::{Label, Symbol, parse, parse_file};

#[test]
fn test_parse_test_file() -> Result<(), String> {
    let mut f = File::open("testdata/asm/test.asm").map_err(|e| e.to_string())?;
    let tl = parse_file(&mut f)?;

    assert_eq!(tl.forms.len(), 13);
    Ok(())
}

#[test]
fn test_parse_label_only_form() -> Result<(), String> {
    let tl = parse(&mut chars("('value1 db)"))?;
    assert_eq!(tl.forms.len(), 1);
    assert_eq!(tl.forms[0].label, Some(Label("value1".to_string())));
    assert_eq!(tl.forms[0].op, Symbol::Sym("db".to_string()));
    assert!(tl.forms[0].exps.is_empty());
    Ok(())
}

// helper

fn chars(str: &'static str) -> Peekable<Chars<'static>> {
    str.chars().peekable()
}
