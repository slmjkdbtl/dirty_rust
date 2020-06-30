## Desktop

```sh
$ cargo build --release
```

### MacOS

create a directory

```sh
$ mkdir -p (AppName).app/Contents/MacOS
```

copy the built binary file here

```sh
$ cp target/release/(Executable) (AppName).app/Contents/MacOS
```

and your `(AppName).app` should be openable

for more information, checkout [apple's official doc](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html)

or use my cli tool [packapp](https://git.sr.ht/~slmjkdbtl/packapp) to quickly generate the bundle

## Web

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

use `wasm-bindgen` to generate the wasm and js file

```sh
$ wasm-bindgen target/wasm32-unknown-unknown/debug/(name).wasm --out-dir (anywhere) --target web
```

then simply create a `.html` file that includes the generated `.js` file

## iOS

