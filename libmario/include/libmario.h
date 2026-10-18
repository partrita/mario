#ifndef LIBRIO_H
#define LIBRIO_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct mario_engine mario_engine_t;
typedef struct mario_surface mario_surface_t;
typedef struct mario_render_state mario_render_state_t;

typedef size_t mario_surface_id_t;

#define RIO_ACTION_SET_TITLE 0u
#define RIO_ACTION_RING_BELL 1u
#define RIO_ACTION_CURSOR_BLINKING_CHANGE 2u
/* OSC 9;4 progress: data_a is the ConEmu state (0 remove, 1 set, 2 error,
   3 indeterminate, 4 paused), data_b the 0-100 value. */
#define RIO_ACTION_PROGRESS 3u

#define RIO_COLOR_NAMED 0u
#define RIO_COLOR_INDEXED 1u
#define RIO_COLOR_RGB 2u
/* An absent color (value and rgb are zero). Returned by
 * mario_render_state_cell_underline_color for cells without an explicit
 * underline color, and as the fg/bg of a missing cell. */
#define RIO_COLOR_NONE 3u

#define RIO_KEY_CHAR 0u
#define RIO_KEY_ENTER 1u
#define RIO_KEY_TAB 2u
#define RIO_KEY_BACKSPACE 3u
#define RIO_KEY_ESCAPE 4u
#define RIO_KEY_UP 5u
#define RIO_KEY_DOWN 6u
#define RIO_KEY_LEFT 7u
#define RIO_KEY_RIGHT 8u
#define RIO_KEY_HOME 9u
#define RIO_KEY_END 10u
#define RIO_KEY_PAGE_UP 11u
#define RIO_KEY_PAGE_DOWN 12u
#define RIO_KEY_INSERT 13u
#define RIO_KEY_DELETE 14u
#define RIO_KEY_F 15u
/* No key: an event that carries only text, such as an input method commit. */
#define RIO_KEY_NONE 16u
/* Modifier keys as keys; reported only in kitty report-all mode. */
#define RIO_KEY_CAPS_LOCK 17u
#define RIO_KEY_SHIFT_LEFT 18u
#define RIO_KEY_SHIFT_RIGHT 19u
#define RIO_KEY_CONTROL_LEFT 20u
#define RIO_KEY_CONTROL_RIGHT 21u
#define RIO_KEY_ALT_LEFT 22u
#define RIO_KEY_ALT_RIGHT 23u
#define RIO_KEY_SUPER_LEFT 24u
#define RIO_KEY_SUPER_RIGHT 25u

#define RIO_KEY_ACTION_PRESS 0u
#define RIO_KEY_ACTION_REPEAT 1u
#define RIO_KEY_ACTION_RELEASE 2u

#define RIO_SELECTION_SIMPLE 0u
#define RIO_SELECTION_WORD 1u
#define RIO_SELECTION_LINE 2u
#define RIO_SELECTION_BLOCK 3u

/* mario_render_state_cursor_shape: DECSCUSR shape, or HIDDEN when the program
   hid the cursor (DECTCEM) or the view is scrolled into history. */
#define RIO_CURSOR_BLOCK 0u
#define RIO_CURSOR_UNDERLINE 1u
#define RIO_CURSOR_BEAM 2u
#define RIO_CURSOR_HIDDEN 3u

#define RIO_MOD_SHIFT (1u << 0)
#define RIO_MOD_CTRL (1u << 1)
#define RIO_MOD_ALT (1u << 2)
#define RIO_MOD_SUPER (1u << 3)

typedef struct {
  uint32_t tag;
  /* Valid only for the duration of the callback. */
  const char *title;
  const char *subtitle;
  /* Numeric payload; meaning depends on tag (see RIO_ACTION_*). */
  uint32_t data_a;
  uint32_t data_b;
} mario_action_s;

