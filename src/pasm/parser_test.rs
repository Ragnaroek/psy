use std::fs::File;

use crate::pasm::parser::parse_file;

#[test]
fn test_parse() -> std::io::Result<()> {
    let mut f = File::open("testdata/pasm/test.asm")?;
    let result = parse_file(&mut f);
    println!("result = {:?}", result);
    Ok(())
}
