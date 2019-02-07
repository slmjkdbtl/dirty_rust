# wengwengweng

run example="yo":
	cargo run --release --example {{example}}

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

outdated:
	cargo outdated --depth 1

