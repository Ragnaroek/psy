use crate::asm::assembler::{
    Label, Memory, Section, State, UnresolvedLabel, check_jr_jump, ds, expect_label_name, jr,
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
    ];

    for (exp, lbl_address, err) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;

        if !tl.forms[0].exps.is_empty() {
            let lbl = expect_label_name(&tl.forms[0].exps[0])?;
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
        ("(jr 'lbl)", Some(Address(0x4000)), None, 0x18, 0xFE),
        // jump maximum backward
        (
            "(jr 'lbl)",
            Some(Address(0x4000 - 126 * 8)),
            None,
            0x18,
            0x80,
        ),
        // jump maximum forward
        (
            "(jr 'lbl)",
            Some(Address(0x4000 + 129 * 8)),
            None,
            0x18,
            0x7F,
        ),
        // forward jump, address not yet defined
        (
            "(jr 'forward)",
            None,
            Some(UnresolvedLabel {
                relative_from: Address(16400), // TEST_SEC_ADDRESS +2 bytes
                label: Label::from_string("forward".to_string()),
                check: check_jr_jump,
                sec_name: TEST_SEC_NAME.to_string(),
                patch_index: 1, // address bytes start at byte 1
            }),
            0x18,
            0x00,
        ),
    ];

    for (exp, may_lbl_address, expect_unresolved_info, inst1, inst2) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;
        let lbl = expect_label_name(&tl.forms[0].exps[0])?;
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

// TODO Test jump out of bound neg and positive

// helper

static TEST_SEC_NAME: &'static str = "test-section";
static TEST_SEC_ADDR: Address = Address(0x4000);

fn assert_eq_unresolved_label(got: Option<UnresolvedLabel>, expect: Option<UnresolvedLabel>) {
    assert!(got.is_some() == expect.is_some());

    if let Some(expect_unresolved) = expect {
        let got_unresolved = got.expect("got is some");
        assert_eq!(
            got_unresolved.relative_from,
            expect_unresolved.relative_from
        );
        assert_eq!(got_unresolved.label, expect_unresolved.label);
        assert_eq!(got_unresolved.sec_name, expect_unresolved.sec_name);
        assert_eq!(got_unresolved.patch_index, expect_unresolved.patch_index);
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
