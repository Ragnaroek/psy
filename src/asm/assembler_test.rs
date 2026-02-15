use crate::arch::sm83::{
    self, INSTR_CP_IMMEDIATE, INSTR_DEC_A, INSTR_DEC_B, INSTR_DEC_BC, INSTR_DEC_DE, INSTR_DEC_HL,
    INSTR_INC_A, INSTR_INC_BC, INSTR_INC_DE, INSTR_INC_HL, INSTR_LD_TO_A_FROM_DEREF_DE,
    INSTR_LD_TO_A_FROM_DEREF_HL, INSTR_LD_TO_A_FROM_DEREF_HL_INC, INSTR_LD_TO_A_FROM_DEREF_LABEL,
    INSTR_LD_TO_A_FROM_IMMEDIATE, INSTR_LD_TO_B_FROM_IMMEDIATE, INSTR_LD_TO_BC_FROM_LABEL,
    INSTR_LD_TO_DE_FROM_LABEL, INSTR_LD_TO_DEREF_DE_FROM_A, INSTR_LD_TO_DEREF_HL_FROM_A,
    INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE, INSTR_LD_TO_DEREF_HL_INC_FROM_A,
    INSTR_LD_TO_DEREF_LABEL_FROM_A, INSTR_LD_TO_HL_FROM_IMMEDIATE, INSTR_LD_TO_HL_FROM_LABEL,
};
use crate::asm::assembler::{
    Form, Label, LabelRef, Memory, Ref, Section, State, assemble_in_state, check_jr_jump, cp, dec,
    ds, expect_label_name, inc, jp, jr, ld, resolve_labels,
};

use crate::asm::parser::{Address, SExp, Symbol, parse_from_string};

#[test]
fn test_ds_ok() -> Result<(), String> {
    let cases = [("(ds 0)", 0), ("(ds 1", 1), ("(ds 66)", 66)];

    for (exp, mem_ptr) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        ds(&mut state, tl.forms.pop().unwrap())?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(sec.memory.mem_ptr, mem_ptr);
    }

    Ok(())
}

