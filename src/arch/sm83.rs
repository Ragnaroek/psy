#[cfg(test)]
#[path = "./sm83_test.rs"]
mod sm83_test;

pub const MAX_INSTRUCTION_BYTE_LENGTH: usize = 3;
pub const SM83_NUM_INSTRUCTIONS: usize = 256;
pub const SM83_NUM_PREFIX_INSTRUCTIONS: usize = 256;

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

#[derive(Debug)]
pub struct Sm83Instr {
    pub mnemonic: &'static str,
    pub op_code: u8,
    pub arg_bytes: usize,
}

pub struct Sm83PrefixInstr {
    pub mnemonic: &'static str,
    pub op_code: u8,
}

impl Sm83Instr {
    /// The length of the instruction in bytes.
    pub fn len(&self) -> usize {
        self.arg_bytes + 1 // +1 for the op code
    }

    /// The (psy) text representation of the instruction.
    /// if the binary block is supplied it should start
    /// with the op_code of the instruction and contains the
    /// arguments of the instruction.
    pub fn text(&self, binary: Option<&[u8]>) -> String {
        let mut str = String::new();
        str.push('(');
        str.push_str(self.mnemonic);

        let ip = 1;
        if self.arg_bytes == 0 {
            str.push(')');
        } else if self.arg_bytes == 1
            && let Some(data) = binary
        {
            if ip < data.len() {
                str.push_str(&format!(" 0x{:x})", data[ip]));
            } else {
                str.push_str("ERR)"); //placeholder for now
            }
        } else if self.arg_bytes == 1 && binary.is_none() {
            str.push_str(" n8)")
        } else if self.arg_bytes == 2
            && let Some(data) = binary
        {
            if ip + 1 < data.len() {
                let a16 = u16::from_le_bytes([data[ip], data[ip + 1]]);
                str.push_str(&format!(" 0x{:x})", a16));
            } else {
                str.push_str("ERR)"); //placeholder for now
            }
        } else if self.arg_bytes == 2 && binary.is_none() {
            str.push_str(" n16)");
        }

        str
    }
}

/// A invalid instruction. Used to represent an instruction in
/// disassemble that cannot be decoded.
pub static INSTR_INVALID: Sm83Instr = Sm83Instr {
    mnemonic: "!!!",
    op_code: 0xD3, //invalid op_code in SM83
    arg_bytes: 0,
};

pub static INSTR_NOP: Sm83Instr = Sm83Instr {
    mnemonic: "NOP",
    op_code: 0x00,
    arg_bytes: 0,
};

// RST
pub static INSTR_RST_38: Sm83Instr = Sm83Instr {
    mnemonic: "RST 0x38",
    op_code: 0xFF,
    arg_bytes: 0,
};

pub static INSTR_RST_28: Sm83Instr = Sm83Instr {
    mnemonic: "RST 0x28",
    op_code: 0xEF,
    arg_bytes: 0,
};

pub static INSTR_DI: Sm83Instr = Sm83Instr {
    mnemonic: "DI",
    op_code: 0xF3,
    arg_bytes: 0,
};

pub static INSTR_EI: Sm83Instr = Sm83Instr {
    mnemonic: "EI",
    op_code: 0xFB,
    arg_bytes: 0,
};

// JP
pub static INSTR_JP: Sm83Instr = Sm83Instr {
    mnemonic: "JP",
    op_code: 0xC3,
    arg_bytes: 2,
};
pub static INSTR_JP_IF_C: Sm83Instr = Sm83Instr {
    mnemonic: "JP #C",
    op_code: 0xDA,
    arg_bytes: 2,
};
pub static INSTR_JP_IF_Z: Sm83Instr = Sm83Instr {
    mnemonic: "JP #Z",
    op_code: 0xCA,
    arg_bytes: 2,
};
pub static INSTR_JP_IF_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "JP #NZ",
    op_code: 0xC2,
    arg_bytes: 2,
};
pub static INSTR_JP_HL: Sm83Instr = Sm83Instr {
    mnemonic: "JP %hl",
    op_code: 0xE9,
    arg_bytes: 0,
};

