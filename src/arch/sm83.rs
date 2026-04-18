#[cfg(test)]
#[path = "./sm83_test.rs"]
mod sm83_test;

pub const MAX_INSTRUCTION_BYTE_LENGTH: usize = 3;
pub const SM83_NUM_INSTRUCTIONS: usize = 256;

pub const REG_HL: &str = "hl";
pub const REG_BC: &str = "bc";
pub const REG_DE: &str = "de";
pub const REG_A: &str = "a";
pub const REG_B: &str = "b";
pub const REG_C: &str = "c";

/// carry flag
pub const FLAG_C: &str = "c";
/// not zero flag
pub const FLAG_NZ: &str = "nz";

pub struct Sm83Instr {
    pub mnemonic: &'static str,
    pub op_code: u8,
    pub immediate_args: &'static [&'static str],
    pub stream_args: usize,
}

impl Sm83Instr {
    /// The (psy) text representation of the instruction.
    /// if the binary block is supplied it should start
    /// with the op_code of the instruction and contains the
    /// arguments of the instruction.
    pub fn text(&self, binary: Option<&[u8]>) -> String {
        let mut str = String::new();
        str.push('(');
        str.push_str(self.mnemonic);

        for arg in self.immediate_args {
            str.push(' ');
            str.push_str(arg);
        }

        let ip = 1;
        if self.stream_args == 0 {
            str.push(')');
        } else if self.stream_args == 1
            && let Some(data) = binary
        {
            if ip < data.len() {
                str.push_str(&format!(" 0x{:x})", data[ip]));
            } else {
                str.push_str("ERR)"); //placeholder for now
            }
        } else if self.stream_args == 1 && binary.is_none() {
            str.push_str("n8)")
        } else if self.stream_args == 2
            && let Some(data) = binary
        {
            if ip + 1 < data.len() {
                let a16 = u16::from_le_bytes([data[ip], data[ip + 1]]);
                str.push_str(&format!(" 0x{:x})", a16));
            } else {
                str.push_str("ERR)"); //placeholder for now
            }
        } else if self.stream_args == 2 && binary.is_none() {
            str.push_str("n16)");
        }

        str
    }
}

/// A invalid instruction. Used to represent an instruction in
/// disassemble that cannot be decoded.
pub static INSTR_INVALID: Sm83Instr = Sm83Instr {
    mnemonic: "!!!",
    op_code: 0xD3, //invalid op_code in SM83
    immediate_args: &[],
    stream_args: 0,
};

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
pub static INSTR_JP_IF_C: Sm83Instr = Sm83Instr {
    mnemonic: "JP #C",
    op_code: 0xDA,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_JP_IF_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "JP #NZ",
    op_code: 0xC2,
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
pub static INSTR_JR_IF_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "JR #NZ",
    op_code: 0x20,
    immediate_args: &[],
    stream_args: 1,
};
pub static INSTR_JR_IF_C: Sm83Instr = Sm83Instr {
    mnemonic: "JR #C",
    op_code: 0x38,
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

pub static INSTR_INC_BC: Sm83Instr = Sm83Instr {
    mnemonic: "INC %bc",
    op_code: 0x03,
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

pub static INSTR_DEC_BC: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %bc",
    op_code: 0x0B,
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

pub static INSTR_LD_TO_HL_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %hl",
    op_code: 0x21,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_TO_DE_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %de",
    op_code: 0x11,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_TO_BC_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %bc",
    op_code: 0x01,
    immediate_args: &[],
    stream_args: 2,
};
pub static INSTR_LD_TO_A_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a",
    op_code: 0x3E,
    immediate_args: &[],
    stream_args: 1,
};
pub static INSTR_LD_TO_A_FROM_DEREF_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a ('lbl)",
    op_code: 0xFA,
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
pub static INSTR_LD_TO_DEREF_HL_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl) %a",
    op_code: 0x77,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_LD_TO_DEREF_HL_INC_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl +) %a",
    op_code: 0x22,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_LD_TO_DEREF_LABEL_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD ('lbl) %a",
    op_code: 0xEA,
    immediate_args: &[],
    stream_args: 2,
};

