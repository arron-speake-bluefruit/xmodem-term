use clap::clap_app;
use std::fs::File;
use xmodem_term::{device::setup_device, xmodem::XModem};
use serial::{BaudRate, CharSize, FlowControl, Parity, StopBits};

fn get_baud_rate(name: &str) -> Result<BaudRate, String> {
    use BaudRate::*;
    match name {
        "110" => Ok(Baud110),
        "300" => Ok(Baud300),
        "600" => Ok(Baud600),
        "1200" => Ok(Baud1200),
        "2400" => Ok(Baud2400),
        "4800" => Ok(Baud4800),
        "9600" => Ok(Baud9600),
        "19200" => Ok(Baud19200),
        "38400" => Ok(Baud38400),
        "57600" => Ok(Baud57600),
        "115200" => Ok(Baud115200),
        _ => Err(format!("Invalid baud rate of {}.", name)),
    }
}

fn main() -> Result<(), String> {
    let matches = clap_app!(app =>
        (name: env!("CARGO_PKG_NAME"))
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@setting ArgRequiredElseHelp)
        (@setting DisableHelpSubcommand)
        (@setting GlobalVersion)
        (@setting StrictUtf8)
        (@arg baud_rate: -b --baudrate +takes_value default_value("115200")
            "The Baud rate of the serial.")
        (@arg char_size: -c --charsize +takes_value default_value("8")
            "The number of bits per character.")
        (@arg parity: -p --parity +takes_value default_value("none")
            "The parity checking mode.")
        (@arg stop_bits: -s --stopbits +takes_value default_value("1")
            "The number of stop bits transmitted after every character.")
        (@arg flow_control: -f --flowcontrol +takes_value default_value("none")
            "The serial flow control mode.")
        (@arg device: +required
            "The device to use for xmodem transfer.")
        (@arg file: +required
            "The file to be transferred.")
    )
    .get_matches();

    let device_path = matches.value_of("device").unwrap();
    let file_path = matches.value_of("file").unwrap();

    let device = setup_device(
        device_path,
        get_baud_rate(matches.value_of("baud_rate").unwrap())?,
        CharSize::Bits8,
        Parity::ParityNone,
        StopBits::Stop1,
        FlowControl::FlowNone,
    )?;

    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}.", e))?;

    let xmodem = XModem::new(device);
    match xmodem.send(file) {
        Some(duration) => {
            println!("Took {}ms since first NAK.", duration.as_millis());
            Ok(())
        }
        None => Err(String::from("The XModem transfer failed.")),
    }
}
