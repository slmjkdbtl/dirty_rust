# wengwengweng

name = "yo"
dname = "yo"
identifier = "me.wengwengweng.yo"
version = "0.0.0"

macos_target = "x86_64-apple-darwin"
macos_bundle = "dist/" + name + ".app"
macos_zip = "dist/" + name + ".mac.zip"
macos_plist = macos_bundle + "/Contents/Info.plist"
macos_resources = macos_bundle + "/Contents/Resources"
macos_bin = macos_bundle + "/Contents/MacOS"
macos_plist_template = "misc/mac.plist"

windows_target = "x86_64-pc-windows-gnu"
windows_exe = "dist/" + name + ".exe"
windows_zip = "dist/" + name + ".windows.zip"

linux_target = "x86_64-unknown-linux-gnu"
wasm_target = "wasm32-unknown-unknown"
ios_target = "x86_64-apple-ios"
android_target = "x86_64-linux-android"

run:
	cargo run

macos:
	cargo build --target {{macos_target}} --release

	# clean
	rm -rf {{macos_zip}}
	rm -rf {{macos_bundle}}

	# setup
	mkdir -p dist
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
		zip -9 -y -r -q {{name}}.mac.zip {{name}}.app
	rm -rf {{macos_bundle}}

windows:
	cargo build --target {{windows_target}} --release

	# clean
	rm -rf {{windows_zip}}
	rm -rf {{windows_exe}}

	# setup
	mkdir -p dist

	# copy
	cp target/{{windows_target}}/release/{{name}}.exe dist/{{name}}.exe

	# zip
	cd dist; \
		zip -9 -y -r -q {{name}}.windows.zip {{name}}.exe
	rm -rf {{windows_exe}}

linux:
	cargo build --target {{linux_target}} --release

wasm:
	cargo build --target {{wasm_target}} --release

ios:
	cargo build --target {{ios_target}} --release

android:
	cargo build --target {{android_target}} --release

