use clap::clap_app;

fn main() {
    let matches = clap_app!(app =>
        (name: env!("CARGO_PKG_NAME"))
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@setting ArgRequiredElseHelp)
        (@setting DisableHelpSubcommand)
        (@setting GlobalVersion)
        (@setting StrictUtf8)
        (@setting StrictUtf8)
        (@arg device: +required "The device to use for xmodem transfer.")
        (@arg file: +required "The file to be transferred.")
    )
    .get_matches();
    println!("{:?}", matches);
}
