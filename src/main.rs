use clap::{ArgAction, Args, Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Subcommand)]
enum MainCommands {
    Disassemble(Disassemble),
    Assemble(Assemble),
    Link(Link),
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

#[derive(Args)]
struct Assemble {
    /// The input file to assemble
    file: String,
    /// Will assemble a 'flat' binary. All sections have to be defined for this
    /// and the result is a direct binary output, not an object file that can be linked.
    #[clap(long, short, action=ArgAction::SetTrue)]
    flat: bool,
    #[clap(long, short, default_value = "a.out")]
    out: String,
}

#[derive(Args)]
struct Link {
    #[command(subcommand)]
    command: LinkSubCommands,
}

#[derive(Subcommand)]
enum LinkSubCommands {
    GB(LinkGB),
}

#[derive(Args)]
struct LinkGB {
    /// The input files to link
    file: Vec<String>,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    match &args.command {
        MainCommands::Disassemble(cmd) => match &cmd.command {
            DisassembleSubCommands::GB(dis_gb_arg) => disassemble_gb(&dis_gb_arg),
        },
        MainCommands::Assemble(asm) => assemble(asm),
        MainCommands::Link(cmd) => match &cmd.command {
            LinkSubCommands::GB(link_gb_arg) => link_gb(&link_gb_arg),
        },
    }
}

fn assemble(arg: &Assemble) -> Result<(), String> {
    if !arg.flat {
        return Err("currently only flat mode assemble is supported".to_string());
    }

    let mut file = File::open(&arg.file).map_err(|e| e.to_string())?;
    let options = psy::asm::assembler::Options {
        flat: arg.flat,
        out: PathBuf::from_str(&arg.out).unwrap(),
    };
    psy::asm::assemble_file(&mut file, options)
}

fn disassemble_gb(arg: &DisassembleGB) -> Result<(), String> {
    let data = read_all_from_file(&arg.file)?;
    let instructions = psy::dasm::gb::disassemble(&data)?;
    for instruction in &instructions {
        println!("{}", instruction)
    }
    Ok(())
}

fn link_gb(arg: &LinkGB) -> Result<(), String> {
    // TODO
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
