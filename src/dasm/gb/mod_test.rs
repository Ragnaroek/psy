use crate::dasm::gb::disassemble;

#[test]
fn test_disassemble() -> Result<(), String> {
    let cases = [(vec!["(ld %a 0)"], vec![0x3E, 0x00])];
    for (texts, bytes) in cases {
        let dis = disassemble(&bytes)?;
        assert_eq!(dis.instructions.len(), texts.len());
    }

    Ok(())
}