typedef struct {
  void *userdata;
  /* May fire on libmario's PTY IO thread. Callbacks must only flag or
     schedule work; calling back into mario_* from inside a callback is
     undefined behavior. */
  void (*wakeup_cb)(void *userdata, mario_surface_id_t surface);
  void (*action_cb)(void *userdata, mario_surface_id_t surface, mario_action_s action);
  void (*clipboard_write_cb)(void *userdata, mario_surface_id_t surface, uint8_t kind,
                             const char *utf8);
  void (*close_surface_cb)(void *userdata, mario_surface_id_t surface);
} mario_runtime_config_s;

typedef struct {
  /* NULL means $SHELL, falling back to /bin/sh. */
  const char *shell;
  const char *working_dir;
  uint16_t cols;
  uint16_t rows;
  uint16_t pixel_width;
  uint16_t pixel_height;
  size_t scrollback;
  /* Arguments for the program, after argv[0]. NULL/0 means none. These reach
   * the child through execvp as separate argv entries, so a host with a
   * command to run can spawn it instead of typing it into the terminal:
   * nothing here is ever read by a shell's line editor. */
  const char *const *args;
  size_t args_len;
} mario_surface_config_s;

typedef struct {
  /* Original form: RIO_COLOR_NAMED / _INDEXED / _RGB. `value` holds the
     named-color id or palette index for the first two. Kind
     RIO_COLOR_NONE means no color at all: value and r/g/b are zero and
     nothing should be drawn from them. */
  uint8_t kind;
  uint16_t value;
  /* The resolved RGB for every kind except RIO_COLOR_NONE, so a CPU
     renderer can read r/g/b directly without owning a palette. */
  uint8_t r;
  uint8_t g;
  uint8_t b;
} mario_color_s;

typedef struct {
  uint32_t codepoint;
  mario_color_s fg;
  mario_color_s bg;
  uint16_t style_flags;
} mario_cell_s;

/* Bit assignments for mario_cell_s.style_flags. Bits 6-10 are the
 * underline kind; at most one is set. At most one blink bit is set.
 * Bits 14-15 are the RIO_CELL_* markers, not styles.
 *
 * mario_cell_s.fg/bg are exported pre-swap: when RIO_STYLE_INVERSE is
 * set the host swaps them when drawing, as mario's own renderer does. */
#define RIO_STYLE_INVERSE (1u << 0)
#define RIO_STYLE_BOLD (1u << 1)
#define RIO_STYLE_ITALIC (1u << 2)
#define RIO_STYLE_DIM (1u << 3)
#define RIO_STYLE_HIDDEN (1u << 4)
#define RIO_STYLE_STRIKEOUT (1u << 5)
#define RIO_STYLE_UNDERLINE (1u << 6)
#define RIO_STYLE_DOUBLE_UNDERLINE (1u << 7)
#define RIO_STYLE_UNDERCURL (1u << 8)
#define RIO_STYLE_DOTTED_UNDERLINE (1u << 9)
#define RIO_STYLE_DASHED_UNDERLINE (1u << 10)
#define RIO_STYLE_SLOW_BLINK (1u << 11)
#define RIO_STYLE_RAPID_BLINK (1u << 12)

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} mario_rgb_s;

/* A color scheme, limited to what the engine itself resolves: the 16 ANSI
 * slots plus the named defaults. Selection/cursor rendering colors stay
 * host-side; `cursor` here covers cells that reference the cursor color. */
typedef struct {
  /* ANSI 0-7 normal, 8-15 bright. */
  mario_rgb_s ansi[16];
  mario_rgb_s foreground;
  mario_rgb_s background;
  mario_rgb_s cursor;
} mario_colors_s;

typedef struct {
  uint16_t line;
  uint16_t column;
} mario_cursor_s;

typedef struct {
  bool active;
  uint16_t start_line;
  uint16_t start_col;
  uint16_t end_line;
  uint16_t end_col;
  bool is_block;
} mario_selection_s;

/* A key event as the platform delivered it.
 *
 * The host reports what happened and libmario decides what the terminal
 * receives, because that depends on state the host does not track:
 * application cursor mode, the kitty keyboard flags, and modifyOtherKeys.
 */
