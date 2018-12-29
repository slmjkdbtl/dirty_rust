# wengwengweng

run bin="yo":
	cargo run --example {{bin}}

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

