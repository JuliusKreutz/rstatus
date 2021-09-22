build:
	cargo build --release

clean:
	rm -rf target/ Cargo.lock

install:
	cp -f target/release/rstatus /usr/local/bin/

uninstall:
	rm /usr/local/bin/rstatus
