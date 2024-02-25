# Template for ev3dev-lang-rust

This is a template for creating and building projects with [`ev3dev-lang-rust`](https://github.com/pixix4/ev3dev-lang-rust). It contains all the necessary files to cross compile the "Hello World" example for ev3dev platform.

## Dependencies

- `cargo`
- `docker`
- `make` (optional)

## Setup

A docker setup is required. The build works by running `docker run`.

## Usage

Build this project:

```bash
make

# or

docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest cargo build --release --target armv5te-unknown-linux-musleabi
```

The resulting binary can be found at `./target/armv5te-unknown-linux-musleabi/release/<ARTIFACT>`. To execute the binary, copy it onto the robot an execute:
```bash
brickrun -r ./<ARTIFACT>
```
