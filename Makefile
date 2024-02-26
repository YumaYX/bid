build:
	cargo build --release

install: build
	cp -pv target/release/bid /usr/local/bin/

test:
	cargo test

clean:
	rm -rf target
