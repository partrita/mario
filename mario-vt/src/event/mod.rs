pub mod sync;

use crate::ansi::graphics::UpdateQueues;
use crate::clipboard::ClipboardType;
use crate::config::colors::ColorRgb;
use crate::crosswords::grid::Scroll;
use crate::crosswords::pos::{Direction, Pos};
use crate::crosswords::search::{Match, RegexSearch};
use crate::error::MarioError;
#[cfg(feature = "mario-window")]
use mario_window::event::Event as MarioWindowEvent;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

#[cfg(feature = "mario-window")]
use mario_window::event_loop::EventLoopProxy;

/// The core's own window identifier. `mario-vt` never aliases this to a
/// windowing crate's type: it is always a plain id, so the terminal core
/// stays independent of `mario-window`. With the `mario-window` feature,
/// `From` conversions bridge to/from `mario_window::window::WindowId` at the
/// frontend boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(u64);

impl From<u64> for WindowId {
    fn from(id: u64) -> Self {
        WindowId(id)
    }
}

impl From<WindowId> for u64 {
    fn from(id: WindowId) -> Self {
        id.0
    }
}

#[cfg(feature = "mario-window")]
impl From<WindowId> for mario_window::window::WindowId {
    fn from(id: WindowId) -> Self {
        mario_window::window::WindowId::from(id.0)
    }
}

#[cfg(feature = "mario-window")]
impl From<mario_window::window::WindowId> for WindowId {
    fn from(id: mario_window::window::WindowId) -> Self {
        WindowId(u64::from(id))
    }
}

/// Terminal viewport size, in cells and in pixels.
///
/// Owned by the core so the event model does not name the PTY layer's type
/// for four integers; the PTY driver converts at its own boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowSize {
    pub rows: u16,
    pub cols: u16,
    pub width: u16,
    pub height: u16,
}

