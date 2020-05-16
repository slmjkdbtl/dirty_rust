# wengwengweng

run example="basic":
	cargo run \
		--example {{example}}

build-web example="basic":
	cargo build \
		--example {{example}} \
		--target wasm32-unknown-unknown
	wasm-bindgen \
		target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm \
		--out-dir target/wasm32-unknown-unknown/debug/examples/ \
		--target web \
		--no-typescript

run-web example="basic":
	miniserve . \
		--index examples/web/index.html

install:
	cargo install \
		--force \
		--path .

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

