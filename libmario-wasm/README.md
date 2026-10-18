# libmario-wasm

[`libmario`](../libmario) without its `pty` feature, compiled to
wasm32-unknown-unknown and exposed through wasm-bindgen. This is the JS
ABI behind the [`marioterm`](https://github.com/raphamorim/mariotermjs) npm
package, the same way `libmario`'s C ABI backs the Swift/C embedders.

There is no PTY in a browser, so the host owns the transport: child
output goes in through `feed`, and bytes the terminal wants delivered to
the child (key encodings, mouse reports, DA responses) come back out
through the `output` callback. Wire those two to a WebSocket for a real
shell, or to an in-page interpreter for a demo.

Build:

```sh
cargo build -p libmario-wasm --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir pkg \
  target/wasm32-unknown-unknown/release/libmario_wasm.wasm
```

The marioterm web repository pins a mario revision and runs this build in CI;
it is not published to crates.io.
