use std::{
    io::{Read, Write},
    fs::File,
    thread::sleep,
    time::{Instant, Duration},
};

use crate::{
    packet::Packet,
    xmodem_file_adapter::XModemFileAdapter,
};

pub struct XModem<F>
where
    F: Read + Write,
{
    file: F,
}

impl<F: Read + Write> XModem<F> {
    pub fn new(file: F) -> Self {
        Self { file }
    }

    pub fn send(mut self, file: File) -> Option<()> {
        self.wait_for_negative_acknowledge()?;
        for packet in XModemFileAdapter::new(file) {
            const MAX_ATTEMPTS : usize = 10;
            let mut exceeded_max_attempts = true;
            'repeat_attempts: for _ in 0..MAX_ATTEMPTS {
                self.write(&packet)?;
                let acknowledged = self.wait_for_response()?;
                if acknowledged {
                    exceeded_max_attempts = false;
                    break 'repeat_attempts;
                }
            }
            if exceeded_max_attempts { return None; }
        }

        Some(())
    }

    fn write(&mut self, packet: &Packet) -> Option<()> {
        self.file.write_all(packet.data()).ok()
    }

    fn read(&mut self) -> Option<bool> {
        const ACKNOWLEDGE : u8 = 0x06;
        const NEGATIVE_ACKNOWLEDGE : u8 = 0x15;
        let mut read_buffer = [0u8; 1];
        self.file.read_exact(&mut read_buffer).ok()?;
        match read_buffer[0] {
            ACKNOWLEDGE => Some(true),
            NEGATIVE_ACKNOWLEDGE => Some(false),
            _ => None,
        }
    }

    fn wait_for_response(&mut self) -> Option<bool> {
        const TIMEOUT : Duration = Duration::from_secs(10);
        const DELAY : Duration = Duration::from_millis(500);
        let timeout_point = Instant::now() + TIMEOUT;

        while Instant::now() < timeout_point {
            let read = self.read();
            if read.is_some() { return read; }
            sleep(DELAY);
        }

        None
    }

    fn wait_for_negative_acknowledge(&mut self) -> Option<()> {
        match self.wait_for_response()? {
            false => Some(()),
            true => None,
        }
    }
}
