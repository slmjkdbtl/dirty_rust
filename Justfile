# wengwengweng

run example="basic":
	cargo run --example {{example}}

build-web example="basic":
	cargo build --example {{example}} --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm --out-dir test --target web --no-typescript

install:
	cargo install --force --path .

test:
	cargo test --tests

build:
	cargo build

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

