# Beachball

This is a (very basic) Rust project for the BBC micro:bit embedded development board, based on the “LED roulette” exercise from the [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/microbit/) book.

It uses the micro:bit's LED display matrix to show an animation (the illuminated LED appears to chase round the edge of the display clockwise).

![](https://docs.rust-embedded.org/discovery/microbit/assets/roulette_fast.mp4)

(Video borrowed from the book)

## Prerequisites

You will need Rust (naturally), `cargo-binutils`, and `probe-rs-tools`:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
rustup component add llvm-tools
cargo install cargo-binutils
```

## Installation

To build the program, flash it to your board, and start it running with an RTT terminal session, run:

```sh
cargo embed
```
