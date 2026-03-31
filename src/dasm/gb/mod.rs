#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use std::collections::HashMap;

use crate::arch::sm83::{self, Sm83Instr};

pub struct GBDisInstr {
    pub offset: usize, // offset into the original byte sequence that produced this disassembly
    pub len: usize,    // length of the instruction in bytes
    pub instr: &'static Sm83Instr,
}

pub struct GBDisassembly {
    pub instructions: HashMap<usize, GBDisInstr>,
}

pub fn disassemble(data: &[u8]) -> Result<GBDisassembly, String> {
    let mut instructions = HashMap::new();
    let mut ip = 0;
    while ip < data.len() {
        let start_ip = ip;
        println!("data = {:x}", data[start_ip]);
        let instr = sm83::decode(data[start_ip]);
        ip += 1 + instr.immediate_args.len() + instr.stream_args;
        instructions.insert(
            start_ip,
            GBDisInstr {
                offset: start_ip,
                len: ip - start_ip,
                instr,
            },
        );
    }
    return Ok(GBDisassembly { instructions });
}
