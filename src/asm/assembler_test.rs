use crate::arch::sm83::{INSTR_LD_B_IMMEDIATE, INSTR_LD_DE_LABEL, INSTR_LD_HL_LABEL};
use crate::asm::assembler::{
    JP, JR, JR_NZ, Label, Memory, Section, State, UnresolvedLabel, check_16_bit_address_range,
    check_jr_jump, ds, expect_label_name, jp, jr, ld,
};

use crate::asm::parser::{Address, parse_from_string};

#[test]
fn test_ds_ok() -> Result<(), String> {
    let cases = [("(ds 0)", 0), ("(ds 1", 1), ("(ds 66)", 66)];

    for (exp, mem_ptr) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;

        ds(&mut state, &tl.forms[0])?;

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
            "(jr 'lbl)",
            Address(0x4000 - 127 * 8),
            "jr: max -128 jumps back, was -129",
        ),
        (
            "(jr 'lbl)",
            Address(0x4000 + 130 * 8),
            "jr: max 127 jumps forward, was 128",
        ),
        (
            // unknown flag
            "(jr #tz 'lbl)",
            Address(0x4000 + 2),
            "jr: unknown flag 'tz'",
        ),
    ];

    for (exp, lbl_address, err) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;

        if !tl.forms[0].exps.is_empty() {
            let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
            state.label_addresses.insert(lbl, lbl_address);
        }

        let r = jr(&mut state, &tl.forms[0]);

        assert_eq!(r.unwrap_err(), err);
    }
    Ok(())
}

#[test]
fn test_jr_ok() -> Result<(), String> {
    let cases = [
        // jump to self
        ("(jr 'lbl)", Some(Address(0x4000)), None, JR, 0xFE),
        // jump maximum backward
        ("(jr 'lbl)", Some(Address(0x4000 - 126 * 8)), None, JR, 0x80),
        // jump maximum forward
        ("(jr 'lbl)", Some(Address(0x4000 + 129 * 8)), None, JR, 0x7F),
        // jump nz
        (
            "(jr #nz 'lbl)",
            Some(Address(0x4000 + 129 * 8)),
            None,
            JR_NZ,
            0x7F,
        ),
        // forward jump, address not yet defined
        (
            "(jr 'forward)",
            None,
            Some(UnresolvedLabel {
                relative_from: Some(Address(16400)), // TEST_SEC_ADDRESS +2 bytes
                label: Label::from_string("forward".to_string()),
                check: check_jr_jump,
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
                patch_width: 1,
            }),
            JR,
            0x00,
        ),
    ];

    for (exp, may_lbl_address, expect_unresolved_info, inst1, inst2) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;
        let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
        let mut expect_address = 0;
        if let Some(lbl_address) = may_lbl_address {
            state.label_addresses.insert(lbl, lbl_address);
            expect_address = lbl_address.0 as i32;
        }

        let got_unresolved_info = jr(&mut state, &tl.forms[0])?;

        assert_eq_unresolved_label(got_unresolved_info, expect_unresolved_info);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], inst1,
            "address={:?}, inst1 was {:x}",
            expect_address, sec.memory.mem[0]
        );
        assert_eq!(
            sec.memory.mem[1], inst2,
            "address={:?}, inst2 was {:x}",
            expect_address, sec.memory.mem[1]
        );
    }

    Ok(())
}

#[test]
fn test_jp_ok() -> Result<(), String> {
    let cases = [
        // jump to self
        ("(jp 'lbl)", Some(Address(0x4000)), None, JP, 0x00, 0x40),
        // forward jump, address not yet defined
        (
            "(jp 'forward)",
            None,
            Some(UnresolvedLabel {
                relative_from: None,
                label: Label::from_string("forward".to_string()),
                check: check_16_bit_address_range,
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
                patch_width: 2,
            }),
            JP,
            0x00,
            0x00,
        ),
    ];

    for (exp, may_lbl_address, expect_unresolved_info, inst1, inst2, inst3) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;
        let lbl = expect_label_name(&tl.forms[0].exps[0])?;
        let mut expect_address = 0;
        if let Some(lbl_address) = may_lbl_address {
            state.label_addresses.insert(lbl, lbl_address);
            expect_address = lbl_address.0 as i32;
        }

        let got_unresolved_info = jp(&mut state, &tl.forms[0])?;

        assert_eq_unresolved_label(got_unresolved_info, expect_unresolved_info);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], inst1,
            "address={:?}, inst1 was {:x}",
            expect_address, sec.memory.mem[0]
        );
        assert_eq!(
            sec.memory.mem[1], inst2,
            "address={:?}, inst2 was {:x}",
            expect_address, sec.memory.mem[1]
        );
        assert_eq!(
            sec.memory.mem[2], inst3,
            "address={:?}, inst3 was {:x}",
            expect_address, sec.memory.mem[2]
        );
    }
    Ok(())
}

