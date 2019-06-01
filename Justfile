# wengwengweng

run example="window":
	cargo run --release --example {{example}}

run-lua: build
	./bin/dirty examples/window.lua

install: build
	upx bin/dirty
	install -v bin/dirty $BIN

build:
	cargo build --release
	rm -rf bin/dirty
	cp target/release/dirty bin/dirty

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

doc:
	cargo doc --no-deps --open

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

