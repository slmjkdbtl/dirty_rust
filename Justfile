# wengwengweng

run example="yo":
	cargo run --example {{example}}

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

