use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Subcommand)]
enum MainCommands {
    Disassemble(Disassemble),
}

#[derive(Args)]
struct Disassemble {
    #[command(subcommand)]
    command: DisassembleSubCommands,
}

#[derive(Subcommand)]
enum DisassembleSubCommands {
    GB(DisassembleGB),
}

#[derive(Args)]
struct DisassembleGB {
    /// The input file to disassemble, provide '-' to read from standard input
    file: String,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    match &args.command {
        MainCommands::Disassemble(cmd) => match &cmd.command {
            DisassembleSubCommands::GB(dis_gb) => disassemble_gb(&dis_gb),
        },
    }
}

fn disassemble_gb(arg: &DisassembleGB) -> Result<(), String> {
    let data = read_all_from_file(&arg.file)?;
    let instructions = psy::dasm::gb::disassemble(&data)?;
    for instruction in &instructions {
        println!("{}", instruction)
    }
    Ok(())
}

fn read_all_from_file(file_def: &str) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    if file_def == "-" {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    } else {
        let mut file = File::open(file_def).map_err(|e| e.to_string())?;
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    }
    Ok(buf)
}
