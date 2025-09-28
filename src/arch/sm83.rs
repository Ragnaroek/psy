#[derive(Debug)]
pub enum Sm83Mnemonic {
    NOP,
    JP,
}

pub struct Sm83Instr {
    pub mnemonic: Sm83Mnemonic,
    pub immediate_args: Vec<String>,
    pub stream_args: usize,
}

pub fn decode(op: u8) -> Result<Sm83Instr, String> {
    match op {
        0x00 => Ok(Sm83Instr {
            mnemonic: Sm83Mnemonic::NOP,
            immediate_args: Vec::with_capacity(0),
            stream_args: 0,
        }),
        0xC3 => Ok(Sm83Instr {
            mnemonic: Sm83Mnemonic::JP,
            immediate_args: Vec::with_capacity(0),
            stream_args: 2,
        }),
        _ => Err(format!("unknown instruction: {:X}", op).to_string()),
    }
}
