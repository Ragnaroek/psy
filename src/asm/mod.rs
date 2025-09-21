pub mod assembler;
mod parser;

use crate::asm;
use std::fs::File;

pub fn assemble_file(file: &mut File) -> Result<Vec<u8>, String> {
    let tl = asm::parser::parse_file(file)?;
    asm::assembler::assemble(tl)?;
    Ok(Vec::with_capacity(0))
}
