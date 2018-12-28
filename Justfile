# wengwengweng

macos_target = "x86_64-apple-darwin"
windows_target = "x86_64-pc-windows-gnu"
linux_target = "x86_64-unknown-linux-gnu"
wasm_target = "asmjs-unknown-emscripten"
ios_target = "x86_64-apple-ios"
android_target = "x86_64-linux-android"

default: macos

run args="":
	./dist/macos/dirty {{args}}

example bin="yo":
	cargo run --example {{bin}}

macos:
	cargo build --target {{macos_target}} --release
	rm -rf dist/macos
	mkdir -p dist/macos
	cp target/{{macos_target}}/release/lua dist/macos/dirty

windows:
	cargo build --target {{windows_target}} --release
	rm -rf dist/windows
	mkdir -p dist/windows
	cp target/{{windows_target}}/release/lua.exe dist/windows/dirty.exe

linux:
	cargo build --target {{linux_target}} --release

wasm:
	cargo build --target {{wasm_target}} --release

ios:
	cargo build --target {{ios_target}} --release

android:
	cargo build --target {{android_target}} --release

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