// JR
pub static INSTR_JR: Sm83Instr = Sm83Instr {
    mnemonic: "JR",
    op_code: 0x18,
    arg_bytes: 1,
};
pub static INSTR_JR_IF_Z: Sm83Instr = Sm83Instr {
    mnemonic: "JR #Z",
    op_code: 0x28,
    arg_bytes: 1,
};
pub static INSTR_JR_IF_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "JR #NZ",
    op_code: 0x20,
    arg_bytes: 1,
};
pub static INSTR_JR_IF_C: Sm83Instr = Sm83Instr {
    mnemonic: "JR #C",
    op_code: 0x38,
    arg_bytes: 1,
};

// ADD
pub static INSTR_ADD_A_A: Sm83Instr = Sm83Instr {
    mnemonic: "ADD %a %a",
    op_code: 0x87,
    arg_bytes: 0,
};
pub static INSTR_ADD_HL_DE: Sm83Instr = Sm83Instr {
    mnemonic: "ADD %hl %de",
    op_code: 0x19,
    arg_bytes: 0,
};
pub static INSTR_ADD_SP_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "ADD %sp n8",
    op_code: 0xE8,
    arg_bytes: 1,
};

// INC
pub static INSTR_INC_A: Sm83Instr = Sm83Instr {
    mnemonic: "INC %a",
    op_code: 0x3C,
    arg_bytes: 0,
};
pub static INSTR_INC_C: Sm83Instr = Sm83Instr {
    mnemonic: "INC %c",
    op_code: 0x0C,
    arg_bytes: 0,
};
pub static INSTR_INC_L: Sm83Instr = Sm83Instr {
    mnemonic: "INC %l",
    op_code: 0x2C,
    arg_bytes: 0,
};
pub static INSTR_INC_BC: Sm83Instr = Sm83Instr {
    mnemonic: "INC %bc",
    op_code: 0x03,
    arg_bytes: 0,
};
pub static INSTR_INC_DE: Sm83Instr = Sm83Instr {
    mnemonic: "INC %de",
    op_code: 0x13,
    arg_bytes: 0,
};
pub static INSTR_INC_HL: Sm83Instr = Sm83Instr {
    mnemonic: "INC %hl",
    op_code: 0x23,
    arg_bytes: 0,
};

