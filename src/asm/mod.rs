use std::fs::File;

use crate::pasm;

pub fn assemble(file: &mut File) -> Result<Vec<u8>, String> {
    let ast = pasm::parser::parse_file(file)?;
    println!("ast = {:?}", ast);
    Ok(Vec::with_capacity(0))
}