#[cfg(feature = "pty")]
impl From<WindowSize> for teletypewriter::WinsizeBuilder {
    fn from(size: WindowSize) -> Self {
        Self {
            rows: size.rows,
            cols: size.cols,
            width: size.width,
            height: size.height,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MarioEventType {
    Mario(MarioEvent),
    Frame,
    // Message(Message),
}

#[derive(Debug)]
pub enum Msg {
    /// Data that should be written to the PTY.
    Input(Cow<'static, [u8]>),

    #[allow(dead_code)]
    Shutdown,

    Resize(WindowSize),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClickState {
    None,
    Click,
    DoubleClick,
    TripleClick,
}

/// Terminal damage hint — coarse signal for the renderer's update path.
/// The actual per-row decision lives on the snapshot's `Row::dirty`
/// (post-`snapshot_visible`); this enum just gates `update` itself
/// (skip vs incremental vs full rebuild). Variants:
/// - `Noop` — no terminal-side change worth rendering for
/// - `Full` — global state changed (resize, palette, mode flip),
///   force a full rebuild even if no individual row is dirty
/// - `Partial` — at least one row's content changed; the snapshot's
///   per-row dirty bits identify which rows
/// - `CursorOnly` — cursor moved/blinked, no cell content changed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerminalDamage {
    /// Nothing changed — skip rendering entirely
    #[default]
    Noop,
    /// The entire terminal needs to be redrawn
    Full,
    /// At least one row changed; consult per-row dirty bits
    Partial,
    /// Only the cursor position has changed
    CursorOnly,
}

#[derive(Clone)]
pub enum MarioEvent {
    PrepareRender(u64),
    PrepareRenderOnRoute(u64, usize),
    PrepareUpdateConfig,
    /// New terminal content available.
    Render,
    /// New terminal content available per route.
    RenderRoute(usize),
    /// Terminal content changed — lightweight notification (no damage payload).
    /// Damage stays in the terminal; renderer extracts it when it locks.
    TerminalDamaged(usize),
    /// Graphics update available from terminal.
    UpdateGraphics {
        route_id: usize,
        queues: UpdateQueues,
    },
    /// A pane's Glyph Protocol registry just became live (first
    /// `register` after session start, or first register following
    /// a clear-all). Frontend installs it into the font library so
    /// subsequent renders consult it. Fires at most once per
    /// (route_id × registry-arc) pair; the registry is Arc-shared,
    /// so further `register`/`clear` mutations made through the
    /// existing handle are visible without re-firing.
    #[cfg(feature = "graphics")]
    GlyphProtocolInstalled {
        route_id: usize,
        registry: mario_graphics::glyph::glyph_registry::GlyphRegistry,
    },
    /// A `q` (query) request arrived from the PTY in `route_id`. The
    /// frontend computes the four-state status — System and/or
    /// Glossary coverage — by consulting both `FontLibrary` (system
    /// fonts) and the per-route glyph registry, then writes the
    /// formatted reply back to the same pane's PTY. Asynchronous
    /// because the dispatcher (in mario-backend) doesn't have access
    /// to the FontLibrary; the frontend does.
    GlyphProtocolQuery {
        route_id: usize,
        cp: u32,
    },
    Paste,
    Copy(String),
    UpdateFontSize(u8),
    Scroll(Scroll),
    ToggleFullScreen,
    ToggleAppearanceTheme,
    Minimize(bool),
    Hide,
    HideOtherApplications,
    UpdateConfig,
    CreateWindow,
    ToggleQuake,
    CloseWindow,
    CreateNativeTab(Option<String>),
    CreateConfigEditor,
    SelectNativeTabByIndex(usize),
    SelectNativeTabLast,
    SelectNativeTabNext,
    SelectNativeTabPrev,

    ReportToAssistant(MarioError),

    /// Grid has changed possibly requiring a mouse cursor shape change.
    MouseCursorDirty,

    /// Terminal title change from the PTY identified by `route_id`.
    Title(usize, String),

    /// Working directory change (OSC 7) from the PTY identified by
    /// `route_id`. Payload-less: the handler re-reads the terminal's
    /// stored directory, which the emitter already committed.
    CurrentDirectoryChanged(usize),

    /// Ask the event loop to refresh the native titlebar from the
    /// currently displayed pane. Payload-less on purpose: the handler
    /// re-reads the displayed title at handling time, so a queued poke
    /// can never overwrite a newer title with a stale snapshot, and
    /// redundant pokes dedupe at the sink.
    SyncWindowTitle,

    /// Window title change.
    TitleWithSubtitle(String, String),

    /// Reset to the default window title.
    ResetTitle,

    /// Request to store a text string in the clipboard.
    ClipboardStore(ClipboardType, String),

    /// Request to write the contents of the clipboard to the PTY.
    ///
    /// `route_id` identifies the panel that emitted the request so
    /// the bytes land on the originating PTY rather than whichever
    /// panel happens to be focused. The attached function is a
    /// formatter which transforms the clipboard content into the
    /// expected escape-sequence form.
    ClipboardLoad(
        usize,
        ClipboardType,
        Arc<dyn Fn(&str) -> String + Sync + Send + 'static>,
    ),

    /// Request to write the RGB value of a color to the PTY.
    ///
    /// `route_id` identifies the panel that emitted the request so
    /// the reply lands on the originating PTY. The attached function
    /// is a formatter which transforms the RGB color into the
    /// expected escape-sequence form.
    ColorRequest(
        usize,
        usize,
        Arc<dyn Fn(ColorRgb) -> String + Sync + Send + 'static>,
    ),

    /// Write some text to the PTY identified by `route_id`. Routing
    /// by panel (rather than the focused context) is required so
    /// CSI / OSC reply bytes land on the shell that asked for them
    /// even if the user focuses a different split mid-flight.
    PtyWrite(usize, String),

    /// Request to write the text area size to the PTY of `route_id`.
    TextAreaSizeRequest(
        usize,
        Arc<dyn Fn(WindowSize) -> String + Sync + Send + 'static>,
    ),

    /// Cursor blinking state has changed.
    CursorBlinkingChange,

    CursorBlinkingChangeOnRoute(usize),

    /// Progress bar report from OSC 9;4 sequence
    ProgressReport(ProgressReport),

    /// Terminal bell ring, from the PTY identified by `route_id`.
    Bell(usize),

    /// Desktop notification from OSC 9 or OSC 777.
    DesktopNotification {
        title: String,
        body: String,
    },

    /// Shutdown request.
    Exit,

    /// Quit request.
    Quit,

    /// Leave current terminal.
    CloseTerminal(usize),

    /// The PTY's child process exited, with the raw wait status when the
    /// platform makes it available (interpret with
    /// `std::process::ExitStatus::from_raw` / `ExitStatusExt`).
    ChildExited(usize, Option<i32>),

    BlinkCursor(u64, usize),

    /// Selection scroll tick — auto-scroll while dragging outside viewport.
    SelectionScrollTick,

    /// Update terminal screen colors.
    ///
    /// The first usize is the route_id, the second is the color index to change.
    /// Color index: 0 for foreground, 1 for background, 2 for cursor color.
    ColorChange(usize, usize, Option<ColorRgb>),

    // No operation
    Noop,
}

impl Debug for MarioEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarioEvent::ClipboardStore(ty, text) => {
                write!(f, "ClipboardStore({ty:?}, {text})")
            }
            MarioEvent::ClipboardLoad(route_id, ty, _) => {
                write!(f, "ClipboardLoad(route={route_id}, {ty:?})")
            }
            MarioEvent::TextAreaSizeRequest(route_id, _) => {
                write!(f, "TextAreaSizeRequest(route={route_id})")
            }
            MarioEvent::ColorRequest(route_id, index, _) => {
                write!(f, "ColorRequest(route={route_id}, idx={index})")
            }
            MarioEvent::PtyWrite(route_id, text) => {
                write!(f, "PtyWrite(route={route_id}, {text})")
            }
            MarioEvent::Title(route_id, title) => {
                write!(f, "Title route {route_id} ({title})")
            }
            MarioEvent::CurrentDirectoryChanged(route_id) => {
                write!(f, "CurrentDirectoryChanged route {route_id}")
            }
            MarioEvent::SyncWindowTitle => write!(f, "SyncWindowTitle"),
            MarioEvent::TitleWithSubtitle(title, subtitle) => {
                write!(f, "TitleWithSubtitle({title}, {subtitle})")
            }
            MarioEvent::Minimize(cond) => write!(f, "Minimize({cond})"),
            MarioEvent::Hide => write!(f, "Hide)"),
            MarioEvent::HideOtherApplications => write!(f, "HideOtherApplications)"),
            MarioEvent::CursorBlinkingChange => write!(f, "CursorBlinkingChange"),
            MarioEvent::CursorBlinkingChangeOnRoute(route_id) => {
                write!(f, "CursorBlinkingChangeOnRoute {route_id}")
            }
            MarioEvent::ProgressReport(report) => {
                write!(f, "ProgressReport({:?})", report)
            }
            MarioEvent::MouseCursorDirty => write!(f, "MouseCursorDirty"),
            MarioEvent::ResetTitle => write!(f, "ResetTitle"),
            MarioEvent::PrepareUpdateConfig => write!(f, "PrepareUpdateConfig"),
            MarioEvent::PrepareRender(millis) => write!(f, "PrepareRender({millis})"),
            MarioEvent::PrepareRenderOnRoute(millis, route) => {
                write!(f, "PrepareRender({millis} on route {route})")
            }
            MarioEvent::Render => write!(f, "Render"),
            MarioEvent::RenderRoute(route) => write!(f, "Render route {route}"),
            MarioEvent::TerminalDamaged(route_id) => {
                write!(f, "TerminalDamaged route {route_id}")
            }
            #[cfg(feature = "graphics")]
            MarioEvent::GlyphProtocolInstalled { route_id, .. } => {
                write!(f, "GlyphProtocolInstalled route {route_id}")
            }
            MarioEvent::GlyphProtocolQuery { route_id, cp } => {
                write!(f, "GlyphProtocolQuery route {route_id} cp {cp:#x}")
            }
            MarioEvent::Scroll(scroll) => write!(f, "Scroll {scroll:?}"),
            MarioEvent::Bell(route_id) => write!(f, "Bell route {route_id}"),
            MarioEvent::DesktopNotification { title, body } => {
                write!(f, "DesktopNotification({title}, {body})")
            }
            MarioEvent::Exit => write!(f, "Exit"),
            MarioEvent::Quit => write!(f, "Quit"),
            MarioEvent::CloseTerminal(route) => write!(f, "CloseTerminal {route}"),
            MarioEvent::ChildExited(route, status) => {
                write!(f, "ChildExited(route={route}, status={status:?})")
            }
            MarioEvent::CreateWindow => write!(f, "CreateWindow"),
            MarioEvent::ToggleQuake => write!(f, "ToggleQuake"),
            MarioEvent::CloseWindow => write!(f, "CloseWindow"),
            MarioEvent::CreateNativeTab(_) => write!(f, "CreateNativeTab"),
            MarioEvent::SelectNativeTabByIndex(tab_index) => {
                write!(f, "SelectNativeTabByIndex({tab_index})")
            }
            MarioEvent::SelectNativeTabLast => write!(f, "SelectNativeTabLast"),
            MarioEvent::SelectNativeTabNext => write!(f, "SelectNativeTabNext"),
            MarioEvent::SelectNativeTabPrev => write!(f, "SelectNativeTabPrev"),
            MarioEvent::CreateConfigEditor => write!(f, "CreateConfigEditor"),
            MarioEvent::UpdateConfig => write!(f, "ReloadConfiguration"),
            MarioEvent::ReportToAssistant(error_report) => {
                write!(f, "ReportToAssistant({})", error_report.report)
            }
            MarioEvent::ToggleFullScreen => write!(f, "FullScreen"),
            MarioEvent::ToggleAppearanceTheme => write!(f, "ToggleAppearanceTheme"),
            MarioEvent::BlinkCursor(timeout, route_id) => {
                write!(f, "BlinkCursor {timeout} {route_id}")
            }
            MarioEvent::SelectionScrollTick => write!(f, "SelectionScrollTick"),
            MarioEvent::Noop => write!(f, "Noop"),
            MarioEvent::Copy(_) => write!(f, "Copy"),
            MarioEvent::Paste => write!(f, "Paste"),
            MarioEvent::UpdateFontSize(action) => write!(f, "UpdateFontSize({action:?})"),
            MarioEvent::UpdateGraphics { route_id, .. } => {
                write!(f, "UpdateGraphics({route_id})")
            }
            MarioEvent::ColorChange(route_id, color, rgb) => {
                write!(f, "ColorChange({route_id}, {color:?}, {rgb:?})")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventPayload {
    /// Event payload.
    pub payload: MarioEventType,
    pub window_id: WindowId,
}

impl EventPayload {
    pub fn new(payload: MarioEventType, window_id: WindowId) -> Self {
        Self { payload, window_id }
    }
}

#[cfg(feature = "mario-window")]
impl From<EventPayload> for MarioWindowEvent<EventPayload> {
    fn from(event: EventPayload) -> Self {
        MarioWindowEvent::UserEvent(event)
    }
}

pub trait OnResize {
    fn on_resize(&mut self, window_size: WindowSize);
}

/// Event Loop for notifying the renderer about terminal events.
pub trait EventListener {
    fn send_event(&self, _event: MarioEvent, _id: WindowId) {}

    fn send_event_with_high_priority(&self, _event: MarioEvent, _id: WindowId) {}

    fn send_redraw(&self, _id: WindowId) {}

    fn send_global_event(&self, _event: MarioEvent) {}
}

#[derive(Clone)]
pub struct VoidListener;

impl From<MarioEvent> for MarioEventType {
    fn from(mario_event: MarioEvent) -> Self {
        Self::Mario(mario_event)
    }
}

impl EventListener for VoidListener {}

#[derive(Debug, Clone)]
#[cfg(feature = "mario-window")]
pub struct EventProxy {
    proxy: EventLoopProxy<EventPayload>,
}

#[cfg(feature = "mario-window")]
impl EventProxy {
    pub fn new(proxy: EventLoopProxy<EventPayload>) -> Self {
        Self { proxy }
    }

    pub fn send_event(&self, event: MarioEventType, id: WindowId) {
        let _ = self.proxy.send_event(EventPayload::new(event, id));
    }
}

#[cfg(feature = "mario-window")]
impl EventListener for EventProxy {
    fn send_event(&self, event: MarioEvent, id: WindowId) {
        let _ = self.proxy.send_event(EventPayload::new(event.into(), id));
    }
}

/// Regex search state.
pub struct SearchState {
    /// Search direction.
    pub direction: Direction,

    /// Current position in the search history.
    pub history_index: Option<usize>,

    /// Change in display offset since the beginning of the search.
    pub display_offset_delta: i32,

    /// Search origin in viewport coordinates relative to original display offset.
    pub origin: Pos,

    /// Focused match during active search.
    pub focused_match: Option<Match>,

    /// Search regex and history.
    ///
    /// During an active search, the first element is the user's current input.
    ///
    /// While going through history, the [`SearchState::history_index`] will point to the element
    /// in history which is currently being previewed.
    pub history: VecDeque<String>,

    /// Compiled search automatons.
    pub dfas: Option<RegexSearch>,
}

impl SearchState {
    /// Search regex text if a search is active.
    pub fn regex(&self) -> Option<&String> {
        self.history_index.and_then(|index| self.history.get(index))
    }

    /// Direction of the search from the search origin.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Focused match during vi-less search.
    pub fn focused_match(&self) -> Option<&Match> {
        self.focused_match.as_ref()
    }

    /// Clear the focused match.
    pub fn clear_focused_match(&mut self) {
        self.focused_match = None;
    }

    /// Active search dfas.
    pub fn dfas_mut(&mut self) -> Option<&mut RegexSearch> {
        self.dfas.as_mut()
    }

    /// Active search dfas.
    pub fn dfas(&self) -> Option<&RegexSearch> {
        self.dfas.as_ref()
    }

    /// Search regex text if a search is active.
    pub fn regex_mut(&mut self) -> Option<&mut String> {
        self.history_index
            .and_then(move |index| self.history.get_mut(index))
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            display_offset_delta: Default::default(),
            focused_match: Default::default(),
            history_index: Default::default(),
            history: Default::default(),
            origin: Default::default(),
            dfas: Default::default(),
        }
    }
}

/// Progress bar state for OSC 9;4 ConEmu/Windows Terminal progress reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// Remove/hide the progress bar (state 0)
    Remove,
    /// Set progress with a specific percentage (state 1)
    Set,
    /// Show error state (state 2)
    Error,
    /// Indeterminate/pulsing progress (state 3)
    Indeterminate,
    /// Paused progress (state 4)
    Pause,
}

/// Progress report from OSC 9;4 sequence
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressReport {
    /// The progress bar state
    pub state: ProgressState,
    /// Optional progress percentage (0-100), only used with Set, Error, and Pause states
    pub progress: Option<u8>,
}
