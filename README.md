# Template for ev3dev-lang-rust

This is a template for creating and building projects with [`ev3dev-lang-rust`](https://github.com/pixix4/ev3dev-lang-rust). It contains all the necessary files to cross compile the "Hello World" example for ev3dev platform.

## Dependencies

- `cargo`
- `docker`
- `make` (optional)

## Setup

Install the `armv5te-musl` toolchain

```bash
rustup target add armv5te-unknown-linux-musleabi
```

## Usage

Build this project:

```bash
cargo build --release
```

The resulting binary can be found at `./target/armv5te-unknown-linux-musleabi/release/<ARTIFACT>`.

Optionally strip the executable to reduce binary size:

```bash
make strip
# or
docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/<ARTIFACT>
```