pub static INSTR_LD_TO_A_FROM_DEREF_HL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%hl)",
    op_code: 0x7E,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_LD_TO_A_FROM_DEREF_HL_INC: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%hl +)",
    op_code: 0x2A,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_LD_TO_A_FROM_DEREF_DE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%de)",
    op_code: 0x1A,
    immediate_args: &[],
    stream_args: 0,
};
pub static INSTR_LD_TO_A_FROM_B: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a %b",
    op_code: 0x78,
    immediate_args: &[],
    stream_args: 0,
};
// CP
pub static INSTR_CP_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "CP",
    op_code: 0xFE,
    immediate_args: &[],
    stream_args: 1,
};
// OR
pub static INSTR_OR_A_C: Sm83Instr = Sm83Instr {
    mnemonic: "OR %a %c",
    op_code: 0xB1,
    immediate_args: &[],
    stream_args: 0,
};

pub static INSTRUCTIONS: [&Sm83Instr; SM83_NUM_INSTRUCTIONS] = [
    /*0x00*/ &INSTR_NOP,
    /*0x01*/ &INSTR_LD_TO_BC_FROM_IMMEDIATE,
    /*0x02*/ &INSTR_INVALID,
    /*0x03*/ &INSTR_INC_BC,
    /*0x04*/ &INSTR_INVALID,
    /*0x05*/ &INSTR_DEC_B,
    /*0x06*/ &INSTR_LD_TO_B_FROM_IMMEDIATE,
    /*0x07*/ &INSTR_INVALID,
    /*0x08*/ &INSTR_INVALID,
    /*0x09*/ &INSTR_INVALID,
    /*0x0A*/ &INSTR_INVALID,
    /*0x0B*/ &INSTR_DEC_BC,
    /*0x0C*/ &INSTR_INVALID,
    /*0x0D*/ &INSTR_INVALID,
    /*0x0E*/ &INSTR_INVALID,
    /*0x0F*/ &INSTR_INVALID,
    /*0x10*/ &INSTR_INVALID,
    /*0x11*/ &INSTR_LD_TO_DE_FROM_IMMEDIATE,
    /*0x12*/ &INSTR_LD_TO_DEREF_DE_FROM_A,
    /*0x13*/ &INSTR_INC_DE,
    /*0x14*/ &INSTR_INVALID,
    /*0x15*/ &INSTR_INVALID,
    /*0x16*/ &INSTR_INVALID,
    /*0x17*/ &INSTR_INVALID,
    /*0x18*/ &INSTR_JR,
    /*0x19*/ &INSTR_INVALID,
    /*0x1A*/ &INSTR_LD_TO_A_FROM_DEREF_DE,
    /*0x1B*/ &INSTR_DEC_DE,
    /*0x1C*/ &INSTR_INVALID,
    /*0x1D*/ &INSTR_INVALID,
    /*0x1E*/ &INSTR_INVALID,
    /*0x1F*/ &INSTR_INVALID,
    /*0x20*/ &INSTR_JR_IF_NZ,
    /*0x21*/ &INSTR_LD_TO_HL_FROM_IMMEDIATE,
    /*0x22*/ &INSTR_LD_TO_DEREF_HL_INC_FROM_A,
    /*0x23*/ &INSTR_INC_HL,
    /*0x24*/ &INSTR_INVALID,
    /*0x25*/ &INSTR_INVALID,
    /*0x26*/ &INSTR_INVALID,
    /*0x27*/ &INSTR_INVALID,
    /*0x28*/ &INSTR_INVALID,
    /*0x29*/ &INSTR_INVALID,
    /*0x2A*/ &INSTR_LD_TO_A_FROM_DEREF_HL_INC,
    /*0x2B*/ &INSTR_DEC_HL,
    /*0x2C*/ &INSTR_INVALID,
    /*0x2D*/ &INSTR_INVALID,
    /*0x2E*/ &INSTR_INVALID,
    /*0x2F*/ &INSTR_INVALID,
    /*0x30*/ &INSTR_INVALID,
    /*0x31*/ &INSTR_INVALID,
    /*0x32*/ &INSTR_INVALID,
    /*0x33*/ &INSTR_INVALID,
    /*0x34*/ &INSTR_INVALID,
    /*0x35*/ &INSTR_INVALID,
    /*0x36*/ &INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE,
    /*0x37*/ &INSTR_INVALID,
    /*0x38*/ &INSTR_JR_IF_C,
    /*0x39*/ &INSTR_INVALID,
    /*0x3A*/ &INSTR_INVALID,
    /*0x3B*/ &INSTR_INVALID,
    /*0x3C*/ &INSTR_INC_A,
    /*0x3D*/ &INSTR_DEC_A,
    /*0x3E*/ &INSTR_LD_TO_A_FROM_IMMEDIATE,
    /*0x3F*/ &INSTR_INVALID,
    /*0x40*/ &INSTR_INVALID,
    /*0x41*/ &INSTR_INVALID,
    /*0x42*/ &INSTR_INVALID,
    /*0x43*/ &INSTR_INVALID,
    /*0x44*/ &INSTR_INVALID,
    /*0x45*/ &INSTR_INVALID,
    /*0x46*/ &INSTR_INVALID,
    /*0x47*/ &INSTR_INVALID,
    /*0x48*/ &INSTR_INVALID,
    /*0x49*/ &INSTR_INVALID,
    /*0x4A*/ &INSTR_INVALID,
    /*0x4B*/ &INSTR_INVALID,
    /*0x4C*/ &INSTR_INVALID,
    /*0x4D*/ &INSTR_INVALID,
    /*0x4E*/ &INSTR_INVALID,
    /*0x4F*/ &INSTR_INVALID,
    /*0x50*/ &INSTR_INVALID,
    /*0x51*/ &INSTR_INVALID,
    /*0x52*/ &INSTR_INVALID,
    /*0x53*/ &INSTR_INVALID,
    /*0x54*/ &INSTR_INVALID,
    /*0x55*/ &INSTR_INVALID,
    /*0x56*/ &INSTR_INVALID,
    /*0x57*/ &INSTR_INVALID,
    /*0x58*/ &INSTR_INVALID,
    /*0x59*/ &INSTR_INVALID,
    /*0x5A*/ &INSTR_INVALID,
    /*0x5B*/ &INSTR_INVALID,
    /*0x5C*/ &INSTR_INVALID,
    /*0x5D*/ &INSTR_INVALID,
    /*0x5E*/ &INSTR_INVALID,
    /*0x5F*/ &INSTR_INVALID,
    /*0x60*/ &INSTR_INVALID,
    /*0x61*/ &INSTR_INVALID,
    /*0x62*/ &INSTR_INVALID,
    /*0x63*/ &INSTR_INVALID,
    /*0x64*/ &INSTR_INVALID,
    /*0x65*/ &INSTR_INVALID,
    /*0x66*/ &INSTR_INVALID,
    /*0x67*/ &INSTR_INVALID,
    /*0x68*/ &INSTR_INVALID,
    /*0x69*/ &INSTR_INVALID,
    /*0x6A*/ &INSTR_INVALID,
    /*0x6B*/ &INSTR_INVALID,
    /*0x6C*/ &INSTR_INVALID,
    /*0x6D*/ &INSTR_INVALID,
    /*0x6E*/ &INSTR_INVALID,
    /*0x6F*/ &INSTR_INVALID,
    /*0x70*/ &INSTR_INVALID,
    /*0x71*/ &INSTR_INVALID,
    /*0x72*/ &INSTR_INVALID,
    /*0x73*/ &INSTR_INVALID,
    /*0x74*/ &INSTR_INVALID,
    /*0x75*/ &INSTR_INVALID,
    /*0x76*/ &INSTR_INVALID,
    /*0x77*/ &INSTR_LD_TO_DEREF_HL_FROM_A,
    /*0x78*/ &INSTR_LD_TO_A_FROM_B,
    /*0x79*/ &INSTR_INVALID,
    /*0x7A*/ &INSTR_INVALID,
    /*0x7B*/ &INSTR_INVALID,
    /*0x7C*/ &INSTR_INVALID,
    /*0x7D*/ &INSTR_INVALID,
    /*0x7E*/ &INSTR_LD_TO_A_FROM_DEREF_HL,
    /*0x7F*/ &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    &INSTR_INVALID,
    /*0xB1*/ &INSTR_OR_A_C,
    /*0xB2*/ &INSTR_INVALID,
    /*0xB3*/ &INSTR_INVALID,
    /*0xB4*/ &INSTR_INVALID,
    /*0xB5*/ &INSTR_INVALID,
    /*0xB6*/ &INSTR_INVALID,
    /*0xB7*/ &INSTR_INVALID,
    /*0xB8*/ &INSTR_INVALID,
    /*0xB9*/ &INSTR_INVALID,
    /*0xBA*/ &INSTR_INVALID,
    /*0xBB*/ &INSTR_INVALID,
    /*0xBC*/ &INSTR_INVALID,
    /*0xBD*/ &INSTR_INVALID,
    /*0xBE*/ &INSTR_INVALID,
    /*0xBF*/ &INSTR_INVALID,
    /*0xC0*/ &INSTR_INVALID,
    /*0xC1*/ &INSTR_INVALID,
    /*0xC2*/ &INSTR_JP_IF_NZ,
    /*0xC3*/ &INSTR_JP,
    /*0xC4*/ &INSTR_INVALID,
    /*0xC5*/ &INSTR_INVALID,
    /*0xC6*/ &INSTR_INVALID,
    /*0xC7*/ &INSTR_INVALID,
    /*0xC8*/ &INSTR_INVALID,
    /*0xC9*/ &INSTR_INVALID,
    /*0xCA*/ &INSTR_INVALID,
    /*0xCB*/ &INSTR_INVALID,
    /*0xCC*/ &INSTR_INVALID,
    /*0xCD*/ &INSTR_INVALID,
    /*0xCE*/ &INSTR_INVALID,
    /*0xCF*/ &INSTR_INVALID,
    /*0xD0*/ &INSTR_INVALID,
    /*0xD1*/ &INSTR_INVALID,
    /*0xD2*/ &INSTR_INVALID,
    /*0xD3*/ &INSTR_INVALID,
    /*0xD4*/ &INSTR_INVALID,
    /*0xD5*/ &INSTR_INVALID,
    /*0xD6*/ &INSTR_INVALID,
    /*0xD7*/ &INSTR_INVALID,
    /*0xD8*/ &INSTR_INVALID,
    /*0xD9*/ &INSTR_INVALID,
    /*0xDA*/ &INSTR_JP_IF_C,
    /*0xDB*/ &INSTR_INVALID,
    /*0xDC*/ &INSTR_INVALID,
    /*0xDD*/ &INSTR_INVALID,
    /*0xDE*/ &INSTR_INVALID,
    /*0xDF*/ &INSTR_INVALID,
    /*0xE9*/ &INSTR_INVALID,
    /*0xE1*/ &INSTR_INVALID,
    /*0xE2*/ &INSTR_INVALID,
    /*0xE3*/ &INSTR_INVALID,
    /*0xE4*/ &INSTR_INVALID,
    /*0xE5*/ &INSTR_INVALID,
    /*0xE6*/ &INSTR_INVALID,
    /*0xE7*/ &INSTR_INVALID,
    /*0xE8*/ &INSTR_INVALID,
    /*0xE9*/ &INSTR_INVALID,
    /*0xEA*/ &INSTR_LD_TO_DEREF_LABEL_FROM_A,
    /*0xEB*/ &INSTR_INVALID,
    /*0xEC*/ &INSTR_INVALID,
    /*0xED*/ &INSTR_INVALID,
    /*0xEE*/ &INSTR_INVALID,
    /*0xEF*/ &INSTR_INVALID,
    /*0xF0*/ &INSTR_INVALID,
    /*0xF1*/ &INSTR_INVALID,
    /*0xF2*/ &INSTR_INVALID,
    /*0xF3*/ &INSTR_INVALID,
    /*0xF4*/ &INSTR_INVALID,
    /*0xF5*/ &INSTR_INVALID,
    /*0xF6*/ &INSTR_INVALID,
    /*0xF7*/ &INSTR_INVALID,
    /*0xF8*/ &INSTR_INVALID,
    /*0xF9*/ &INSTR_INVALID,
    /*0xFA*/ &INSTR_LD_TO_A_FROM_DEREF_LABEL,
    /*0xFB*/ &INSTR_INVALID,
    /*0xFC*/ &INSTR_INVALID,
    /*0xFD*/ &INSTR_INVALID,
    /*0xFE*/ &INSTR_CP_IMMEDIATE,
    /*0xFF*/ &INSTR_RST,
];

pub fn decode(op: u8) -> &'static Sm83Instr {
    INSTRUCTIONS[op as usize]
}
