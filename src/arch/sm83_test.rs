use crate::arch::sm83::{INSTR_INVALID, INSTRUCTIONS, SM83_NUM_INSTRUCTIONS};

#[test]
fn test_op_code_matches_index() -> Result<(), String> {
    for i in 0..SM83_NUM_INSTRUCTIONS {
        let instr = INSTRUCTIONS[i];
        if instr.op_code != INSTR_INVALID.op_code {
            assert_eq!(
                instr.op_code, i as u8,
                "o_code=0x{:x}, index=0x{:x}",
                instr.op_code, i
            );
        }
    }
    Ok(())
}
