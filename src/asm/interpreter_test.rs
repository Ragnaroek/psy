use crate::asm::interpreter::eval_aar;
use crate::asm::parser::{Address, Label, SExp, Symbol, parse_from_string};

#[test]
fn test_eval_aar_fails() -> Result<(), String> {
    let cases = [
        (
            "(-)",
            &[].iter().cloned().collect(),
            "-: invalid number or arguments 0",
        ),
        (
            "(- 'a)",
            &[].iter().cloned().collect(),
            "-: invalid number or arguments 1",
        ),
        (
            "(- 'a 'b)",
            &[].iter().cloned().collect(),
            "undefined label: Label(\"a\")",
        ),
        (
            "(- 'a 'b)", // negative address not allowed
            &[
                (Label::from_str("a"), Address(0x00)),
                (Label::from_str("b"), Address(0x01)),
            ]
            .iter()
            .cloned()
            .collect(),
            "-: negative address",
        ),
        (
            "(x 'a)",
            &[].iter().cloned().collect(),
            "illegal arithmetic address operator: \"x\"",
        ),
    ];
    for (exp, label_addresses, err) in cases {
        let sexp = must_parse_form(exp);
        let r = eval_aar(&sexp, label_addresses);

        assert!(
            r.is_err(),
            "expected error '{}' on expression = {:?}",
            err,
            exp
        );
        assert_eq!(r.unwrap_err(), err, "exp={:?}", exp);
    }

    Ok(())
}

#[test]
fn test_eval_aar_ok() -> Result<(), String> {
    let cases = [
        (
            "label only expression",
            SExp::Symbol(Symbol::Label(Label::from_str("test-label"))),
            &[(Label::from_str("test-label"), Address(0x666))]
                .iter()
                .cloned()
                .collect(),
            Address(0x666),
        ),
        (
            "subtract - 2 labels",
            must_parse_form("(- 'lbl2 'lbl1)"),
            &[
                (Label::from_str("lbl1"), Address(0x4000)),
                (Label::from_str("lbl2"), Address(0x5000)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x1000),
        ),
        (
            "subtract - 3 labels",
            must_parse_form("(- 'lbl3 'lbl2 'lbl1)"),
            &[
                (Label::from_str("lbl1"), Address(0x3000)),
                (Label::from_str("lbl2"), Address(0x5000)),
                (Label::from_str("lbl3"), Address(0x10000)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x8000),
        ),
        (
            "subtract - expression tree",
            must_parse_form("(- (- 'lbl3 'lbl2) (- 'lbl1 (- 'lbl0 'lblX)))"),
            &[
                (Label::from_str("lbl1"), Address(0x3000)),
                (Label::from_str("lbl2"), Address(0x5000)),
                (Label::from_str("lbl3"), Address(0x10000)),
                (Label::from_str("lbl0"), Address(0x2000)),
                (Label::from_str("lblX"), Address(0x500)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x9B00),
        ),
        (
            "add - 0 labels",
            must_parse_form("(+)"),
            &[].iter().cloned().collect(),
            Address(0x00),
        ),
        (
            "add - 1 labels",
            must_parse_form("(+ 'lbl1)"),
            &[(Label::from_str("lbl1"), Address(0x4000))]
                .iter()
                .cloned()
                .collect(),
            Address(0x4000),
        ),
        (
            "add - 2 labels",
            must_parse_form("(+ 'lbl2 'lbl1)"),
            &[
                (Label::from_str("lbl1"), Address(0x4000)),
                (Label::from_str("lbl2"), Address(0x5000)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x9000),
        ),
        (
            "add - 3 labels",
            must_parse_form("(+ 'lbl3 'lbl2 'lbl1)"),
            &[
                (Label::from_str("lbl1"), Address(0x4000)),
                (Label::from_str("lbl2"), Address(0x5000)),
                (Label::from_str("lbl3"), Address(0x100)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x9100),
        ),
        (
            "expression - mixed",
            must_parse_form("(+ 'lbl3 (- 'lbl2 'lbl1))"),
            &[
                (Label::from_str("lbl1"), Address(0x4000)),
                (Label::from_str("lbl2"), Address(0x5000)),
                (Label::from_str("lbl3"), Address(0x100)),
            ]
            .iter()
            .cloned()
            .collect(),
            Address(0x1100),
        ),
    ];
    for (test, sexp, label_addresses, want_result_address) in cases {
        let got_result_address = eval_aar(&sexp, label_addresses)?;
        assert_eq!(got_result_address, want_result_address, "test: {}", test);
    }
    Ok(())
}

fn must_parse_form(str: &str) -> SExp {
    SExp::Form(
        parse_from_string(str)
            .expect("parse form")
            .forms
            .pop()
            .expect("test form"),
    )
}