// DEC
pub static INSTR_DEC_A: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %a",
    op_code: 0x3D,
    arg_bytes: 0,
};
pub static INSTR_DEC_B: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %b",
    op_code: 0x05,
    arg_bytes: 0,
};
pub static INSTR_DEC_C: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %c",
    op_code: 0x0D,
    arg_bytes: 0,
};
pub static INSTR_DEC_BC: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %bc",
    op_code: 0x0B,
    arg_bytes: 0,
};
pub static INSTR_DEC_DE: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %de",
    op_code: 0x1B,
    arg_bytes: 0,
};
pub static INSTR_DEC_HL: Sm83Instr = Sm83Instr {
    mnemonic: "DEC %hl",
    op_code: 0x2B,
    arg_bytes: 0,
};
// LD
pub static INSTR_LD_TO_HL_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %hl",
    op_code: 0x21,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_DE_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %de",
    op_code: 0x11,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_BC_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %bc",
    op_code: 0x01,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_SP_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %sp",
    op_code: 0x31,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_A_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a",
    op_code: 0x3E,
    arg_bytes: 1,
};
pub static INSTR_LD_TO_A_FROM_DEREF_LABEL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a ('lbl)",
    op_code: 0xFA,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_B_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %b",
    op_code: 0x06,
    arg_bytes: 1,
};
pub static INSTR_LD_TO_C_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %c",
    op_code: 0x0E,
    arg_bytes: 1,
};
pub static INSTR_LD_TO_D_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %d",
    op_code: 0x16,
    arg_bytes: 1,
};
pub static INSTR_LD_TO_DEREF_HL_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl)",
    op_code: 0x36,
    arg_bytes: 1,
};
pub static INSTR_LD_TO_DEREF_DE_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%de) %a",
    op_code: 0x12,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_DEREF_HL_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl) %a",
    op_code: 0x77,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_DEREF_HL_DEC_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl -) %a",
    op_code: 0x32,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_DEREF_HL_INC_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD (%hl +) %a",
    op_code: 0x22,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_DEREF_LABEL_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD ('lbl) %a",
    op_code: 0xEA,
    arg_bytes: 2,
};
pub static INSTR_LD_TO_A_FROM_DEREF_HL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%hl)",
    op_code: 0x7E,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_D_FROM_DEREF_HL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %d (%hl)",
    op_code: 0x56,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_E_FROM_DEREF_HL: Sm83Instr = Sm83Instr {
    mnemonic: "LD %e (%hl)",
    op_code: 0x5E,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_A_FROM_DEREF_HL_INC: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%hl +)",
    op_code: 0x2A,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_A_FROM_DEREF_DE: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a (%de)",
    op_code: 0x1A,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_A_FROM_B: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a %b",
    op_code: 0x78,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_A_FROM_C: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a %c",
    op_code: 0x79,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_A_FROM_H: Sm83Instr = Sm83Instr {
    mnemonic: "LD %a %h",
    op_code: 0x7C,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_B_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD %b %a",
    op_code: 0x47,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_C_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD %c %a",
    op_code: 0x4F,
    arg_bytes: 0,
};
pub static INSTR_LD_TO_E_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LD %e %a",
    op_code: 0x5F,
    arg_bytes: 0,
};
// LDH
pub static INSTR_LDH_TO_IMMEDIATE_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LDH (0xFF00+n8) %a",
    op_code: 0xE0,
    arg_bytes: 1,
};
pub static INSTR_LDH_TO_A_FROM_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "LDH %a (0xFF00+n8)",
    op_code: 0xF0,
    arg_bytes: 1,
};
pub static INSTR_LDH_TO_DEREF_C_FROM_A: Sm83Instr = Sm83Instr {
    mnemonic: "LDH (%c) %a",
    op_code: 0xE2,
    arg_bytes: 0,
};
// CP
pub static INSTR_CP_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "CP",
    op_code: 0xFE,
    arg_bytes: 1,
};
// OR
pub static INSTR_OR_A_B: Sm83Instr = Sm83Instr {
    mnemonic: "OR %a %b",
    op_code: 0xB0,
    arg_bytes: 0,
};
pub static INSTR_OR_A_C: Sm83Instr = Sm83Instr {
    mnemonic: "OR %a %c",
    op_code: 0xB1,
    arg_bytes: 0,
};

// AND
pub static INSTR_AND_A_IMMEDIATE: Sm83Instr = Sm83Instr {
    mnemonic: "AND %a n8",
    op_code: 0xE6,
    arg_bytes: 1,
};
pub static INSTR_AND_A_A: Sm83Instr = Sm83Instr {
    mnemonic: "AND %a %a",
    op_code: 0xA7,
    arg_bytes: 0,
};
pub static INSTR_AND_A_C: Sm83Instr = Sm83Instr {
    mnemonic: "AND %a %c",
    op_code: 0xA1,
    arg_bytes: 0,
};

// XOR
pub static INSTR_XOR_A_A: Sm83Instr = Sm83Instr {
    mnemonic: "XOR %a %a",
    op_code: 0xAF,
    arg_bytes: 0,
};
pub static INSTR_XOR_A_C: Sm83Instr = Sm83Instr {
    mnemonic: "XOR %a %c",
    op_code: 0xA9,
    arg_bytes: 0,
};
// CPL
pub static INSTR_CPL: Sm83Instr = Sm83Instr {
    mnemonic: "CPL",
    op_code: 0x2F,
    arg_bytes: 0,
};
// ROTATE
pub static INSTR_RRCA: Sm83Instr = Sm83Instr {
    mnemonic: "RRCA",
    op_code: 0x0F,
    arg_bytes: 0,
};
// CALL
pub static INSTR_CALL: Sm83Instr = Sm83Instr {
    mnemonic: "CALL 'fn",
    op_code: 0xCD,
    arg_bytes: 2,
};
pub static INSTR_CALL_IF_C: Sm83Instr = Sm83Instr {
    mnemonic: "CALL #c 'fn",
    op_code: 0xDC,
    arg_bytes: 2,
};
pub static INSTR_CALL_IF_NZ: Sm83Instr = Sm83Instr {
    mnemonic: "CALL #nz 'fn",
    op_code: 0xC4,
    arg_bytes: 2,
};

