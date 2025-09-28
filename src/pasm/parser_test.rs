use std::fs::File;

use crate::pasm::parser::parse_file;

#[test]
fn test_parse_test_file() -> Result<(), String> {
    let mut f = File::open("testdata/pasm/test.asm").map_err(|e| e.to_string())?;
    let tl = parse_file(&mut f)?;

    assert_eq!(tl.forms.len(), 13);
    Ok(())
}