typedef struct {
  /* RIO_KEY_ACTION_*. Releases are dropped unless the program running in the
   * terminal asked for key event types. */
  uint32_t action;
  /* RIO_KEY_*, naming the key with no shift applied: shift+a is
   * RIO_KEY_CHAR with codepoint 'a', and the "A" belongs in text. */
  uint32_t tag;
  uint32_t codepoint;
  /* 1 to 12 when tag is RIO_KEY_F. */
  uint8_t function_key;
  /* Every modifier held, as RIO_MOD_*. */
  uint8_t mods;
  /* Modifiers the platform already spent producing text, so they are not
   * encoded twice. Zero when the platform cannot say. */
  uint8_t consumed_mods;
  /* Whether an input method owns the key right now. Nothing is encoded; the
   * text arrives when composition commits. */
  bool composing;
  /* What the platform produced for this key, UTF-8, after any dead key or
   * input method composition. May be NULL. */
  const char *text;
  size_t text_len;
} mario_key_event_s;

/* Hosts on Linux should ignore SIGPIPE: internal wake pipes written from
 * signal handlers expect EPIPE. On macOS/BSD the sockets opt out per-fd. */
mario_engine_t *mario_engine_new(const mario_runtime_config_s *config);
void mario_engine_free(mario_engine_t *engine);

/* Replace the palette used to resolve named/indexed cell colors, process-wide.
 * NULL restores Mario's default theme. Dim variants derive from the new palette.
 * The host owns redraw: repaint every surface after this (cells re-resolve on
 * the next mario_render_state_cell read; no re-snapshot needed). */
void mario_set_colors(const mario_colors_s *colors);

mario_surface_t *mario_surface_new(mario_engine_t *engine,
                               const mario_surface_config_s *config);
void mario_surface_free(mario_surface_t *surface);
mario_surface_id_t mario_surface_id(const mario_surface_t *surface);
/* Pid of the spawned program, for identity and diagnostics only. Do not
 * signal it on teardown: mario_surface_free already hangs up the process
 * group and an internal reader escalates SIGHUP -> SIGKILL and reaps, so
 * a host-side killpg() races that escalation and can hit a recycled pid.
 * On Windows it is the conpty child's process id. 0 when the surface has
 * no PTY or the pid was unavailable. */
uint32_t mario_surface_child_pid(const mario_surface_t *surface);
/* Deliver SIGHUP to the child's process group through its lifecycle
 * guard: a no-op after the child was reaped, so it never signals a
 * recycled pid or races the internal escalation. Returns true when
 * delivery succeeded (unix with a PTY only). */
bool mario_surface_hangup_child(const mario_surface_t *surface);

/* Input entry points are callable from any thread. */
void mario_surface_text(mario_surface_t *surface, const char *bytes, size_t len);
/* Paste `bytes` (UTF-8), bracketed when the program has enabled mode 2004. */
void mario_surface_paste(mario_surface_t *surface, const char *bytes, size_t len);
/* Terminal modes for input decisions the host makes on its own: bit 0 mouse
 * reporting, bit 1 application cursor keys, bit 2 alternate screen, bit 3
 * bracketed paste. */
uint32_t mario_surface_mode_bits(const mario_surface_t *surface);
/* Returns true when the event was consumed and encoded to the PTY. */
bool mario_surface_key(mario_surface_t *surface, const mario_key_event_s *event);
/* Whether alt acts as meta, prefixing with ESC, instead of letting the text
 * the platform produced through. Defaults to true, as terminals do: it is the
 * difference between alt+d deleting a word and inserting a character. */
void mario_surface_set_alt_is_meta(mario_surface_t *surface, bool enabled);
/* Grapheme cluster processing (DEC private mode 2027) as the default for
 * cell layout. On by default; embedders whose renderers assume legacy
 * wcwidth layout can turn it off. A program's DECSET/DECRST still wins at
 * runtime, and RIS restores this configured default. */
void mario_surface_set_grapheme_clustering(mario_surface_t *surface, bool enabled);
void mario_surface_resize(mario_surface_t *surface, uint16_t cols, uint16_t rows,
                        uint16_t pixel_width, uint16_t pixel_height);
