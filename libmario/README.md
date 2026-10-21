# libmario

Embeddable terminal core extracted from [Mario](https://github.com/partrita/mario):
PTY spawning, VT parsing, terminal state, and a host-pulled render-state API
with per-row dirty tracking. No drawing or windowing code — bring your own
renderer.

`libmario` wraps [`mario-vt`](../mario-vt) as a C ABI for non-Rust consumers (Swift,
C, and anything that speaks C). It is **not published to crates.io**: it lives
in the Mario source tree and ships as the `MarioKit.xcframework` static library in
Mario's GitHub releases. If you write Rust, depend on `mario-vt` directly.

The Rust surface it exposes over the C boundary:

```rust
use libmario::{Engine, RenderState, SurfaceDelegate, SurfaceDesc};
use std::sync::Arc;

struct Delegate;
impl SurfaceDelegate for Delegate {
    fn wakeup(&self, _surface: usize) { /* schedule a render */ }
}

let engine = Engine::new(Arc::new(Delegate));
let surface = engine.create_surface(&SurfaceDesc::default())?;
let mut state = RenderState::new(&surface);
surface.text("ls\r");
state.update();
```

The C ABI is compiled by default (`crate-type = ["staticlib"]`); see
`libmario/include` for the curated header and modulemap, and the `Makefile`
`libmario-xcframework` target for packaging.
