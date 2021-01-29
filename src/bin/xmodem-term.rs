use clap::clap_app;
use std::fs::File;
use xmodem_term::{device::setup_device, xmodem::XModem};
use serial::{BaudRate, CharSize, FlowControl, Parity, StopBits};

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
            possible_values(&[ "110", "300", "600", "1200", "2400",
                "4800", "9600", "19200", "38400", "57600", "115200" ])
            "The Baud rate of the serial."
        )
        (@arg char_size: -c --charsize +takes_value default_value("8")
            possible_values(&[ "5", "6", "7", "8" ])
            "The number of bits per character."
        )
        (@arg parity: -p --parity +takes_value default_value("none")
            possible_values(&[ "even", "odd", "none" ])
            "The parity checking mode."
        )
        (@arg stop_bits: -s --stopbits +takes_value default_value("1")
            possible_values(&[ "1", "2" ])
            "The number of stop bits transmitted after every character."
        )
        (@arg flow_control: -f --flowcontrol +takes_value default_value("none")
            possible_values(&[ "software", "hardware", "none" ])
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

    let device = setup_device(
        device_path,
        arg_match_to_parser(&matches, "baud_rate",    get_baud_rate),
        arg_match_to_parser(&matches, "char_size",    get_char_size),
        arg_match_to_parser(&matches, "parity",       get_parity),
        arg_match_to_parser(&matches, "stop_bits",    get_stop_bits),
        arg_match_to_parser(&matches, "flow_control", get_flow_control),
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

fn arg_match_to_parser<T>(
    matches: &clap::ArgMatches,
    arg: &str,
    parser: fn(&str) -> T,
) -> T {
    parser(matches.value_of(arg).unwrap())
}

// TODO: Maybe macro_rules! these?

fn get_baud_rate(name: &str) -> BaudRate {
    use BaudRate::*;
    match name {
        "110" => Baud110,
        "300" => Baud300,
        "600" => Baud600,
        "1200" => Baud1200,
        "2400" => Baud2400,
        "4800" => Baud4800,
        "9600" => Baud9600,
        "19200" => Baud19200,
        "38400" => Baud38400,
        "57600" => Baud57600,
        "115200" => Baud115200,
        _ => panic!(),
    }
}

fn get_char_size(name: &str) -> CharSize {
    use CharSize::*;
    match name {
        "5" => Bits5,
        "6" => Bits6,
        "7" => Bits7,
        "8" => Bits8,
        _ => panic!(),
    }
}

fn get_parity(name: &str) -> Parity {
    use Parity::*;
    match name {
        "even" => ParityEven,
        "none" => ParityNone,
        "odd" => ParityOdd,
        _ => panic!(),
    }
}

fn get_stop_bits(name: &str) -> StopBits {
    use StopBits::*;
    match name {
        "1" => Stop1,
        "2" => Stop2,
        _ => panic!(),
    }
}

fn get_flow_control(name: &str) -> FlowControl {
    use FlowControl::*;
    match name {
        "hardware" => FlowHardware,
        "none" => FlowNone,
        "software" => FlowSoftware,
        _ => panic!(),
    }
}
