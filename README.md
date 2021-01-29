# xmodem-term 0.3.0

A simple xmodem transmission client.

The serial connection can be set up using various options, see `--help` for more information.

## Building

To build the project, you'll need Rust's Cargo installed, see the Rust website for installation instructions.

In the project root, run `cargo build --release`, or to install run `cargo install --path .`.

## Usage

To use this either run `cargo run --release -- <device> <file>` to transmit a file over the given serial device. If installed, you can do `xmodem-term <device> <file>` instead.

You may need to run this with sudo, and in that case, you'll need to specify the path of the executable directly, so either: `./target/release/xmodem-term` or `~/.cargo/bin/xmodem-term` for in-source and installed respectively.
