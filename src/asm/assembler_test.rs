use crate::asm::assembler::{Memory, Section, State, ds, expect_label_name, jr};
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
        ("(jr 'lbl)", Address(0x4000), 0x18, 0xFE),
        // jump maximum backward
        ("(jr 'lbl)", Address(0x4000 - 126 * 8), 0x18, 0x80),
        // jump maximum forward
        ("(jr 'lbl)", Address(0x4000 + 129 * 8), 0x18, 0x7F),
    ];

    for (exp, lbl_address, inst1, inst2) in cases {
        let mut state = test_state();
        let tl = parse_from_string(exp)?;
        let lbl = expect_label_name(&tl.forms[0].exps[0])?;
        state.label_addresses.insert(lbl, lbl_address);

        jr(&mut state, &tl.forms[0])?;

        let sec = state.lookup_section(&TEST_SEC_NAME).expect("test sec");
        assert_eq!(
            sec.memory.mem[0], inst1,
            "address={:?}, inst1 was {:x}",
            lbl_address, sec.memory.mem[0]
        );
        assert_eq!(
            sec.memory.mem[1], inst2,
            "address={:?}, inst2 was {:x}",
            lbl_address, sec.memory.mem[1]
        );
    }

    Ok(())
}

// TODO Test jump out of bound neg and positive

// helper

static TEST_SEC_NAME: &'static str = "test-section";
static TEST_SEC_ADDR: Address = Address(0x4000);

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
