# libmario

`libmario` is an embeddable terminal library extracted from Mario: the PTY and VT
state core, usable from Swift, C, and anything that speaks C, without pulling in
a windowing stack.

It began as a research exercise studying two prior art decompositions, the
libghostty split (a terminal-state library shipped first, renderer as a later
peer) and [libvterm](https://www.leonerd.org.uk/code/libvterm/), then kept the
parts that fit Mario's existing engine and dropped the rest.

`libmario` is a single Rust crate that wraps [`mario-vt`](../mario-vt), Mario's terminal
core, and compiles to a C ABI. It is deliberately VT-only: PTY, VT state, and a
host-pulled render-state API, with no renderer.

## Distribution

`libmario` is **not published to crates.io**. It lives in the Mario source tree and
exists only to produce static artifacts: a `MarioKit.xcframework` (static lib +
curated header + `module.modulemap`, `module MarioKit`) shipped in Mario's GitHub
releases, built with `cargo build` then `lipo` and
`xcodebuild -create-xcframework`. The C header is curated (cbindgen seeds it,
never auto-committed).

If you write Rust, depend on `mario-vt` directly; that is the published, idiomatic
crate. `libmario` is the C boundary for everyone else, and versions with Mario.

## Design

Owns the PTY (`teletypewriter`), the VT machine (`Machine` / `performer`), and
the grid (`Crosswords`). Knows nothing about pixels: the host pulls a
render-state snapshot with per-row dirty flags (which the host resets) and
draws it however it likes. This pull model is the permanent dogfood test: if
our own renderer can be built on it, so can a third-party CPU renderer, a wasm
canvas, a TUI proxy, or a test harness.

The callback surface is deliberately small: `userdata` + a `wakeup_cb` + a
single enum-tagged `action_cb` + clipboard and close-surface callbacks. Adding
a new action never changes the ABI struct. Input is a per-surface set of entry
points: key (returns a consumed bool), text, preedit (IME), mouse
button/pos/scroll, set_size, set_content_scale, set_focus, set_occlusion.

### Rust API

```rust
pub struct Engine { /* shared config, action fan-out */ }
pub struct Surface { /* one PTY + grid; owns the IO thread */ }
pub struct RenderState { /* snapshot + dirty rows */ }

pub trait SurfaceDelegate: Send + Sync {
    fn wakeup(&self);                                  // any thread; schedule only
    fn action(&self, surface_id: SurfaceId, action: Action);
    fn clipboard_write(&self, kind: ClipboardKind, text: &str);
    fn close_surface(&self, surface_id: SurfaceId);
}

pub enum Action {
    SetTitle(String),
    RingBell,
    ChildExited(i32),
    CursorShape(CursorShape),
    // grows freely
}

impl Surface {
    pub fn new(engine: &Engine, desc: SurfaceDesc) -> Result<Surface>;
    pub fn key(&self, event: KeyEvent) -> bool;        // true = consumed
    pub fn text(&self, s: &str);
    pub fn preedit(&self, s: &str);
    pub fn mouse_button(&self, ...); pub fn mouse_pos(&self, ...);
    pub fn scroll(&self, dx: f64, dy: f64);
    pub fn set_size(&self, cols: u16, rows: u16, px_w: u32, px_h: u32);
    pub fn set_content_scale(&self, scale: f64);
    pub fn set_focus(&self, focused: bool);
    pub fn set_occlusion(&self, occluded: bool);
}

impl RenderState {
    pub fn new(surface: &Surface) -> RenderState;
    pub fn update(&mut self);                          // snapshot under the terminal lock
    pub fn dims(&self) -> (u16, u16);
    pub fn row_dirty(&self, row: u16) -> bool;
    pub fn reset_dirty(&mut self);
    pub fn cell(&self, row: u16, col: u16) -> Cell;    // materialized view
    pub fn row_cells(&self, row: u16) -> &[Cell];
    pub fn cursor(&self) -> Cursor;
    pub fn selection(&self) -> Option<SelectionRange>;
    pub fn palette(&self) -> &Palette;
}
```

### C ABI

```c
typedef struct mario_engine        mario_engine_t;
typedef struct mario_config        mario_config_t;
typedef struct mario_surface       mario_surface_t;
typedef struct mario_render_state  mario_render_state_t;

typedef enum {
  RIO_ACTION_SET_TITLE, RIO_ACTION_RING_BELL,
  RIO_ACTION_CHILD_EXITED, RIO_ACTION_CURSOR_SHAPE,
} mario_action_tag_t;
typedef struct { mario_action_tag_t tag; /* tagged union payload */ } mario_action_s;

typedef struct {
  void *userdata;
  void (*wakeup_cb)(void *ud);                    /* any thread; schedule only */
  void (*action_cb)(void *ud, mario_surface_t*, mario_action_s);
  void (*clipboard_write_cb)(void *ud, int kind, const char *utf8);
  void (*close_surface_cb)(void *ud, mario_surface_t*);
} mario_runtime_config_s;

mario_engine_t  *mario_engine_new(const mario_runtime_config_s*, const mario_config_t*);
void           mario_engine_free(mario_engine_t*);

mario_surface_t *mario_surface_new(mario_engine_t*, const mario_surface_config_s*);
void           mario_surface_free(mario_surface_t*);
bool mario_surface_key(mario_surface_t*, mario_key_event_s);   /* true = consumed */
void mario_surface_text(mario_surface_t*, const char*, uintptr_t);
void mario_surface_preedit(mario_surface_t*, const char*, uintptr_t);
void mario_surface_mouse_button(mario_surface_t*, mario_mouse_event_s);
void mario_surface_mouse_pos(mario_surface_t*, double x, double y);
void mario_surface_scroll(mario_surface_t*, double dx, double dy);
void mario_surface_set_size(mario_surface_t*, uint16_t cols, uint16_t rows,
                          uint32_t px_w, uint32_t px_h);
void mario_surface_set_content_scale(mario_surface_t*, double);
void mario_surface_set_focus(mario_surface_t*, bool);
void mario_surface_set_occlusion(mario_surface_t*, bool);

mario_render_state_t *mario_render_state_new(mario_surface_t*);
void mario_render_state_free(mario_render_state_t*);
void mario_render_state_update(mario_render_state_t*);
bool mario_render_state_row_dirty(const mario_render_state_t*, uint16_t row);
void mario_render_state_reset_dirty(mario_render_state_t*);
void mario_render_state_dims(const mario_render_state_t*, uint16_t *cols, uint16_t *rows);
const mario_cell_s *mario_render_state_row(const mario_render_state_t*, uint16_t row,
                                       uintptr_t *out_len);
mario_cursor_s mario_render_state_cursor(const mario_render_state_t*);
```

Every `extern "C"` entry point wraps its body in `catch_unwind`, a Rust panic
must never unwind into Swift/C (UB).

Threading contract (identical to today's marioterm split): the surface owns one
PTY IO thread; `wakeup_cb`/`action_cb` may fire on it and must only
flag/schedule. All `render_state_*` calls and input calls are host-thread safe
(input posts to the IO channel; render-state update takes the FairMutex).
