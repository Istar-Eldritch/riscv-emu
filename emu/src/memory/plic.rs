use super::{Device, DeviceMap, Memory, MemoryError};
use crate::memory::Clocked;
use std::cell::RefCell;

pub struct PLIC {
    pub source_priority: [u32; 52], // addr 0x0
    pub pending: RefCell<u64>,      // addr 0xd0
    pub h0mie: u64,                 // hart0 M-Mode interrupt enables - addr 216
    pub h0mpt: u32,                 // hart0 M-Mode priority threshold - addr 224
                                    // hart0 M-Mode claim/complete - addr 228
}

impl PLIC {
    pub fn new() -> Self {
        Self {
            source_priority: [0; 52],
            pending: RefCell::new(0),
            h0mie: 0,
            h0mpt: 0,
        }
    }

    pub fn claim_interrupt(&self) -> u32 {
        let mut interrupts = self.get_interrupts();
        let interrupt = interrupts.pop().unwrap_or(0);
        let mut pending = self.pending.borrow_mut();
        *pending = *pending & !(1 << interrupt);
        interrupt
    }

    // Returns the enabled and pending interrupts ordered by priority
    fn get_interrupts(&self) -> Vec<u32> {
        let mut interrupts = Vec::new();
        for i in 0..52 {
            let code = 1 << i;
            let priority = self.source_priority[i as usize];
            if self.h0mie & code != 0
                && *self.pending.borrow() & code != 0
                && priority >= self.h0mpt
            {
                interrupts.push(i);
            }
        }
        interrupts.sort_by(|a, b| {
            let pa = self.source_priority[*a as usize];
            let pb = self.source_priority[*b as usize];
            pa.cmp(&pb)
        });
        interrupts
    }
}

impl<'a> TryFrom<&'a Device> for &'a PLIC {
    type Error = ();
    fn try_from(device: &Device) -> Result<&PLIC, Self::Error> {
        match device {
            Device::PLIC(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl Clocked<&DeviceMap> for PLIC {
    // This updates the external interrupts based on the device order. if UART0 is added 3rd then
    // 0b1000 will denote a pending interrupt on uart0
    fn tick(&mut self, devices: &DeviceMap) {
        let devices = devices.borrow();

        let mut pending = self.pending.borrow_mut();
        for (idx, (_k, device)) in devices.iter().enumerate() {
            match device {
                Device::UART(u) => {
                    let uart_ip = u.get_ip();
                    let mask = 1 << idx + 1;
                    if uart_ip != 0 {
                        *pending |= mask as u64;
                    } else {
                        *pending &= !(mask as u64);
                    };
                }
                _ => {}
            }
        }
    }
}

impl Memory for PLIC {
    fn rb(&self, _addr: u32) -> Result<u8, MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn wb(&mut self, _addr: u32, _value: u8) -> Result<(), MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn rhw(&self, _addr: u32) -> Result<u16, MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn whw(&mut self, _addr: u32, _value: u16) -> Result<(), MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        match addr {
            v if v >= 0x4 && v < 0xd4 => Ok(self.source_priority[((v - 4) / 4) as usize]),
            0x1000 => Ok(*self.pending.borrow() as u32),
            0x1004 => Ok((*self.pending.borrow() >> 32) as u32),
            0x2000 => Ok(self.h0mie as u32),
            0x2004 => Ok((self.h0mie >> 32) as u32),
            0x20_0000 => Ok(self.h0mpt),
            0x20_0004 => {
                // XXX: Should we keep a list of claimed interrupts?
                let interrupt = self.claim_interrupt();
                Ok(interrupt)
            }
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        match addr {
            v if v >= 0x4 && v < 0xd4 => {
                self.source_priority[((v - 4) / 4) as usize] = value;
                Ok(())
            }
            v if v == 0x1000 || v == 0x1004 => {
                // pending is not writable
                Ok(())
            }
            0x2000 => {
                self.h0mie = (self.h0mie & !0xffff_ffff) | value as u64;
                Ok(())
            }
            0x2004 => {
                self.h0mie = (self.h0mie & !(0xffff_ffff << 32)) | (value as u64) << 32;
                Ok(())
            }
            0x20_0000 => {
                self.h0mpt = value;
                Ok(())
            }
            0x20_0004 => {
                // XXX: Not sure what to do here. Should we keep a list of the claimed interrupts?
                Ok(())
            }
            _ => Err(MemoryError::AccessFault),
        }
    }
}