// RET
pub static INSTR_RET: Sm83Instr = Sm83Instr {
    mnemonic: "RET",
    op_code: 0xC9,
    arg_bytes: 0,
};

// PUSH
pub static INSTR_PUSH_AF: Sm83Instr = Sm83Instr {
    mnemonic: "PUSH %af",
    op_code: 0xF5,
    arg_bytes: 0,
};
pub static INSTR_PUSH_BC: Sm83Instr = Sm83Instr {
    mnemonic: "PUSH %bc",
    op_code: 0xC5,
    arg_bytes: 0,
};
pub static INSTR_PUSH_DE: Sm83Instr = Sm83Instr {
    mnemonic: "PUSH %de",
    op_code: 0xD5,
    arg_bytes: 0,
};
pub static INSTR_PUSH_HL: Sm83Instr = Sm83Instr {
    mnemonic: "PUSH %hl",
    op_code: 0xE5,
    arg_bytes: 0,
};

// POP
pub static INSTR_POP_AF: Sm83Instr = Sm83Instr {
    mnemonic: "POP %af",
    op_code: 0xF1,
    arg_bytes: 0,
};
pub static INSTR_POP_BC: Sm83Instr = Sm83Instr {
    mnemonic: "POP %bc",
    op_code: 0xC1,
    arg_bytes: 0,
};
pub static INSTR_POP_DE: Sm83Instr = Sm83Instr {
    mnemonic: "POP %de",
    op_code: 0xD1,
    arg_bytes: 0,
};
pub static INSTR_POP_HL: Sm83Instr = Sm83Instr {
    mnemonic: "POP %hl",
    op_code: 0xE1,
    arg_bytes: 0,
};

// PREFIX / EXTENDED OP
pub static INSTR_PREFIX: Sm83Instr = Sm83Instr {
    mnemonic: "PREFIX",
    op_code: 0xCB,
    arg_bytes: 1,
};

pub static INSTR_PREFIX_SWAP_A: Sm83PrefixInstr = Sm83PrefixInstr {
    mnemonic: "SWAP %a",
    op_code: 0x37,
};

pub static INSTR_PREFIX_RST_0_A: Sm83PrefixInstr = Sm83PrefixInstr {
    mnemonic: "RST 0 %a",
    op_code: 0x87,
};

