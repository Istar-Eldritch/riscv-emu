use clap::{command, Parser};
use riscv_emu::{Emulator, EmulatorOpts, TermEmulator};
use std::fs;
use std::io::{BufReader, Read};

/// A ruscv emulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the binary to flash
    flash: String,
    #[arg(short, long)]
    dump_folder: Option<String>,
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
    let mut term = TermEmulator::new();
    term.lock();
    let opts = EmulatorOpts {
        speed: args.speed.unwrap_or(1),
        terminal: Some(Box::new(term)),
        dump_path: std::path::PathBuf::from(
            args.dump_folder.unwrap_or(
                std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            ),
        ),
    };
    let mut emu = Emulator::new(opts);
    emu.flash(mem);
    emu.run_program()?;
    log::info!("Program ended execution");
    Ok(())
}
