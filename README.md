# Template for ev3dev-lang-rust

This is a template for creating and building projects with [`ev3dev-lang-rust`](https://github.com/pixix4/ev3dev-lang-rust). It contains all the necessary files to cross compile the "Hello World" example for ev3dev platform.

## Dependencies

- `cargo`
- `docker`
- `make` (optional)

## Initial setup

This template uses `cargo vendor` to cache all dependencies for the docker build. Before the first run and after changes to your dependencies you need to rebuild this dependency cache
```bash
make install-dependencies
# or
cargo vendor
```

## Usage

```bash
make build
# or
docker run --rm -v $(PWD):/opt/project/ -w /opt/project/ pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /opt/project/target/armv5te-unknown-linux-gnueabi/release/<ARTIFACT>"
```

The resulting binary can be found at `./target/armv5te-unknown-linux-gnueabi/release/<ARTIFACT>`.
