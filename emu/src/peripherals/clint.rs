use crate::cpu::{CSRs, CPU};
use crate::instructions::Interrupt;
use crate::memory::Clocked;
use crate::memory::{Memory, MemoryError};
use crate::peripherals::{Peripheral, RegisterInterrupt};

pub struct CLINT {
    pub msip0: u32,    // addr 0
    pub mtimecmp: u64, // addr 4
    pub mtime: u64,    // addr 12
}

impl CLINT {
    pub fn new() -> Self {
        Self {
            msip0: 0,
            mtimecmp: 0,
            mtime: 0,
        }
    }

    /// Software interrupt pending check
    fn update_mip_msip(&mut self, cpu: &mut CPU) {
        let software_interrupt = self.msip0;

        let mip = cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_msi = if software_interrupt > 0 {
            self.msip0 = 0;
            mip | (1 << Interrupt::MSoftInterrupt as u32)
        } else {
            mip & !(1 << Interrupt::MSoftInterrupt as u32)
        };

        cpu.set_csr(CSRs::mip as u32, mip_msi).unwrap();
    }

    /// Timer interrupt pending check
    fn update_mip_mtip(&mut self, cpu: &mut CPU) {
        use Interrupt::*;

        let cmp_time = self.mtimecmp;

        let time = self.mtime;

        let mip = cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_mti = if cmp_time != 0 && time >= cmp_time {
            mip | (1 << MTimerInterrupt as u32)
        } else {
            mip & !(1 << MTimerInterrupt as u32)
        };
        cpu.set_csr(CSRs::mip as u32, mip_mti).unwrap();
    }
}

impl Peripheral for CLINT {}

impl Clocked<RegisterInterrupt> for CLINT {
    /// Increases time, generates timer & software interrupts
    fn tick(&mut self, register_interrupt: RegisterInterrupt) -> () {
        self.mtime += 1;

        // TODO: Register interrupts
        // Generate timer & software interrupts
        // let mstatus = cpu.get_csr(CSRs::mstatus as u32).unwrap();
        // let mstatus_mie = (mstatus & (1 << 3)) != 0;
        // if mstatus_mie {
        //     self.update_mip_mtip(cpu);
        //     self.update_mip_msip(cpu);
        // }
    }
}

impl Memory for CLINT {
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
            0 => Ok(self.msip0),
            0x4000 => Ok(self.mtimecmp as u32),
            0x4004 => Ok((self.mtimecmp >> 32) as u32),
            0xbff8 => Ok(self.mtime as u32),
            0xbffc => Ok((self.mtime >> 32) as u32),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        match addr {
            0 => {
                self.msip0 = value;
                Ok(())
            }
            0x4000 => {
                self.mtimecmp = (self.mtimecmp & !0xffff_ffff) | value as u64;
                Ok(())
            }
            0x4004 => {
                self.mtimecmp = (self.mtimecmp & !(0xffff_ffff << 32)) | ((value as u64) << 32);
                Ok(())
            }
            0xbff8 => Ok(()),
            0xbffc => Ok(()),
            v if v == 0xbff8 || v == (0xbff8 + 4) => Ok(()),
            _ => Err(MemoryError::AccessFault),
        }
    }
}
