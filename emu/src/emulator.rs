use crate::cpu::CPU;
use crate::mcu::{TickResult, MCU};
use crate::memory::{uart::UARTDevice, MMU};

pub struct Emulator {
    mcu: MCU,
    speed: u32, // speed in hz
}

fn wait_cycles(hz: u32, cycles: u32) {
    std::thread::sleep(std::time::Duration::from_nanos(
        (1e9 / hz as f64).round() as u64 * cycles as u64,
    ))
}

pub struct EmulatorOpts {
    pub speed: u32,
    pub terminal: Option<Box<dyn UARTDevice>>,
    pub dump_path: std::path::PathBuf,
}

impl Emulator {
    pub fn new(opts: EmulatorOpts) -> Self {
        Emulator {
            mcu: MCU::new(CPU::new(), Box::new(MMU::new(opts.terminal))),
            speed: opts.speed,
        }
    }

    pub fn run_program(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut pending_work = 0;
        let mut cycles = 0;
        loop {
            if pending_work > 0 {
                pending_work -= 1;
                wait_cycles(self.speed, 1);
            } else {
                match self.mcu.tick() {
                    TickResult::Cycles(v) => {
                        log::trace!("cycle: {}", cycles);
                        cycles += v;
                        pending_work += v
                    }
                    TickResult::WFI => pending_work += 1, // TODO: This should actually block on a callback  instead of doing polling

                    // TODO: This two results should be done using the AON
                    TickResult::HALT => return Ok(()),
                    TickResult::Dump(_range) => {}
                }
            }
        }
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        self.mcu.flash(mem)
    }
}