#[test]
fn test_ld_fails() -> Result<(), String> {
    let cases = [("(ld)", Address(0), "ld: needs at least two arguments")];

    for (exp, lbl_address, err) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;

        if !tl.forms[0].exps.is_empty() {
            let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
            state.label_addresses.insert(lbl, lbl_address);
        }

        let r = ld(&mut state, &tl.forms[0]);

        assert_eq!(r.unwrap_err(), err);
    }
    Ok(())
}

#[test]
fn test_ld_ok() -> Result<(), String> {
    let cases = [
        // load mem hl to reg
        (
            "(ld %hl 'lbl)",
            Some(Address(0x4000)),
            None,
            INSTR_LD_HL_LABEL.op_code,
            0x00,
            0x40,
        ),
        // load hl mem to reg forward
        (
            "(ld %hl 'forward)",
            None,
            Some(UnresolvedLabel {
                relative_from: None,
                label: Label::from_string("forward".to_string()),
                check: check_16_bit_address_range,
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1,
                patch_width: 2,
            }),
            INSTR_LD_HL_LABEL.op_code,
            0x00,
            0x00,
        ),
        (
            "(ld %de 'lbl)",
            Some(Address(0x5001)),
            None,
            INSTR_LD_DE_LABEL.op_code,
            0x01,
            0x50,
        ),
        // load immediate to reg
        (
            "(ld %de 42)",
            None,
            None,
            INSTR_LD_B_IMMEDIATE.op_code,
            0x2A,
            0x00,
        ),
    ];

    for (exp, may_lbl_address, expect_unresolved_info, inst1, inst2, inst3) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;

        let mut expect_address = 0;
        if let Some(lbl_address) = may_lbl_address {
            let lbl = expect_label_name(&tl.forms[0].exps[tl.forms[0].exps.len() - 1])?;
            state.label_addresses.insert(lbl, lbl_address);
            expect_address = lbl_address.0 as i32;
        }

        let got_unresolved_info = ld(&mut state, &tl.forms[0])?;

        assert_eq_unresolved_label(got_unresolved_info, expect_unresolved_info);

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], inst1,
            "address={:?}, inst1 was {:x}",
            expect_address, sec.memory.mem[0]
        );
        assert_eq!(
            sec.memory.mem[1], inst2,
            "address={:?}, inst2 was {:x}",
            expect_address, sec.memory.mem[1]
        );
        assert_eq!(
            sec.memory.mem[2], inst3,
            "address={:?}, inst3 was {:x}",
            expect_address, sec.memory.mem[2]
        );
    }
    Ok(())
}
// helper

static TEST_SEC_NAME: &'static str = "test-section";
static TEST_SEC_ADDR: Address = Address(0x4000);

fn assert_eq_unresolved_label(got: Option<UnresolvedLabel>, expect: Option<UnresolvedLabel>) {
    assert!(got.is_some() == expect.is_some());

    if let Some(expect_unresolved) = expect {
        let got_unresolved = got.expect("got is some");
        assert_eq!(
            got_unresolved.relative_from, expect_unresolved.relative_from,
            "relative_from"
        );
        assert_eq!(got_unresolved.label, expect_unresolved.label, "label");
        assert_eq!(
            got_unresolved.sec_name, expect_unresolved.sec_name,
            "sec_name"
        );
        assert_eq!(
            got_unresolved.patch_index, expect_unresolved.patch_index,
            "patch_index"
        );
        assert_eq!(
            got_unresolved.patch_width, expect_unresolved.patch_width,
            "patch_width"
        );
    }
}

fn test_state() -> State {
    let mut state = State::new();
    state.sections.push(Section {
        name: TEST_SEC_NAME.to_string(),
        length: Some(100),
        offset: TEST_SEC_ADDR,
        label_only: false,
        memory: Memory {
            mem: vec![0; 100],
            mem_ptr: 0,
        },
    });
    state.current_section_name = Some(TEST_SEC_NAME.to_string());
    state.current_section_address = TEST_SEC_ADDR;
    state
}
