# wengwengweng

run example="yo":
	cargo run --release --example {{example}}

bin:
	cargo build --release
	rm -rf bin/dirty
	cp target/release/dirty bin/dirty

doc:
	cargo doc --no-deps --open

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

test-windows example="yo":
	cargo build --release --target x86_64-pc-windows-gnu --example {{example}}

