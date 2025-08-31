use crate::arch::sm83;

pub fn disassemble(data: &[u8]) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut ip = 0;
    while ip < data.len() {
        let instr = sm83::decode(data[ip])?;
        ip += 1;
        let mut str = String::new();

        str.push_str(&format!("({:?}", instr.mnemonic));
        for arg in &instr.immediate_args {
            str.push(' ');
            str.push_str(arg);
        }

        if instr.stream_args == 0 {
            str.push(')')
        } else if instr.stream_args == 1 {
            str.push_str(&format!(" {:x})", data[ip]));
        } else if instr.stream_args == 2 {
            let a16 = u16::from_le_bytes([data[ip], data[ip + 1]]);
            str.push_str(&format!(" {:x})", a16));
        }
        ip += instr.stream_args;

        result.push(str);
    }
    return Ok(result);
}
