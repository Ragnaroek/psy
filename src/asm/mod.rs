use std::fs::File;

use crate::pasm;

pub fn assemble(file: &mut File) -> Result<Vec<u8>, String> {
    let tl = pasm::parser::parse_file(file)?;
    pasm::interpreter::interpret(tl)?;
    Ok(Vec::with_capacity(0))
}
