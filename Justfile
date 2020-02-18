# wengwengweng

run example="sprite":
	cargo run --example {{example}}

install:
	cargo install --force --path .

test:
	cargo test --tests

build:
	cargo build

build-web:
	cargo build --target wasm32-unknown-unknown

build-ios:
	cargo build --target x86_64-apple-ios

build-windows:
	cargo build --target x86_64-pc-windows-gnu

build-linux:
	cargo build --target x86_64-unknown-linux-gnu

doc crate="dirty":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

depgraph:
	cargo deps --all-deps | dot -Tpng > $TMPDIR/graph.png; \
		open $TMPDIR/graph.png