pub static INSTR_PREFIX_RST_7_DEREF_HL: Sm83PrefixInstr = Sm83PrefixInstr {
    mnemonic: "RST 7 (%hl)",
    op_code: 0xBE,
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
    /*0x0C*/ &INSTR_INC_C,
    /*0x0D*/ &INSTR_DEC_C,
    /*0x0E*/ &INSTR_LD_TO_C_FROM_IMMEDIATE,
    /*0x0F*/ &INSTR_RRCA,
    /*0x10*/ &INSTR_INVALID,
    /*0x11*/ &INSTR_LD_TO_DE_FROM_IMMEDIATE,
    /*0x12*/ &INSTR_LD_TO_DEREF_DE_FROM_A,
    /*0x13*/ &INSTR_INC_DE,
    /*0x14*/ &INSTR_INVALID,
    /*0x15*/ &INSTR_INVALID,
    /*0x16*/ &INSTR_LD_TO_D_FROM_IMMEDIATE,
    /*0x17*/ &INSTR_INVALID,
    /*0x18*/ &INSTR_JR,
    /*0x19*/ &INSTR_ADD_HL_DE,
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
    /*0x28*/ &INSTR_JR_IF_Z,
    /*0x29*/ &INSTR_INVALID,
    /*0x2A*/ &INSTR_LD_TO_A_FROM_DEREF_HL_INC,
    /*0x2B*/ &INSTR_DEC_HL,
    /*0x2C*/ &INSTR_INC_L,
    /*0x2D*/ &INSTR_INVALID,
    /*0x2E*/ &INSTR_INVALID,
    /*0x2F*/ &INSTR_CPL,
    /*0x30*/ &INSTR_INVALID,
    /*0x31*/ &INSTR_LD_TO_SP_FROM_IMMEDIATE,
    /*0x32*/ &INSTR_LD_TO_DEREF_HL_DEC_FROM_A,
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
    /*0x47*/ &INSTR_LD_TO_B_FROM_A,
    /*0x48*/ &INSTR_INVALID,
    /*0x49*/ &INSTR_INVALID,
    /*0x4A*/ &INSTR_INVALID,
    /*0x4B*/ &INSTR_INVALID,
    /*0x4C*/ &INSTR_INVALID,
    /*0x4D*/ &INSTR_INVALID,
    /*0x4E*/ &INSTR_INVALID,
    /*0x4F*/ &INSTR_LD_TO_C_FROM_A,
    /*0x50*/ &INSTR_INVALID,
    /*0x51*/ &INSTR_INVALID,
    /*0x52*/ &INSTR_INVALID,
    /*0x53*/ &INSTR_INVALID,
    /*0x54*/ &INSTR_INVALID,
    /*0x55*/ &INSTR_INVALID,
    /*0x56*/ &INSTR_LD_TO_D_FROM_DEREF_HL,
    /*0x57*/ &INSTR_INVALID,
    /*0x58*/ &INSTR_INVALID,
    /*0x59*/ &INSTR_INVALID,
    /*0x5A*/ &INSTR_INVALID,
    /*0x5B*/ &INSTR_INVALID,
    /*0x5C*/ &INSTR_INVALID,
    /*0x5D*/ &INSTR_INVALID,
    /*0x5E*/ &INSTR_LD_TO_E_FROM_DEREF_HL,
    /*0x5F*/ &INSTR_LD_TO_E_FROM_A,
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
    /*0x79*/ &INSTR_LD_TO_A_FROM_C,
    /*0x7A*/ &INSTR_INVALID,
    /*0x7B*/ &INSTR_INVALID,
    /*0x7C*/ &INSTR_LD_TO_A_FROM_H,
    /*0x7D*/ &INSTR_INVALID,
    /*0x7E*/ &INSTR_LD_TO_A_FROM_DEREF_HL,
    /*0x7F*/ &INSTR_INVALID,
    /*0x80*/ &INSTR_INVALID,
    /*0x81*/ &INSTR_INVALID,
    /*0x82*/ &INSTR_INVALID,
    /*0x83*/ &INSTR_INVALID,
    /*0x84*/ &INSTR_INVALID,
    /*0x85*/ &INSTR_INVALID,
    /*0x86*/ &INSTR_INVALID,
    /*0x87*/ &INSTR_ADD_A_A,
    /*0x88*/ &INSTR_INVALID,
    /*0x89*/ &INSTR_INVALID,
    /*0x8A*/ &INSTR_INVALID,
    /*0x8B*/ &INSTR_INVALID,
    /*0x8C*/ &INSTR_INVALID,
    /*0x8D*/ &INSTR_INVALID,
    /*0x8E*/ &INSTR_INVALID,
    /*0x8F*/ &INSTR_INVALID,
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
    /*0xA1*/ &INSTR_AND_A_C,
    /*0xA2*/ &INSTR_INVALID,
    /*0xA3*/ &INSTR_INVALID,
    /*0xA4*/ &INSTR_INVALID,
    /*0xA5*/ &INSTR_INVALID,
    /*0xA6*/ &INSTR_INVALID,
    /*0xA7*/ &INSTR_AND_A_A,
    /*0xA8*/ &INSTR_INVALID,
    /*0xA9*/ &INSTR_XOR_A_C,
    /*0xAA*/ &INSTR_INVALID,
    /*0xAB*/ &INSTR_INVALID,
    /*0xAC*/ &INSTR_INVALID,
    /*0xAD*/ &INSTR_INVALID,
    /*0xAE*/ &INSTR_INVALID,
    /*0xAF*/ &INSTR_XOR_A_A,
    /*0xB0*/ &INSTR_OR_A_B,
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
    /*0xC1*/ &INSTR_POP_BC,
    /*0xC2*/ &INSTR_JP_IF_NZ,
    /*0xC3*/ &INSTR_JP,
    /*0xC4*/ &INSTR_CALL_IF_NZ,
    /*0xC5*/ &INSTR_PUSH_BC,
    /*0xC6*/ &INSTR_INVALID,
    /*0xC7*/ &INSTR_INVALID,
    /*0xC8*/ &INSTR_INVALID,
    /*0xC9*/ &INSTR_RET,
    /*0xCA*/ &INSTR_JP_IF_Z,
    /*0xCB*/ &INSTR_PREFIX,
    /*0xCC*/ &INSTR_INVALID,
    /*0xCD*/ &INSTR_CALL,
    /*0xCE*/ &INSTR_INVALID,
    /*0xCF*/ &INSTR_INVALID,
    /*0xD0*/ &INSTR_INVALID,
    /*0xD1*/ &INSTR_POP_DE,
    /*0xD2*/ &INSTR_INVALID,
    /*0xD3*/ &INSTR_INVALID,
    /*0xD4*/ &INSTR_INVALID,
    /*0xD5*/ &INSTR_PUSH_DE,
    /*0xD6*/ &INSTR_INVALID,
    /*0xD7*/ &INSTR_INVALID,
    /*0xD8*/ &INSTR_INVALID,
    /*0xD9*/ &INSTR_INVALID,
    /*0xDA*/ &INSTR_JP_IF_C,
    /*0xDB*/ &INSTR_INVALID,
    /*0xDC*/ &INSTR_CALL_IF_C,
    /*0xDD*/ &INSTR_INVALID,
    /*0xDE*/ &INSTR_INVALID,
    /*0xDF*/ &INSTR_INVALID,
    /*0xE0*/ &INSTR_LDH_TO_IMMEDIATE_FROM_A,
    /*0xE1*/ &INSTR_POP_HL,
    /*0xE2*/ &INSTR_LDH_TO_DEREF_C_FROM_A,
    /*0xE3*/ &INSTR_INVALID,
    /*0xE4*/ &INSTR_INVALID,
    /*0xE5*/ &INSTR_PUSH_HL,
    /*0xE6*/ &INSTR_AND_A_IMMEDIATE,
    /*0xE7*/ &INSTR_INVALID,
    /*0xE8*/ &INSTR_ADD_SP_IMMEDIATE,
    /*0xE9*/ &INSTR_JP_HL,
    /*0xEA*/ &INSTR_LD_TO_DEREF_LABEL_FROM_A,
    /*0xEB*/ &INSTR_INVALID,
    /*0xEC*/ &INSTR_INVALID,
    /*0xED*/ &INSTR_INVALID,
    /*0xEE*/ &INSTR_INVALID,
    /*0xEF*/ &INSTR_RST_28,
    /*0xF0*/ &INSTR_LDH_TO_A_FROM_IMMEDIATE,
    /*0xF1*/ &INSTR_POP_AF,
    /*0xF2*/ &INSTR_INVALID,
    /*0xF3*/ &INSTR_DI,
    /*0xF4*/ &INSTR_INVALID,
    /*0xF5*/ &INSTR_PUSH_AF,
    /*0xF6*/ &INSTR_INVALID,
    /*0xF7*/ &INSTR_INVALID,
    /*0xF8*/ &INSTR_INVALID,
    /*0xF9*/ &INSTR_INVALID,
    /*0xFA*/ &INSTR_LD_TO_A_FROM_DEREF_LABEL,
    /*0xFB*/ &INSTR_EI,
    /*0xFC*/ &INSTR_INVALID,
    /*0xFD*/ &INSTR_INVALID,
    /*0xFE*/ &INSTR_CP_IMMEDIATE,
    /*0xFF*/ &INSTR_RST_38,
];

pub fn decode(op: u8) -> &'static Sm83Instr {
    INSTRUCTIONS[op as usize]
}
