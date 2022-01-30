ARTIFACT := $(shell cargo pkgid |  rev | cut -d "/" -f1  | rev | cut -d "\#" -f1) # Try to determine the artifact name. If this does not work replace it with the explicit name.

strip:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/$(ARTIFACT)
