use std::fs::{OpenOptions, File};
use clap::clap_app;

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

    let device = OpenOptions::new()
        .write(true)
        .read(true)
        .open(device_path)
        .map_err(|e| format!("Failed to open device: {}.", e))?;

    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}.", e))?;

    println!("Device: {:?}", device);
    println!("File: {:?}", file);

    Ok(())
}
