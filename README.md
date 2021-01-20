# xmodem-term

A simple xmodem transmission client.

Currently this is hardcoded for use over a serial connection on a unix-like system with fixed configuration, so unless you modify it its unlikely to be useful.

## Building

To build the project, you'll need Rust's Cargo installed, see the Rust website for installation instructions.

In the project root, run `cargo build --release`, or to install run `cargo install --path .`.

## Usage

To use this either run `cargo run --release -- <device> <file>` to transmit a file over the given serial device. If installed, you can do `xmodem-term <device> <file>` instead.

You may need to run this with sudo, and in that case, you'll need to specify the path of the executable directly, so either: `./target/release/xmodem-term` or `~/.cargo/bin/xmodem-term` for in-source and installed respectively.
