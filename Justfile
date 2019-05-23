# wengwengweng

run: build
	./bin/dirty examples/main.lua

build:
	cargo build --release
	rm -rf bin/dirty
	cp target/release/dirty bin/dirty

doc:
	cargo doc --no-deps --open

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

