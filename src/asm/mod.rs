pub mod assembler;
mod parser;

use crate::asm;
use std::fs::File;

pub fn assemble_file(file: &mut File, options: asm::assembler::Options) -> Result<(), String> {
    let tl = asm::parser::parse_file(file)?;
    asm::assembler::assemble(tl, options)
}
