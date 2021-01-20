use std::{
    fs::File,
    io::{Read, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{packet::Packet, xmodem_file_adapter::XModemFileAdapter};

pub struct XModem {
    port: serial::SystemPort,
}

impl XModem {
    pub fn new(port: serial::SystemPort) -> Self {
        Self { port }
    }

    pub fn send(mut self, file: File) -> Option<()> {
        self.wait_for_negative_acknowledge()?;
        'a: for packet in XModemFileAdapter::new(file) {
            print!("Sending packet");
            const MAX_ATTEMPTS: usize = 10;
            for _ in 0..MAX_ATTEMPTS {
                print!(".");
                self.write(&packet)?;
                let acknowledged = self.wait_for_response()?;
                if acknowledged {
                    println!("Done");
                    continue 'a;
                }
            }
            println!("Failed");
        }

        Some(())
    }

    fn write(&mut self, packet: &Packet) -> Option<()> {
        self.port
            .write_all(packet.data())
            .map_err(|e| {
                println!("Write failed: {}", e);
                e
            })
            .ok()
    }

    fn read(&mut self) -> Option<bool> {
        const ACKNOWLEDGE: u8 = 0x06;
        const NEGATIVE_ACKNOWLEDGE: u8 = 0x15;
        let mut read_buffer = [0u8; 1];
        self.port
            .read_exact(&mut read_buffer)
            .map_err(|e| {
                println!("Read failed: {}", e);
                e
            })
            .ok()?;
        match read_buffer[0] {
            ACKNOWLEDGE => Some(true),
            NEGATIVE_ACKNOWLEDGE => Some(false),
            _ => None,
        }
    }

    fn wait_for_response(&mut self) -> Option<bool> {
        const TIMEOUT: Duration = Duration::from_secs(10);
        const DELAY: Duration = Duration::from_millis(500);
        let timeout_point = Instant::now() + TIMEOUT;

        while Instant::now() < timeout_point {
            let read = self.read();
            if read.is_some() {
                return read;
            }
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
