# bitwig-monome

## Installation

1. Install the Rust toolchain via [`rustup`](https://rustup.rs/).
2. Build the application `cargo build --release`.
3. Put `target/release/bitwig-monome` anywhere on your path.
4. Install DrivenByMoss and follow instructions to set up the OSC controller
in Bitwig.

## Run

By default, the program will connect to port `9000` for Bitwig, which
can be configured in Bitwig under the DrivenByMoss controller, and listen
for OSC messages from Grid on port `8000`.

Run `--help` to see more information about configuring these addresses.