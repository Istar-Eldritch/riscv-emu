use crate::peripherals::uart::UARTDevice;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct TermEmulator {
    input_buffer: Arc<Mutex<VecDeque<u8>>>,
    handle: Option<JoinHandle<()>>,
}

impl TermEmulator {
    pub fn new() -> Self {
        Self {
            input_buffer: Arc::new(Mutex::new(VecDeque::new())),
            handle: None,
        }
    }

    pub fn lock(&mut self) {
        let buffer = Arc::clone(&self.input_buffer);
        let handle = std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let mut reader = stdin.lock();
            let mut buff = [0; 2];

            loop {
                match reader.read(&mut buff) {
                    Ok(n) => {
                        let mut buffer = buffer.lock().unwrap();
                        for i in 0..n {
                            buffer.push_front(buff[i]);
                        }
                    }
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
        });
        self.handle = Some(handle);
    }
}

impl UARTDevice for TermEmulator {
    fn read(&mut self) -> Option<u8> {
        self.input_buffer.lock().unwrap().pop_back()
    }

    fn write(&mut self, v: u8) {
        print!("{}", v as char);
        std::io::stdout().flush().unwrap();
    }
}
