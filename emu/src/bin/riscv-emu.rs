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
    #[arg(short, long)]
    speed: Option<u32>,
    #[arg(short, long)]
    log: Option<String>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut builder = env_logger::Builder::new();
    builder.parse_filters(&args.log.unwrap_or("info".into()));
    builder.init();
    let file = fs::File::open(args.flash)?;
    let mut br = BufReader::new(file);
    let mut mem: Vec<u8> = Vec::new();
    br.read_to_end(&mut mem)?;

    log::info!("Flash memory loaded");
    let mut emu = Emulator::new(args.speed.unwrap_or(1));
    emu.flash(mem);
    emu.run_program()?;
    log::info!("Program ended execution");
    if let Some(path_str) = args.dump {
        let path = std::path::Path::new(&path_str);
        if path.exists() {
            fs::remove_file(path)?;
        }

        let file = fs::File::create(path)?;

        let mut bw = BufWriter::new(file);
        bw.write_all(&emu.dump())?;
        log::info!("Resulting machine state dumped to {path_str}");
    }
    Ok(())
}
