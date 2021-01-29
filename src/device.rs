use serial::{BaudRate, CharSize, FlowControl, Parity, SerialPort, StopBits, SystemPort};
use std::path::Path;

pub fn setup_device(
    path: &str,
    baud_rate: BaudRate,
    char_size: CharSize,
    parity: Parity,
    stop_bits: StopBits,
    flow_control: FlowControl,
) -> std::result::Result<SystemPort, String> {
    let mut device =
        SystemPort::open(Path::new(path)).map_err(|e| format!("Failed to open device: {}.", e))?;

    device
        .reconfigure(&|s| {
            s.set_baud_rate(baud_rate)?;
            s.set_char_size(char_size);
            s.set_parity(parity);
            s.set_stop_bits(stop_bits);
            s.set_flow_control(flow_control);
            Ok(())
        })
        .map_err(|e| format!("Failed to configure device: {}.", e))?;

    Ok(device)
}
