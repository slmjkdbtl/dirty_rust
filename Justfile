# wengwengweng

check:
	cargo c

run example:
	cargo run \
		--example {{example}}

build-web example:
	cargo build \
		--example {{example}} \
		--release \
		--target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/examples/{{example}}.wasm \
		--out-dir site/examples \
		--target web \
		--no-typescript

run-site:
	cd site; \
		now dev

deploy-site:
	cd site; \
		now --prod

test:
	cargo test --tests

build:
	cargo build

doc crate="dirty":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update
	cargo outdated --root-deps-only

loc:
	loc

