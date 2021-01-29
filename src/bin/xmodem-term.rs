use clap::clap_app;
use std::fs::File;
use xmodem_term::{device::setup_device, xmodem::XModem};
use serial::{BaudRate, CharSize, FlowControl, Parity, StopBits};

// TODO: Maybe macro_rules! these?

fn get_baud_rate(name: String) -> Result<BaudRate, String> {
    use BaudRate::*;
    match name.as_ref() {
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
        _ => Err(name),
    }
}

fn get_char_size(name: String) -> Result<CharSize, String> {
    use CharSize::*;
    match name.as_ref() {
        "5" => Ok(Bits5),
        "6" => Ok(Bits6),
        "7" => Ok(Bits7),
        "8" => Ok(Bits8),
        _ => Err(name),
    }
}

fn get_parity(name: String) -> Result<Parity, String> {
    use Parity::*;
    match name.as_ref() {
        "even" => Ok(ParityEven),
        "none" => Ok(ParityNone),
        "odd" => Ok(ParityOdd),
        _ => Err(name),
    }
}

fn get_stop_bits(name: String) -> Result<StopBits, String> {
    use StopBits::*;
    match name.as_ref() {
        "1" => Ok(Stop1),
        "2" => Ok(Stop2),
        _ => Err(name),
    }
}

fn get_flow_control(name: String) -> Result<FlowControl, String> {
    use FlowControl::*;
    match name.as_ref() {
        "hardware" => Ok(FlowHardware),
        "none" => Ok(FlowNone),
        "software" => Ok(FlowSoftware),
        _ => Err(name),
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
            validator(|b| get_baud_rate(b).map(|_| ()))
            "The Baud rate of the serial."
        )
        (@arg char_size: -c --charsize +takes_value default_value("8")
            validator(|b| get_char_size(b).map(|_| ()))
            "The number of bits per character."
        )
        (@arg parity: -p --parity +takes_value default_value("none")
            validator(|b| get_parity(b).map(|_| ()))
            "The parity checking mode."
        )
        (@arg stop_bits: -s --stopbits +takes_value default_value("1")
            validator(|b| get_stop_bits(b).map(|_| ()))
            "The number of stop bits transmitted after every character."
        )
        (@arg flow_control: -f --flowcontrol +takes_value default_value("none")
            validator(|b| get_flow_control(b).map(|_| ()))
            "The serial flow control mode."
        )
        (@arg device: +required
            "The device to use for xmodem transfer."
        )
        (@arg file: +required
            "The file to be transferred."
        )
    )
    .get_matches();

    // Its okay to unwrap arguments as they're all either required or defaulted.

    let device_path = matches.value_of("device").unwrap();
    let file_path = matches.value_of("file").unwrap();

    fn unwrap_argument_parser<T>(
        arg: Option<&str>,
        parser: fn(String) -> Result<T, String>,
    ) -> T {
        parser(arg.unwrap().to_owned()).unwrap()
    }

    let device = setup_device(
        device_path,
        unwrap_argument_parser(matches.value_of("baud_rate"),    get_baud_rate),
        unwrap_argument_parser(matches.value_of("char_size"),    get_char_size),
        unwrap_argument_parser(matches.value_of("parity"),       get_parity),
        unwrap_argument_parser(matches.value_of("stop_bits"),    get_stop_bits),
        unwrap_argument_parser(matches.value_of("flow_control"), get_flow_control),
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
