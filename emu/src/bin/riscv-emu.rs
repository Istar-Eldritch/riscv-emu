use clap::{command, Parser};
use riscv_emu::Emulator;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};

/// A ruscv emulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the binary to flash
    flash: String,
    #[arg(short, long)]
    dump: Option<String>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = fs::File::open(args.flash)?;
    let mut br = BufReader::new(file);
    let mut mem: Vec<u8> = Vec::new();
    br.read_to_end(&mut mem)?;
    println!("loaded flash");
    let mut emu = Emulator::new(1024 * 100, 1);
    emu.flash(mem);
    emu.run_program()?;
    if let Some(path) = args.dump {
        let path = std::path::Path::new(&path);
        if path.exists() {
            fs::remove_file(path)?;
        }

        let file = fs::File::create(path)?;

        let mut bw = BufWriter::new(file);
        bw.write_all(&emu.dump())?;
    }
    Ok(())
}
