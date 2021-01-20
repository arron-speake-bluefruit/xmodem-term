use serial::{BaudRate, CharSize, FlowControl, Parity, SerialPort, StopBits, SystemPort};
use std::path::Path;

pub fn setup_device(path: &str) -> std::result::Result<SystemPort, String> {
    let mut device =
        SystemPort::open(Path::new(path)).map_err(|e| format!("Failed to open device: {}.", e))?;

    device
        .reconfigure(&|s| {
            s.set_baud_rate(BaudRate::Baud115200)?;
            s.set_char_size(CharSize::Bits8);
            s.set_parity(Parity::ParityNone);
            s.set_stop_bits(StopBits::Stop1);
            s.set_flow_control(FlowControl::FlowNone);
            Ok(())
        })
        .map_err(|e| format!("Failed to configure device: {}.", e))?;

    Ok(device)
}
