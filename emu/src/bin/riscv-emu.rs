use clap::{command, Parser};
use riscv_emu::emulator::{Emulator, EmulatorOpts};
use riscv_emu::mcu::DeviceDef;
use riscv_emu::memory::GenericMemory;
use riscv_emu::peripherals::{clint::CLINT, plic::PLIC, uart::UART, Peripheral};
use riscv_emu::terminal::TermEmulator;
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
    let file = fs::File::open(args.flash).map_err(|err| {
        log::error!("Error geting flash image: {}", err);
        err
    })?;
    let mut br = BufReader::new(file);
    let mut mem: Vec<u8> = Vec::new();
    br.read_to_end(&mut mem)?;

    log::info!("Flash memory loaded");
    let mut term = TermEmulator::new();
    term.lock();
    let opts = EmulatorOpts {
        speed: args.speed.unwrap_or(1),
        terminal: None,
        dump_path: std::path::PathBuf::from(
            args.dump_folder.unwrap_or(
                std::env::current_dir()
                    .unwrap_or_else(|err| {
                        log::error!("Error getting hold of the current directory");
                        panic!("{}", err)
                    })
                    .to_string_lossy()
                    .to_string(),
            ),
        ),
    };

    let mut uart_device = UART::new(Some(Box::new(term)));
    uart_device.set_interrupt_id(0b10000);

    let devices = vec![
        DeviceDef {
            identifier: "FLASH".to_string(),
            memory_start: 0,
            memory_end: 0x3_2000,
            device: Peripheral::FLASH(GenericMemory::new(0x3_2000)),
        },
        DeviceDef {
            identifier: "CLINT".to_string(),
            memory_start: 0x200_0000,
            memory_end: 0x200_FFFF,
            device: Peripheral::CLINT(CLINT::new()),
        },
        DeviceDef {
            identifier: "PLIC".to_string(),
            memory_start: 0x0C00_0000,
            memory_end: 0x1000_0000,
            device: Peripheral::PLIC(PLIC::new()),
        },
        DeviceDef {
            identifier: "UART0".to_string(),
            memory_start: 0x1001_3000,
            memory_end: 0x1001_3FFF,
            device: Peripheral::UART(uart_device),
        },
    ];

    let mut emu = Emulator::new(opts);
    emu.setup_devices(devices).unwrap();
    emu.flash(mem);
    emu.run_program()?;
    log::info!("Program ended execution");
    Ok(())
}
