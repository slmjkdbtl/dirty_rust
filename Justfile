# wengwengweng

name = "yo"
dname = "yo"
identifier = "me.wengwengweng.yo"
version = "0.0.0"

macos_target = "x86_64-apple-darwin"
macos_bundle = "dist/" + name + "/" + name + ".app"
macos_zip = "dist/" + name + ".mac.zip"
macos_plist = macos_bundle + "/Contents/Info.plist"
macos_resources = macos_bundle + "/Contents/Resources"
macos_bin = macos_bundle + "/Contents/MacOS"
macos_plist_template = "misc/mac.plist"

windows_target = "x86_64-pc-windows-gnu"
windows_zip = "dist/" + name + ".windows.zip"

linux_target = "x86_64-unknown-linux-gnu"
wasm_target = "asmjs-unknown-emscripten"
ios_target = "x86_64-apple-ios"
android_target = "x86_64-linux-android"

run:
	cargo run

macos:
	cargo build --target {{macos_target}} --release

	# clean
	rm -rf dist/{{name}}
	rm -rf {{macos_zip}}

	# setup
	mkdir -p dist
	mkdir -p dist/{{name}}
	mkdir -p {{macos_bundle}}/Contents

	# plist
	cp {{macos_plist_template}} {{macos_plist}}
	sed -i "" "s/##name##/"{{name}}"/" {{macos_plist}}
	sed -i "" "s/##dname##/"{{dname}}"/" {{macos_plist}}
	sed -i "" "s/##identifier##/"{{identifier}}"/" {{macos_plist}}
	sed -i "" "s/##version##/"{{version}}"/" {{macos_plist}}

	# bin
	mkdir -p {{macos_bin}}
	cp target/{{macos_target}}/release/{{name}} {{macos_bin}}/{{name}}

	# resources
	mkdir -p {{macos_resources}}
	sips -s format icns icon.png --out {{macos_bundle}}/Contents/Resources/icon.icns

	# compress
	cd dist; \
		zip -9 -y -r -q {{name}}.mac.zip {{name}}
	rm -rf dist/{{name}}

windows:
	cargo build --target {{windows_target}} --release

	# clean
	rm -rf dist/{{name}}
	rm -rf {{windows_zip}}

	# setup
	mkdir -p dist
	mkdir -p dist/{{name}}

	# copy
	cp target/{{windows_target}}/release/{{name}}.exe dist/{{name}}/{{name}}.exe

	# zip
	cd dist; \
		zip -9 -y -r -q {{name}}.windows.zip {{name}}
	rm -rf dist/{{name}}

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

