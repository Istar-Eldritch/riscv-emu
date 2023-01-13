use riscv_emu::MCU;

#[no_mangle]
pub unsafe extern "C" fn new_mcu() -> *mut MCU {
    let mut mcu = MCU::new();
    let mcu_ptr: *mut MCU = &mut mcu;
    std::mem::forget(mcu);
    mcu_ptr
}

#[repr(C)]
#[derive(Default)]
pub struct TickResult {
    code: i8,
    dump_range_from: u32,
    dump_range_to: u32,
    cycles: u32,
}

#[no_mangle]
pub unsafe extern "C" fn tick(mcu: *mut MCU) -> TickResult {
    use riscv_emu::TickResult::*;
    let mut mcu = mcu.read();
    match mcu.tick() {
        HALT => TickResult {
            code: 0,
            ..TickResult::default()
        },
        WFI => TickResult {
            code: 1,
            ..TickResult::default()
        },
        Dump(range) => TickResult {
            code: 2,
            dump_range_from: *range.start(),
            dump_range_to: *range.end(),
            ..TickResult::default()
        },
        Cycles(n) => TickResult {
            code: 3,
            cycles: n,
            ..TickResult::default()
        },
    }
}

#[repr(C)]
pub enum DeviceType {
    PLIC,
    CLINT,
    UART(ExternUART),
    FLASH(u32),
}

#[repr(C)]
pub struct ExternUART {
    read: extern "C" fn() -> u8,
    write: extern "C" fn(u8),
}

impl riscv_emu::UARTDevice for ExternUART {
    fn read(&mut self) -> Option<u8> {
        let r = (self.read)();
        if r != 0 {
            Some(r)
        } else {
            None
        }
    }
    fn write(&mut self, byte: u8) {
        (self.write)(byte)
    }
}

impl From<DeviceType> for riscv_emu::Device {
    fn from(detype: DeviceType) -> riscv_emu::Device {
        match detype {
            DeviceType::PLIC => riscv_emu::Device::PLIC(riscv_emu::PLIC::new()),
            DeviceType::CLINT => riscv_emu::Device::CLINT(riscv_emu::CLINT::new()),
            DeviceType::UART(extern_uart) => {
                let extern_uart: Option<Box<dyn riscv_emu::UARTDevice>> =
                    Some(Box::new(extern_uart));
                riscv_emu::Device::UART(riscv_emu::UART::new(extern_uart))
            }
            DeviceType::FLASH(size) => {
                riscv_emu::Device::FLASH(riscv_emu::GenericMemory::new(size))
            }
        }
    }
}

#[repr(C)]
pub struct DeviceDef {
    identifier: *mut i8,
    memory_start: u32,
    memory_end: u32,
    device: DeviceType,
}

impl From<DeviceDef> for riscv_emu::DeviceDef {
    fn from(device: DeviceDef) -> riscv_emu::DeviceDef {
        let identifier = (unsafe { std::ffi::CString::from_raw(device.identifier) })
            .to_string_lossy()
            .to_string();

        riscv_emu::DeviceDef {
            identifier,
            memory_start: device.memory_start,
            memory_end: device.memory_end,
            device: device.device.into(),
        }
    }
}

/// Returns 0 if its ok, 1 if there are overlapping memory regions with other devices
#[no_mangle]
pub unsafe extern "C" fn add_device(mcu: *mut MCU, device: DeviceDef) -> u32 {
    let mut mcu = mcu.read();

    if let Err(()) = mcu.add_device(device.into()) {
        1
    } else {
        0
    }
}