#[test]
fn test_jr_fails() -> Result<(), String> {
    let cases = [
        ("(jr)", Address(0), "jr: needs at least one argument"),
        (
            // unknown flag
            "(jr #tz 'lbl)",
            Address(0x4000 + 2),
            "jr: unknown flag 'tz'",
        ),
    ];

    for (exp, lbl_address, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        if !tl.forms[0].exps.is_empty() {
            let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
            state.label_addresses.insert(lbl, lbl_address);
        }

        let r = jr(&mut state, tl.forms.pop().unwrap());

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
fn test_jr_ok() -> Result<(), String> {
    let cases = [
        // jump to self
        (
            "(jr 'lbl)",
            Some(LabelRef {
                reference: Ref::Relative(
                    Address(16386),
                    Label::from_string("lbl".to_string()),
                    check_jr_jump,
                ),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JR.op_code,
        ),
        // jump nz
        (
            "(jr #nz 'lbl2)",
            Some(LabelRef {
                reference: Ref::Relative(
                    Address(16386),
                    Label::from_string("lbl2".to_string()),
                    check_jr_jump,
                ),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JR_IF_NZ.op_code,
        ),
        // jump c
        (
            "(jr #c 'lbl3)",
            Some(LabelRef {
                reference: Ref::Relative(
                    Address(16386),
                    Label::from_string("lbl3".to_string()),
                    check_jr_jump,
                ),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JR_IF_C.op_code,
        ),
        // jump flag, address not yet defined
        (
            "(jr #c 'lbl4)",
            Some(LabelRef {
                reference: Ref::Relative(
                    Address(16386),
                    Label::from_string("lbl4".to_string()),
                    check_jr_jump,
                ),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JR_IF_C.op_code,
        ),
    ];

    for (exp, expect_label_ref, op_code) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let got_label_ref = jr(&mut state, tl.forms.pop().unwrap())?;

        assert_eq_label_ref(got_label_ref, expect_label_ref);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], op_code,
            "op_code was {:x}, expected {:?}, expression={:?}",
            sec.memory.mem[0], op_code, exp,
        );
        assert_eq!(
            sec.memory.mem[1], 0,
            "inst2 was {:x}, expected 0, expression={:?}",
            sec.memory.mem[1], exp
        );
    }

    Ok(())
}

#[test]
fn test_jp_fails() -> Result<(), String> {
    let cases = [
        ("(jp)", "jp: needs at least one argument"),
        ("(jp #c 'foo 'bar)", "jp: illegal arguments"),
    ];

    for (exp, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let r = jp(&mut state, tl.forms.pop().unwrap());

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
fn test_jp_ok() -> Result<(), String> {
    let cases = [
        (
            "(jp 'forward)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("forward".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JP.op_code,
        ),
        // jump if #c
        (
            "(jp #c 'wait)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("wait".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            sm83::INSTR_JP_IF_C.op_code,
        ),
    ];

    for (exp, expect_label_ref, op_code) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let got_label_ref = jp(&mut state, tl.forms.pop().unwrap())?;

        assert_eq_label_ref(got_label_ref, expect_label_ref);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], op_code,
            "op_code was {:x}, expected {:?}",
            sec.memory.mem[0], op_code
        );
        assert_eq!(
            sec.memory.mem[1], 0,
            "inst2 was {:x} (expected 0)",
            sec.memory.mem[1]
        );
        assert_eq!(
            sec.memory.mem[2], 0,
            "inst3 was {:x} (expected 0)",
            sec.memory.mem[2]
        );
    }
    Ok(())
}

#[test]
fn test_ld_fails() -> Result<(), String> {
    let cases = [("(ld)", Address(0), "ld: needs at least two arguments")];

    for (exp, lbl_address, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        if !tl.forms[0].exps.is_empty() {
            let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
            state.label_addresses.insert(lbl, lbl_address);
        }

        let r = ld(&mut state, tl.forms.pop().unwrap());

        assert_eq!(r.unwrap_err(), err);
    }
    Ok(())
}

#[test]
fn test_ld_ok() -> Result<(), String> {
    let cases = [
        // load hl mem to reg forward
        (
            "(ld %hl 'forward)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("forward".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_HL_FROM_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %bc 'lbl)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("lbl".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_BC_FROM_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %bc (- 'lbl2 'lbl1))",
            Some(LabelRef {
                reference: Ref::from_form(Form {
                    op: Symbol::Sym("-".to_string()),
                    label: None,
                    exps: vec![
                        SExp::Symbol(Symbol::Label(Label::from_str("lbl2"))),
                        SExp::Symbol(Symbol::Label(Label::from_str("lbl1"))),
                    ],
                }),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_BC_FROM_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %de 'lbl)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("lbl".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_DE_FROM_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %hl 'lbl)",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("lbl".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_HL_FROM_LABEL.op_code,
            0x00,
            0x00,
        ),
        // load immediate to reg
        (
            "(ld %a 42)",
            None,
            2,
            INSTR_LD_TO_A_FROM_IMMEDIATE.op_code,
            0x2A,
            0x00,
        ),
        (
            "(ld %b 42)",
            None,
            2,
            INSTR_LD_TO_B_FROM_IMMEDIATE.op_code,
            0x2A,
            0x00,
        ),
        (
            "(ld %b 42)",
            None,
            2,
            INSTR_LD_TO_B_FROM_IMMEDIATE.op_code,
            0x2A,
            0x00,
        ),
        (
            "(ld %hl 32)",
            None,
            2,
            INSTR_LD_TO_HL_FROM_IMMEDIATE.op_code,
            0x20,
            0x00,
        ),
        // load deref immediate
        (
            "(ld (%hl) 42)",
            None,
            2,
            INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE.op_code,
            0x2A,
            0x00,
        ),
        (
            "(ld (%de) %a)",
            None,
            1,
            INSTR_LD_TO_DEREF_DE_FROM_A.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld (%hl) %a)",
            None,
            1,
            INSTR_LD_TO_DEREF_HL_FROM_A.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld (%hl +) %a)",
            None,
            1,
            INSTR_LD_TO_DEREF_HL_INC_FROM_A.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %a ('lblX))",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("lblX".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_A_FROM_DEREF_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %a (%hl))",
            None,
            1,
            INSTR_LD_TO_A_FROM_DEREF_HL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %a (%hl +))",
            None,
            1,
            INSTR_LD_TO_A_FROM_DEREF_HL_INC.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %a (%de))",
            None,
            1,
            INSTR_LD_TO_A_FROM_DEREF_DE.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld ('lbl) %a",
            Some(LabelRef {
                reference: Ref::from_label(Label::from_string("lbl".to_string())),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            }),
            3,
            INSTR_LD_TO_DEREF_LABEL_FROM_A.op_code,
            0x00,
            0x00,
        ),
    ];

    for (exp, expect_label_ref, byte_size, op_code, inst1, inst2) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let got_label_ref = ld(&mut state, tl.forms.pop().unwrap())?;

        assert_eq_label_ref(got_label_ref, expect_label_ref);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], op_code,
            "ld expression={:?}, op_code was {:x}, expected {:?}",
            exp, op_code, sec.memory.mem[0]
        );
        assert_eq!(
            sec.memory.mem[1], inst1,
            "ld expression={:?}, inst1 was {:x} (expected {:?})",
            exp, sec.memory.mem[1], inst1
        );
        assert_eq!(
            sec.memory.mem[2], inst2,
            "ld expression={:?}, inst2 was {:x} (expected {:?})",
            exp, sec.memory.mem[2], inst2
        );

        assert_eq!(state.current_section_address.0, TEST_SEC_ADDR.0 + byte_size);
    }
    Ok(())
}

#[test]
fn test_inc_fails() -> Result<(), String> {
    let cases = [
        ("(inc)", "inc: needs exactly one argument"),
        ("(inc 42)", "inc: illegal argument: Immediate(42)"),
    ];

    for (exp, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let r = inc(&mut state, tl.forms.pop().unwrap());

        assert_eq!(r.unwrap_err(), err);
    }
    Ok(())
}

#[test]
fn test_inc_ok() -> Result<(), String> {
    let cases = [
        ("(inc %a)", 1, INSTR_INC_A.op_code),
        ("(inc %bc)", 1, INSTR_INC_BC.op_code),
        ("(inc %de)", 1, INSTR_INC_DE.op_code),
        ("(inc %hl)", 1, INSTR_INC_HL.op_code),
    ];
    for (exp, byte_size, op) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;
        inc(&mut state, tl.forms.pop().unwrap())?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(sec.memory.mem[0], op, "inc expression={:?}", exp);

        assert_eq!(state.current_section_address.0, TEST_SEC_ADDR.0 + byte_size);
    }
    Ok(())
}

#[test]
fn test_dec_fails() -> Result<(), String> {
    let cases = [
        ("(dec)", "dec: needs exactly one argument"),
        ("(dec 42)", "dec: illegal argument: Immediate(42)"),
    ];

    for (exp, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;

        let r = dec(&mut state, tl.forms.pop().unwrap());

        assert_eq!(r.unwrap_err(), err);
    }
    Ok(())
}

#[test]
fn test_dec_ok() -> Result<(), String> {
    let cases = [
        ("(dec %a)", 1, INSTR_DEC_A.op_code),
        ("(dec %b)", 1, INSTR_DEC_B.op_code),
        ("(dec %bc)", 1, INSTR_DEC_BC.op_code),
        ("(dec %de)", 1, INSTR_DEC_DE.op_code),
        ("(dec %hl)", 1, INSTR_DEC_HL.op_code),
    ];
    for (exp, byte_size, op) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;
        dec(&mut state, tl.forms.pop().unwrap())?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(sec.memory.mem[0], op, "dec expression={:?}", exp);

        assert_eq!(state.current_section_address.0, TEST_SEC_ADDR.0 + byte_size);
    }
    Ok(())
}

