use crate::mcu::{DeviceDef, TickResult, MCU};
use crate::memory::uart::UARTDevice;

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
        let mcu = MCU::new();
        Emulator {
            mcu,
            speed: opts.speed,
        }
    }

    pub fn setup_devices(&mut self, devices: Vec<DeviceDef>) -> Result<(), ()> {
        for device in devices.into_iter() {
            self.mcu.add_device(device)?;
        }

        Ok(())
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