void mario_surface_scroll(mario_surface_t *surface, int32_t delta_lines);
/* A wheel scroll, dispatched by libmario: a mouse report when the program
   asked for mouse events, cursor keys on the alternate screen with
   alternate-scroll on, otherwise the scrollback view. `lines` is positive
   for scrolling up; `col`/`row` are the cell under the pointer. Returns
   true when the program consumed it. */
bool mario_surface_scroll_wheel(mario_surface_t *surface, int32_t lines,
                              uint16_t col, uint16_t row, uint8_t mods);

/* Report a mouse button press/release to the program when it asked for mouse
   events (DEC 1000/1002/1003, SGR/UTF8/X10). `button` is 0=left, 1=middle,
   2=right; `mods` uses the same bits as mario_surface_scroll_wheel. Returns
   true when a report reached the program (the host should then skip its own
   selection); false when nothing grabs the mouse, or shift bypasses to a
   local selection. */
bool mario_surface_mouse_button(mario_surface_t *surface, uint16_t col, uint16_t row,
                              uint8_t button, bool pressed, uint8_t mods);

/* Report pointer motion for button-event (1002, a button held) / any-event
   (1003, bare motion) modes. `button` is 0/1/2 for the button held during a
   drag, or 3 for none. Returns true when a report was emitted. */
bool mario_surface_mouse_motion(mario_surface_t *surface, uint16_t col, uint16_t row,
                              uint8_t button, uint8_t mods);

/* `side_right` is true when the pointer sits in the right half of the cell.
   It decides whether that cell falls inside the selection, so a drag can
   reach the cells at both ends. */
void mario_surface_selection_begin(mario_surface_t *surface, int32_t viewport_line,
                                 uint16_t col, uint8_t kind, bool side_right);
void mario_surface_selection_update(mario_surface_t *surface, int32_t viewport_line,
                                  uint16_t col, bool side_right);
void mario_surface_selection_clear(mario_surface_t *surface);
/* Returns NULL when nothing is selected. Free with mario_text_free. */
char *mario_surface_selection_text(const mario_surface_t *surface);

/* Session persistence: current working directory (OSC 7), NULL if unknown.
 * Free with mario_text_free. */
char *mario_surface_working_dir(const mario_surface_t *surface);
/* Foreground process name (the running program). Free with mario_text_free. */
char *mario_surface_foreground_process_name(const mario_surface_t *surface);
/* Inject bytes into the terminal DISPLAY (not the PTY input), for replaying
 * persisted scrollback on restore. */
void mario_surface_inject_output(mario_surface_t *surface, const char *bytes,
                               size_t len);
/* Whole buffer (scrollback + screen) as UTF-8 text, for persist/replay.
 * Free with mario_text_free. */
char *mario_surface_dump(const mario_surface_t *surface);

void mario_text_free(char *text);

/* Render-state calls must all come from one thread. */
mario_render_state_t *mario_render_state_new(const mario_surface_t *surface);
void mario_render_state_free(mario_render_state_t *state);
void mario_render_state_update(mario_render_state_t *state);
uint16_t mario_render_state_lines(const mario_render_state_t *state);
uint16_t mario_render_state_columns(const mario_render_state_t *state);
bool mario_render_state_row_dirty(const mario_render_state_t *state, uint16_t line);
void mario_render_state_reset_dirty(mario_render_state_t *state);
/* Set in mario_cell_s.style_flags when the cell carries attached cluster
 * codepoints (combining marks, or a DEC-2027 grapheme cluster) beyond
 * `codepoint`. Fetch the full text with mario_render_state_cell_cluster
 * and draw that instead of the base char. Style flags proper occupy
 * bits 0-12 (RIO_STYLE_*); this is bit 15. */
#define RIO_CELL_HAS_CLUSTER (1u << 15)

/* Write the full text of a cell (base codepoint plus attached cluster
 * codepoints) as UTF-32 into `out` (capacity `cap` code units).
 * Returns the total codepoint count, which may exceed `cap` (call
 * again with a larger buffer); 0 for plain cells. */
size_t mario_render_state_cell_cluster(const mario_render_state_t *state,
                                     uint16_t line, uint16_t column,
                                     uint32_t *out, size_t cap);

