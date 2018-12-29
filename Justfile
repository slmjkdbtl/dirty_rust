# wengwengweng

run bin="yo":
	cargo run --example {{bin}} --features=col,res,lua

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

