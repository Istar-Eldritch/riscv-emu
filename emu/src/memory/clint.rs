use super::Memory;
use super::MemoryError;
use crate::memory::Clocked;
use crate::memory::Device;

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
}

impl<'a> TryFrom<&'a Device> for &'a CLINT {
    type Error = ();
    fn try_from(device: &Device) -> Result<&CLINT, Self::Error> {
        match device {
            Device::CLINT(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Device> for &'a mut CLINT {
    type Error = ();
    fn try_from(device: &mut Device) -> Result<&mut CLINT, Self::Error> {
        match device {
            Device::CLINT(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl Clocked<()> for CLINT {
    fn tick(&mut self, _: ()) -> () {
        self.mtime += 1
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