/* Measure the first grapheme cluster in a UTF-32 buffer: returns the
 * number of codepoints the cluster spans and writes its terminal cell
 * width to `out_width` when non-null (2 for wide, 1 for narrow, 0 for
 * bare zero-width marks); 0 codepoints for a null or empty buffer. Same segmentation and width rules as printing
 * under grapheme clustering (DEC mode 2027). The buffer must contain
 * a complete first cluster or the logical end of the text. Values
 * that are not Unicode scalars measure as one single-width codepoint
 * when first and terminate the cluster when later. */
size_t mario_cluster_width(const uint32_t *codepoints, size_t len,
                         uint8_t *out_width);

/* The cell at (line, column). A NULL state or out-of-range query returns
 * a blank cell whose fg/bg have kind RIO_COLOR_NONE, so a miss is never
 * mistaken for a real black cell; real cells always carry a drawable
 * fg/bg kind. */
mario_cell_s mario_render_state_cell(const mario_render_state_t *state, uint16_t line,
                                 uint16_t column);
/* Set in mario_cell_s.style_flags when the cell carries an explicit SGR 58
 * underline color; fetch it with mario_render_state_cell_underline_color.
 * Without this bit, draw the underline in the color used for the glyph:
 * the fallback mario's own renderer applies. This is bit 14. */
#define RIO_CELL_HAS_UNDERLINE_COLOR (1u << 14)
/* The cell's explicit SGR 58 underline color, or kind RIO_COLOR_NONE
 * when the cell has none (also for a NULL state, an out-of-range cell,
 * or an internal error); see RIO_CELL_HAS_UNDERLINE_COLOR. */
mario_color_s mario_render_state_cell_underline_color(const mario_render_state_t *state,
                                                  uint16_t line, uint16_t column);
mario_cursor_s mario_render_state_cursor(const mario_render_state_t *state);
/* RIO_CURSOR_*; see the defines. */
uint8_t mario_render_state_cursor_shape(const mario_render_state_t *state);

/* This frame's dynamic (OSC 10/11/12) default colors. `which`: 0 =
 * foreground, 1 = background, 2 = cursor. Writes the resolved RGB into
 * `out` and returns true when the program has set that color via OSC;
 * returns false and leaves `out` untouched when unset (including after an
 * OSC 110/111/112 reset), so the host falls back to its own scheme. */
bool mario_render_state_dynamic_color(const mario_render_state_t *state,
                                    uint8_t which, mario_rgb_s *out);

size_t mario_render_state_display_offset(const mario_render_state_t *state);
bool mario_render_state_alt_screen(const mario_render_state_t *state);

const uint8_t *mario_symbols_nerd_font(size_t *len);

/* A kitty image placement resolved to pixels. x/y are relative to the
 * grid origin (add your padding); src_* is the normalized source
 * rectangle (origin + size) to crop from the image. */
typedef struct {
  uint32_t image_id;
  int32_t z_index;
  float x;
  float y;
  float width;
  float height;
  float src_x;
  float src_y;
  float src_w;
  float src_h;
} mario_kitty_placement_s;

size_t mario_render_state_kitty_count(const mario_render_state_t *state);
bool mario_render_state_kitty_placement(const mario_render_state_t *state,
                                      size_t index, float cell_width,
                                      float cell_height,
                                      mario_kitty_placement_s *out);
bool mario_render_state_kitty_image_info(const mario_render_state_t *state,
                                       uint32_t image_id, uint32_t *out_width,
                                       uint32_t *out_height,
                                       uint64_t *out_stamp);
size_t mario_render_state_kitty_image_rgba(const mario_render_state_t *state,
                                         uint32_t image_id, uint8_t *buf,
                                         size_t cap);

typedef struct {
  double x;
  double y;
  double width;
  double height;
} mario_glyph_box_s;

bool mario_nerd_constrain(uint32_t codepoint, mario_glyph_box_s glyph,
                        double cell_width, double cell_height,
                        double icon_height_single, uint8_t constraint_width,
                        mario_glyph_box_s *out);
mario_selection_s mario_render_state_selection(const mario_render_state_t *state);

#ifdef __cplusplus
}
#endif

#endif
