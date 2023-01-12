use super::{Clocked, ClockedMemory, DeviceMap, Memory, MemoryError};

#[derive(Debug)]
pub struct DeviceMeta {
    mem_start: u32,
    mem_end: u32,
    identifier: String,
}

impl DeviceMeta {
    pub fn new(identifier: String, mem_start: u32, mem_end: u32) -> Self {
        Self {
            mem_start,
            mem_end,
            identifier,
        }
    }
}

pub struct MMU {
    device_idx: Vec<DeviceMeta>,
    devices: DeviceMap,
}

impl MMU {
    pub fn new(devices: DeviceMap) -> Self {
        MMU {
            device_idx: Vec::new(),
            devices,
        }
    }
}

impl Clocked<()> for MMU {
    fn tick(&mut self, _: ()) {}
}

impl ClockedMemory for MMU {
    fn as_mem(&self) -> &dyn Memory {
        self
    }
    fn as_mut_mem(&mut self) -> &mut dyn Memory {
        self
    }
}

impl MMU {
    fn find_device_index(&self, addr: u32) -> Result<usize, usize> {
        self.device_idx.binary_search_by(|d| {
            if addr < d.mem_start {
                std::cmp::Ordering::Less
            } else if addr > d.mem_end {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    }
    fn find_device_meta(&self, addr: u32) -> Option<&DeviceMeta> {
        let search_result = self.find_device_index(addr);

        if let Ok(idx) = search_result {
            Some(&self.device_idx.get(idx).unwrap())
        } else {
            None
        }
    }

    /// Returns an error if the device overlaps memory with another
    pub fn insert_device(&mut self, meta: DeviceMeta) -> Result<(), ()> {
        //let search_result = self.find_device_index(meta.mem_start);

        let search_result = self.device_idx.binary_search_by(|d| {
            if meta.mem_start > d.mem_end {
                std::cmp::Ordering::Greater
            } else if meta.mem_end < d.mem_start {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if let Err(idx) = search_result {
            self.device_idx.insert(idx, meta);
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Memory for MMU {
    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let device = devices.get(&meta.identifier).unwrap().borrow();
            device.rb(addr - meta.mem_start)
        } else {
            Err(MemoryError::AccessFault)
        }
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let mut device = devices.get(&meta.identifier).unwrap().borrow_mut();
            device.wb(addr - meta.mem_start, value)
        } else {
            Err(MemoryError::AccessFault)
        }
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let device = devices.get(&meta.identifier).unwrap().borrow();
            device.rhw(addr - meta.mem_start)
        } else {
            Err(MemoryError::AccessFault)
        }
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let mut device = devices.get(&meta.identifier).unwrap().borrow_mut();
            device.whw(addr - meta.mem_start, value)
        } else {
            Err(MemoryError::AccessFault)
        }
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let device = devices.get(&meta.identifier).unwrap().borrow();
            device.rw(addr - meta.mem_start)
        } else {
            Err(MemoryError::AccessFault)
        }
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        if let Some(meta) = self.find_device_meta(addr) {
            let devices = self.devices.borrow();
            let mut device = devices.get(&meta.identifier).unwrap().borrow_mut();
            device.ww(addr - meta.mem_start, value)
        } else {
            Err(MemoryError::AccessFault)
        }
    }
}
