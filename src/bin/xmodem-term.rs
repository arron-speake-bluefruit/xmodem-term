use clap::clap_app;
use std::fs::File;
use xmodem_term::{device::setup_device, xmodem::XModem};

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
        (@arg device: +required "The device to use for xmodem transfer.")
        (@arg file: +required "The file to be transferred.")
    )
    .get_matches();

    let device_path = matches.value_of("device").unwrap();
    let file_path = matches.value_of("file").unwrap();

    let device = setup_device(device_path)?;

    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}.", e))?;

    let xmodem = XModem::new(device);
    match xmodem.send(file) {
        Some(duration) => {
            println!("XModem transfer completed successfully (Took {}ms since first NAK).",
                duration.as_millis());
            Ok(())
        }
        None => Err(String::from("The XModem transfer failed.")),
    }
}
