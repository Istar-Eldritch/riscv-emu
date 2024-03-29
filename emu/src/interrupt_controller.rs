use crate::cpu::{CSRs, CPU};
use crate::instructions::Interrupt;
use crate::memory::DeviceMap;

pub struct InterruptController {
    peripherals: DeviceMap,
    interrupts: Vec<Interrupt>,
}

impl std::fmt::Debug for InterruptController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterruptController")
    }
}

impl InterruptController {
    pub fn new(peripherals: DeviceMap) -> Self {
        Self {
            peripherals,
            interrupts: vec![],
        }
    }

    /// Register a new interrupt. The interrupts will be accumulated until the end of the tick and
    /// disposed afterwards.
    pub fn interrupt(&mut self, interrupt: Interrupt, id: u32) {
        if interrupt == Interrupt::MExternalInterrupt {
            let peripherals = self.peripherals.borrow();
            let peripheral = peripherals
                .get("PLIC")
                .ok_or(())
                .and_then(|p| p.try_borrow_mut().map_err(|_| ()));

            if let Ok(mut peripheral) = peripheral {
                if let Some(plic) = peripheral.as_plic() {
                    if plic.h0mie & id as u64 != 0 {
                        let mut pending = plic.pending.borrow_mut();
                        *pending |= id as u64;
                        self.interrupts.push(interrupt);
                        return;
                    }
                }
            }
        }

        self.interrupts.push(interrupt)
    }

    fn highest_priority_interrupt(&self) -> Option<Interrupt> {
        let mut highest_priority = None;
        for &interrupt in self.interrupts.iter() {
            match interrupt {
                // Interrupt::NMI => highest_priority = Some(Interrupt::NMI),
                Interrupt::MExternalInterrupt => return Some(interrupt),
                Interrupt::SExternalInterrupt => highest_priority = Some(interrupt),
                Interrupt::MSoftInterrupt => {
                    if highest_priority != Some(Interrupt::SExternalInterrupt) {
                        highest_priority = Some(interrupt);
                    }
                }
                Interrupt::SSoftInterrupt => {
                    if highest_priority != Some(Interrupt::SExternalInterrupt)
                        && highest_priority != Some(Interrupt::MSoftInterrupt)
                    {
                        highest_priority = Some(interrupt);
                    }
                }
                Interrupt::MTimerInterrupt => {
                    if highest_priority != Some(Interrupt::SExternalInterrupt)
                        && highest_priority != Some(Interrupt::MSoftInterrupt)
                        && highest_priority != Some(Interrupt::SSoftInterrupt)
                    {
                        highest_priority = Some(interrupt);
                    }
                }
                Interrupt::STimerInterrupt => {
                    if highest_priority != Some(Interrupt::SExternalInterrupt)
                        && highest_priority != Some(Interrupt::MSoftInterrupt)
                        && highest_priority != Some(Interrupt::SSoftInterrupt)
                        && highest_priority != Some(Interrupt::MTimerInterrupt)
                    {
                        highest_priority = Some(interrupt);
                    }
                }
            }
        }
        highest_priority
    }

    pub fn notify_cpu(&mut self, cpu: &mut CPU) {
        if let Some(interrupt) = self.highest_priority_interrupt() {
            let mip = cpu.get_csr(CSRs::mip as u32).unwrap();
            let mip_mei = mip | (1 << interrupt as u32);
            cpu.set_csr(CSRs::mip as u32, mip_mei).unwrap();
        }
    }

    pub fn reset(&mut self, cpu: &mut CPU) {
        self.interrupts = vec![];

        let peripherals = self.peripherals.borrow();
        let peripheral = peripherals
            .get("PLIC")
            .ok_or(())
            .and_then(|p| p.try_borrow_mut().map_err(|_| ()));
        if let Ok(mut peripheral) = peripheral {
            if let Some(plic) = peripheral.as_plic() {
                let mut pending = plic.pending.borrow_mut();
                *pending = 0;
            }
        }

        cpu.set_csr(CSRs::mip as u32, 0).unwrap();
    }
}
