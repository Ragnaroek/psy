pub struct Sm83Instr {
    pub mnemonic: &'static str,
    pub op_code: u8,
    pub immediate_args: &'static [&'static str],
    pub stream_args: usize,
}

pub const REG_HL: &str = "hl";
pub const REG_DE: &str = "de";
pub const REG_B: &str = "b";

pub static INSTR_NOP: Sm83Instr = Sm83Instr {
    mnemonic: "NOP",
    op_code: 0x00,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_JP: Sm83Instr = Sm83Instr {
    mnemonic: "JP",
    op_code: 0xC3,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_RST: Sm83Instr = Sm83Instr {
    mnemonic: "RST",
    op_code: 0xFF,
    immediate_args: &["0x38"],
    stream_args: 0,
};
pub static INSTR_LD_HL_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD HL",
    op_code: 0x21,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_DE_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD DE",
    op_code: 0x11,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_B_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD B",
    op_code: 0x06,
    immediate_args: &[],
    stream_args: 1,
};

pub static INSTRUCTIONS: &[&Sm83Instr] = &[
    &INSTR_NOP,
    &INSTR_JP,
    &INSTR_RST,
    &INSTR_LD_HL_LABEL,
    &INSTR_LD_DE_LABEL,
    &INSTR_LD_B_IMMEDIATE,
];

pub fn decode(op: u8) -> Result<&'static Sm83Instr, String> {
    for instr in INSTRUCTIONS {
        if instr.op_code == op {
            return Ok(instr);
        }
    }
    Err(format!("unknown instruction: {:X}", op).to_string())
}