#[test]
fn test_cp_fails() -> Result<(), String> {
    let cases = [("(cp)", "cp: needs exactly one argument")];

    for (exp, err) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;
        let r = cp(&mut state, tl.forms.pop().unwrap());

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
fn test_cp_ok() -> Result<(), String> {
    let cases = [("(cp 144)", 2, INSTR_CP_IMMEDIATE.op_code, 144)];
    for (exp, byte_size, op, arg1) in cases {
        let mut state = test_state();
        let mut tl = parse_from_string(exp)?;
        cp(&mut state, tl.forms.pop().unwrap())?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(sec.memory.mem[0], op, "cp expression={:?}", exp);
        assert_eq!(
            sec.memory.mem[1], arg1,
            "cp expression={:?}, arg1 was {:x}",
            exp, sec.memory.mem[1]
        );

        assert_eq!(state.current_section_address.0, TEST_SEC_ADDR.0 + byte_size);
    }
    Ok(())
}

#[test]
fn test_label() -> Result<(), String> {
    // start at 0 so the label address computation gets easier
    let mut state = test_state_with_section_offset(Address(0x00));
    let lt = parse_from_string("(ld %a 0) (cp 144) (label 'test) (ld %b 0)")?;
    assemble_in_state(lt, &mut state)?;

    let lbl_address = state
        .label_addresses
        .get(&Label::from_string("test".to_string()));

    assert_eq!(lbl_address, Some(&Address(4)));
    Ok(())
}

#[test]
fn test_resolve_label_fails() -> Result<(), String> {
    let test_label = Label::from_string("lbl".to_string());
    let cases = [
        (
            LabelRef {
                reference: Ref::Relative(TEST_SEC_ADDR, test_label.clone(), check_jr_jump),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(TEST_SEC_ADDR.0 - 129),
            "jr: max -128 jumps back, was -129",
        ),
        (
            LabelRef {
                reference: Ref::Relative(TEST_SEC_ADDR, test_label.clone(), check_jr_jump),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(TEST_SEC_ADDR.0 + 130),
            "jr: max 127 jumps forward, was 130",
        ),
    ];

    for (label_ref, label_address, err) in cases {
        let mut label_refs = Vec::new();
        label_refs.push(label_ref);

        let mut state = test_state();
        state
            .label_addresses
            .insert(test_label.clone(), label_address);

        let r = resolve_labels(label_refs, &mut state);

        assert!(r.is_err(), "expected error '{}'", err);
        assert_eq!(r.unwrap_err(), err);
    }

    Ok(())
}

#[test]
fn test_resolve_label_ok() -> Result<(), String> {
    let test_label = Label::from_string("lbl".to_string());

    let cases = [
        (
            "jump maximum back",
            LabelRef {
                reference: Ref::Relative(TEST_SEC_ADDR, test_label.clone(), check_jr_jump),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(TEST_SEC_ADDR.0 - 126),
            (-126i32) as u8,
            0x00,
        ),
        (
            "jump maximum forward",
            LabelRef {
                reference: Ref::Relative(TEST_SEC_ADDR, test_label.clone(), check_jr_jump),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(TEST_SEC_ADDR.0 + 127),
            127,
            0x00,
        ),
        (
            "absolute jump",
            LabelRef {
                reference: Ref::from_label(test_label.clone()),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(0x5001),
            0x01,
            0x50,
        ),
        (
            // check ref, Ref::Expression evaluation is tested in interpreter_test.rs
            "arith address 1",
            LabelRef {
                reference: Ref::from_label(test_label.clone()),
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
            },
            Address(0x6754),
            0x54,
            0x67,
        ),
    ];
    for (test, label_ref, label_address, mem_1, mem_2) in cases {
        let patch_ix = label_ref.patch_index;
        let mut label_refs = Vec::new();
        label_refs.push(label_ref);

        let mut state = test_state();
        state
            .label_addresses
            .insert(test_label.clone(), label_address);

        resolve_labels(label_refs, &mut state)?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[patch_ix], mem_1,
            "mem_1 was {:x}, expected {:x}, test: {}",
            sec.memory.mem[patch_ix], mem_1, test,
        );
        assert_eq!(
            sec.memory.mem[patch_ix + 1],
            mem_2,
            "mem_2 was {:x}, expected {:x}, test: {}",
            sec.memory.mem[patch_ix + 1],
            mem_2,
            test,
        );
    }
    Ok(())
}

// helper

static TEST_SEC_NAME: &'static str = "test-section";
static TEST_SEC_ADDR: Address = Address(0x4000);

fn assert_eq_label_ref(got: Option<LabelRef>, expect: Option<LabelRef>) {
    assert!(got.is_some() == expect.is_some());

    if let Some(expect_ref) = expect {
        let got_ref = got.expect("got is some");
        assert_eq_ref(&got_ref.reference, &expect_ref.reference);
        assert_eq!(got_ref.sec_name, expect_ref.sec_name, "sec_name");
        assert_eq!(got_ref.patch_index, expect_ref.patch_index, "patch_index");
    }
}

fn assert_eq_ref(got_ref: &Ref, expect_ref: &Ref) {
    match got_ref {
        Ref::Expression(got_exp) => match expect_ref {
            Ref::Expression(expect_exp) => assert_eq!(got_exp, expect_exp, "Ref::Expression"),
            _ => assert!(false, "got Ref::Expression, expected {:?}", expect_ref),
        },
        Ref::Relative(got_address, got_label, got_check) => match expect_ref {
            Ref::Relative(expect_address, expect_label, expect_check) => {
                assert_eq!(got_address, expect_address, "relative address");
                assert_eq!(got_label, expect_label, "relative label");
                assert!(
                    std::ptr::fn_addr_eq(*got_check, *expect_check),
                    "relative check fn"
                );
            }
            _ => assert!(false, "got Ref::Relative, expected: {:?}", expect_ref),
        },
    }
}

fn test_state() -> State {
    test_state_with_section_offset(TEST_SEC_ADDR)
}

fn test_state_with_section_offset(offset_address: Address) -> State {
    let mut state = State::new();
    state.sections.push(Section {
        name: TEST_SEC_NAME.to_string(),
        length: Some(100),
        offset: offset_address,
        label_only: false,
        memory: Memory {
            mem: vec![0; 100],
            mem_ptr: 0,
        },
    });
    state.current_section_name = Some(TEST_SEC_NAME.to_string());
    state.current_section_address = offset_address;
    state
}
