pub struct Sm83Instr {
    pub mnemonic: &'static str,
    pub op_code: u8,
    pub immediate_args: &'static [&'static str],
    pub stream_args: usize,
}

pub const REG_HL: &str = "hl";
pub const REG_DE: &str = "de";
pub const REG_A: &str = "a";
pub const REG_B: &str = "b";

pub static INSTR_NOP: Sm83Instr = Sm83Instr {
    mnemonic: "NOP",
    op_code: 0x00,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_RST: Sm83Instr = Sm83Instr {
    mnemonic: "RST",
    op_code: 0xFF,
    immediate_args: &["0x38"],
    stream_args: 0,
};

// JP
pub static INSTR_JP: Sm83Instr = Sm83Instr {
    mnemonic: "JP",
    op_code: 0xC3,
    immediate_args: &[],
    stream_args: 2,
};

// JR
pub static INSTR_JR: Sm83Instr = Sm83Instr {
    mnemonic: "JR",
    op_code: 0x18,
    immediate_args: &[],
    stream_args: 1,
};

pub static INSTR_JR_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "JR #NZ",
    op_code: 0x20,
    immediate_args: &[],
    stream_args: 1,
};

// INC
pub static INSTR_INC_A: Sm83Instr = Sm83Instr {
    mnemonic: "INC %a",
    op_code: 0x3C,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_INC_DE: Sm83Instr = Sm83Instr {
    mnemonic: "INC %de",
    op_code: 0x13,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_INC_HL: Sm83Instr = Sm83Instr {
    mnemonic: "INC %hl",
    op_code: 0x23,
    immediate_args: &[],
    stream_args: 0,
};

// DEC
pub static INSTR_DEC_A: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %a",
    op_code: 0x3D,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_DEC_B: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %b",
    op_code: 0x05,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_DEC_DE: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %de",
    op_code: 0x1B,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTR_DEC_HL: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %hl",
    op_code: 0x2B,
    immediate_args: &[],
    stream_args: 0,
};

// LD

pub static INSTR_LD_TO_HL_FROM_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %hl",
    op_code: 0x21,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_TO_DE_FROM_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %de",
    op_code: 0x11,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_TO_B_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %b",
    op_code: 0x06,
    immediate_args: &[],
    stream_args: 1,
};
pub static INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl)",
    op_code: 0x36,
    immediate_args: &[],
    stream_args: 1,
};
pub static INSTR_LD_TO_DEREF_DE_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%de) %a",
    op_code: 0x12,
    immediate_args: &[],
    stream_args: 1,
};

pub static INSTR_LD_TO_A_FROM_DEREF_HL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%hl)",
    op_code: 0x7E,
    immediate_args: &[],
    stream_args: 1,
};

pub static INSTRUCTIONS: &[&Sm83Instr] = &[
    &INSTR_NOP,
    &INSTR_RST,
    &INSTR_JP,
    &INSTR_JR,
    &INSTR_JR_NZ,
    &INSTR_LD_TO_HL_FROM_LABEL,
    &INSTR_LD_TO_DE_FROM_LABEL,
    &INSTR_LD_TO_B_FROM_IMMEDIATE,
    &INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE,
    &INSTR_LD_TO_DEREF_DE_FROM_A,
    &INSTR_LD_TO_A_FROM_DEREF_HL,
    &INSTR_INC_A,
    &INSTR_INC_DE,
    &INSTR_INC_HL,
    &INSTR_DEC_B,
    &INSTR_DEC_DE,
    &INSTR_DEC_HL,
];

pub fn decode(op: u8) -> Result<&'static Sm83Instr, String> {
    for instr in INSTRUCTIONS {
        if instr.op_code == op {
            return Ok(instr);
        }
    }
    Err(format!("unknown instruction: {:X}", op).to_string())
}
