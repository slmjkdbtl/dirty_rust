# wengwengweng

check:
	cargo c

check-web:
	cargo c --target wasm32-unknown-unknown

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

doc crate:
	cargo doc \
		--no-deps \
		--open \
		-p {{crate}}

build-doc:
	rm -rf target/doc
	cargo doc \
		--no-deps \
		--release \
		--all-features
	rm -rf site/doc
	cp -r target/doc site/
	cp site/doc/light.css site/doc/dark.css
	cp logo.png site/doc/rust-logo.png
	convert logo.png -resize 128x128 -filter point site/doc/favicon.ico

update:
	cargo update
	cargo outdated --root-deps-only

loc:
	loc

