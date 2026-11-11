use crate::config::colors::{deserialize_to_arr, deserialize_to_arr_opt, ColorArray};
use crate::config::default_bool_true;
use serde::{Deserialize, Serialize};

#[inline]
pub fn default_unfocused_split_opacity() -> f32 {
    0.7
}

/// macOS hides the strip for a lone tab (the native-app feel);
/// Linux/Windows keep it, drawn as a centred title.
#[inline]
pub fn default_hide_if_single() -> bool {
    cfg!(target_os = "macos")
}

#[inline]
pub fn default_max_tab_width() -> f32 {
    240.0
}

/// Clamp `max_tab_width` to `[80.0, 280.0]`.
///
/// Below the lower bound the close button and title no longer fit.
#[inline]
pub fn clamp_max_tab_width(v: f32) -> f32 {
    if !v.is_finite() {
        return default_max_tab_width();
    }
    v.clamp(80.0, 280.0)
}

/// Clamp `unfocused_split_opacity` to `[0.15, 1.0]`.
///
/// A value of `0.0` makes the inactive pane invisible, which is never what
/// the user wants; the lower bound keeps the pane legible at the darkest
/// setting.
#[inline]
pub fn clamp_unfocused_split_opacity(v: f32) -> f32 {
    v.clamp(0.15, 1.0)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum NavigationMode {
    #[serde(alias = "plain")]
    Plain,
    #[serde(alias = "tab")]
    Tab,
    #[cfg(target_os = "macos")]
    #[serde(alias = "nativetab")]
    NativeTab,
}

#[allow(clippy::derivable_impls)]
impl Default for NavigationMode {
    fn default() -> NavigationMode {
        #[cfg(target_os = "macos")]
        {
            // Use Tab for full GPU rendering
            NavigationMode::Tab
        }

        #[cfg(not(target_os = "macos"))]
        NavigationMode::Tab
    }
}

impl NavigationMode {
    const PLAIN_STR: &'static str = "Plain";
    const TAB_STR: &'static str = "Tab";
    #[cfg(target_os = "macos")]
    const NATIVE_TAB_STR: &'static str = "NativeTab";

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plain => Self::PLAIN_STR,
            Self::Tab => Self::TAB_STR,
            #[cfg(target_os = "macos")]
            Self::NativeTab => Self::NATIVE_TAB_STR,
        }
    }
}

#[inline]
pub fn modes_as_vec_string() -> Vec<String> {
    [
        NavigationMode::Plain,
        NavigationMode::Tab,
        #[cfg(target_os = "macos")]
        NavigationMode::NativeTab,
    ]
    .iter()
    .map(|navigation_mode| navigation_mode.to_string())
    .collect()
}

impl std::fmt::Display for NavigationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNavigationModeError;

impl std::str::FromStr for NavigationMode {
    type Err = ParseNavigationModeError;

    fn from_str(s: &str) -> Result<NavigationMode, ParseNavigationModeError> {
        match s {
            Self::PLAIN_STR => Ok(NavigationMode::Plain),
            Self::TAB_STR => Ok(NavigationMode::Tab),
            #[cfg(target_os = "macos")]
            Self::NATIVE_TAB_STR => Ok(NavigationMode::NativeTab),
            _ => Ok(NavigationMode::default()),
        }
    }
}

/// What a tab's title shows. The 1-based position is drawn by the strip
/// itself (`island.rs`), not by the title template, so it stays put
/// whatever the template renders and cannot be truncated away.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TabTitle {
    /// `1 claude` — position first, then the pane's title text. Position
    /// is the part that survives a narrow tab, so it leads.
    #[serde(rename = "index+title", alias = "index-and-title")]
    IndexAndTitle,
    /// `claude` — the title text alone.
    #[serde(rename = "title")]
    Title,
    /// The pre-0.5.31 look: no position, and the working directory when
    /// the pane sets no title of its own.
    #[serde(rename = "path")]
    Path,
}

#[allow(clippy::derivable_impls)]
impl Default for TabTitle {
    fn default() -> TabTitle {
        TabTitle::IndexAndTitle
    }
}

impl TabTitle {
    const INDEX_AND_TITLE_STR: &'static str = "index+title";
    const TITLE_STR: &'static str = "title";
    const PATH_STR: &'static str = "path";

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndexAndTitle => Self::INDEX_AND_TITLE_STR,
            Self::Title => Self::TITLE_STR,
            Self::Path => Self::PATH_STR,
        }
    }

    /// Whether the strip prefixes every tab with its 1-based position.
    #[inline]
    pub fn shows_index(&self) -> bool {
        matches!(self, Self::IndexAndTitle)
    }
}

impl std::fmt::Display for TabTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTabTitleError;

impl std::str::FromStr for TabTitle {
    type Err = ParseTabTitleError;

    fn from_str(s: &str) -> Result<TabTitle, ParseTabTitleError> {
        match s {
            Self::INDEX_AND_TITLE_STR => Ok(TabTitle::IndexAndTitle),
            Self::TITLE_STR => Ok(TabTitle::Title),
            Self::PATH_STR => Ok(TabTitle::Path),
            _ => Ok(TabTitle::default()),
        }
    }
}

/// Which glyph set the runner draws from. The completion mark is `!`
/// either way — it is ASCII to begin with, which is why it renders
/// everywhere. The sets differ only in the runner: Braille dots, which
/// need a font that carries them, versus `|/-\`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TabGlyphs {
    #[serde(rename = "unicode")]
    Unicode,
    #[serde(rename = "ascii")]
    Ascii,
}

#[allow(clippy::derivable_impls)]
impl Default for TabGlyphs {
    fn default() -> TabGlyphs {
        TabGlyphs::Unicode
    }
}

impl TabGlyphs {
    const UNICODE_STR: &'static str = "unicode";
    const ASCII_STR: &'static str = "ascii";

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unicode => Self::UNICODE_STR,
            Self::Ascii => Self::ASCII_STR,
        }
    }

    /// The runner's frames, in the order they play.
    #[inline]
    pub fn spinner_frames(&self) -> &'static [char] {
        match self {
            Self::Unicode => &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            Self::Ascii => &['|', '/', '-', '\\'],
        }
    }
}

impl std::fmt::Display for TabGlyphs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTabGlyphsError;

impl std::str::FromStr for TabGlyphs {
    type Err = ParseTabGlyphsError;

    fn from_str(s: &str) -> Result<TabGlyphs, ParseTabGlyphsError> {
        match s {
            Self::UNICODE_STR => Ok(TabGlyphs::Unicode),
            Self::ASCII_STR => Ok(TabGlyphs::Ascii),
            _ => Ok(TabGlyphs::default()),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ColorAutomation {
    #[serde(default = "String::new")]
    pub program: String,
    #[serde(default = "String::new")]
    pub path: String,
    #[serde(
        deserialize_with = "deserialize_to_arr",
        default = "crate::config::colors::defaults::tabs"
    )]
    pub color: ColorArray,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Navigation {
    #[serde(default = "NavigationMode::default")]
    pub mode: NavigationMode,
    #[serde(
        default = "Vec::default",
        rename = "color-automation",
        skip_serializing
    )]
    pub color_automation: Vec<ColorAutomation>,
    #[serde(default = "bool::default", skip_serializing)]
    pub clickable: bool,
    #[serde(
        default = "default_bool_true",
        rename = "current-working-directory",
        alias = "cwd"
    )]
    pub current_working_directory: bool,
    #[serde(default = "bool::default", rename = "use-terminal-title")]
    pub use_terminal_title: bool,
    #[serde(default = "default_hide_if_single", rename = "hide-if-single")]
    pub hide_if_single: bool,
    #[serde(default = "default_bool_true", rename = "use-split")]
    pub use_split: bool,
    #[serde(default = "default_bool_true", rename = "open-config-with-split")]
    pub open_config_with_split: bool,
    /// The opacity level of an unfocused split. A value of `1.0` disables the
    /// dim; lower values fade the pane out. Clamped to `[0.15, 1.0]` at load
    /// time — a value of `0` makes the pane invisible, which is never useful.
    #[serde(
        default = "default_unfocused_split_opacity",
        rename = "unfocused-split-opacity"
    )]
    pub unfocused_split_opacity: f32,
    /// The color used to dim an unfocused split. The overlay's alpha is
    /// derived from `unfocused_split_opacity` — this field is an RGB tint
    /// only. When unset, the terminal's background color is used.
    #[serde(
        default = "Option::default",
        deserialize_with = "deserialize_to_arr_opt",
        rename = "unfocused-split-fill"
    )]
    pub unfocused_split_fill: Option<ColorArray>,
    /// Maximum width of a tab in logical pixels. Tabs shrink below
    /// this as more open; the cap only limits how wide a tab grows
    /// when few are open. Clamped to `[80.0, 280.0]` at load time.
    #[serde(default = "default_max_tab_width", rename = "max-tab-width")]
    pub max_tab_width: f32,
    /// What each tab's title shows: `index+title` (the default),
    /// `title`, or `path` for the pre-0.5.31 look.
    #[serde(default = "TabTitle::default", rename = "tab-title")]
    pub tab_title: TabTitle,
    /// Glyph set the running runner draws from.
    #[serde(default = "TabGlyphs::default", rename = "tab-glyphs")]
    pub tab_glyphs: TabGlyphs,
    /// Animate the runner. With it off a tab still shows that its
    /// command is running, but the frames stand still — the cheap
    /// option on a remote display or a machine already at capacity.
    #[serde(default = "default_bool_true", rename = "tab-animation")]
    pub tab_animation: bool,
}

impl Default for Navigation {
    fn default() -> Navigation {
        Navigation {
            mode: NavigationMode::default(),
            color_automation: Vec::default(),
            clickable: false,
            current_working_directory: true,
            use_terminal_title: false,
            hide_if_single: default_hide_if_single(),
            use_split: true,
            unfocused_split_opacity: default_unfocused_split_opacity(),
            unfocused_split_fill: None,
            open_config_with_split: true,
            max_tab_width: default_max_tab_width(),
            tab_title: TabTitle::default(),
            tab_glyphs: TabGlyphs::default(),
            tab_animation: true,
        }
    }
}

impl Navigation {
    #[inline]
    pub fn is_native(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.mode == NavigationMode::NativeTab
        }

        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }

    #[inline]
    pub fn has_navigation_key_bindings(&self) -> bool {
        self.mode != NavigationMode::Plain
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.mode == NavigationMode::Tab
    }

    /// Whether the mario-rendered tab strip ("island") is actually painted
    /// this frame. Mirrors the gate at `island.rs:358` — input layers
    /// (click routing, cursor override) must agree with the renderer so
    /// the empty band over a hidden island doesn't intercept events.
    #[inline]
    pub fn island_visible(&self, num_tabs: usize) -> bool {
        self.is_enabled() && !(self.hide_if_single && num_tabs == 1)
    }

    /// The title template actually rendered: the user's `[title] content`,
    /// except that `tab-title = "path"` restores the path-first default
    /// when `[title] content` still holds the shipped default. Asking for
    /// the old look must not require knowing the old template string, and
    /// an explicitly configured `[title] content` still wins.
    pub fn title_template(&self, configured: &str) -> String {
        if self.tab_title == TabTitle::Path
            && configured == crate::config::defaults::default_title_content()
        {
            return crate::config::defaults::legacy_title_content();
        }

        configured.to_string()
    }

    /// Whether the top band is reserved as custom window chrome this
    /// frame, painted island or not. On macOS the full-size content
    /// view keeps the band whenever Tab navigation is enabled, even
    /// with `hide-if-single` hiding the strip; other platforms render
    /// the terminal from the top when the island is hidden. Must agree
    /// with `padding_top_from_config`, which reserves the band's
    /// height under the same condition.
    #[inline]
    pub fn chrome_band_reserved(&self, num_tabs: usize) -> bool {
        if cfg!(target_os = "macos") {
            self.is_enabled()
        } else {
            self.island_visible(num_tabs)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn chrome_band_follows_padding_contract() {
        use crate::config::navigation::{Navigation, NavigationMode};

        let mut nav = Navigation {
            mode: NavigationMode::Tab,
            hide_if_single: true,
            ..Navigation::default()
        };

        // Visible island: band reserved everywhere.
        assert!(nav.island_visible(2));
        assert!(nav.chrome_band_reserved(2));

        // Hidden island (single tab): macOS keeps the band as chrome,
        // other platforms hand it to the terminal, matching
        // padding_top_from_config.
        assert!(!nav.island_visible(1));
        assert_eq!(nav.chrome_band_reserved(1), cfg!(target_os = "macos"));

        // Non-Tab modes never reserve the band.
        nav.mode = NavigationMode::Plain;
        assert!(!nav.island_visible(1));
        assert!(!nav.chrome_band_reserved(1));
        assert!(!nav.chrome_band_reserved(2));

        #[cfg(target_os = "macos")]
        {
            nav.mode = NavigationMode::NativeTab;
            assert!(!nav.chrome_band_reserved(1));
            assert!(!nav.chrome_band_reserved(2));
        }
    }

    use crate::config::colors::hex_to_color_arr;
    use crate::config::navigation::{Navigation, NavigationMode, TabGlyphs, TabTitle};
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize, PartialEq)]
    struct Root {
        #[serde(default = "Navigation::default")]
        navigation: Navigation,
    }

    /// The default is platform-split: macOS hides the strip for a lone
    /// tab, Linux/Windows keep it as a centred title with no island
    /// behind it.
    #[test]
    fn hide_if_single_platform_default() {
        let decoded = toml::from_str::<Root>("[navigation]\nmode = 'Tab'\n").unwrap();
        assert_eq!(decoded.navigation.hide_if_single, cfg!(target_os = "macos"));
        assert_eq!(
            decoded.navigation.island_visible(1),
            !cfg!(target_os = "macos")
        );
        assert_eq!(
            Navigation::default().island_visible(1),
            !cfg!(target_os = "macos")
        );
        // More than one tab always shows the strip.
        assert!(decoded.navigation.island_visible(2));
    }

    /// Both explicit values must override the platform default.
    #[test]
    fn hide_if_single_explicit_override() {
        let on =
            toml::from_str::<Root>("[navigation]\nmode = 'Tab'\nhide-if-single = true\n")
                .unwrap();
        assert!(on.navigation.hide_if_single);
        assert!(!on.navigation.island_visible(1));
        assert!(on.navigation.island_visible(2));

        let off = toml::from_str::<Root>(
            "[navigation]\nmode = 'Tab'\nhide-if-single = false\n",
        )
        .unwrap();
        assert!(!off.navigation.hide_if_single);
        assert!(off.navigation.island_visible(1));
    }

    /// The new title style and its two escape hatches.
    #[test]
    fn tab_title_mode_parses_and_defaults() {
        let default = toml::from_str::<Root>("[navigation]\nmode = 'Tab'\n").unwrap();
        assert_eq!(default.navigation.tab_title, TabTitle::IndexAndTitle);
        assert!(default.navigation.tab_title.shows_index());
        assert_eq!(default.navigation.tab_glyphs, TabGlyphs::Unicode);
        assert!(default.navigation.tab_animation);

        let titled =
            toml::from_str::<Root>("[navigation]\ntab-title = 'title'\n").unwrap();
        assert_eq!(titled.navigation.tab_title, TabTitle::Title);
        assert!(!titled.navigation.tab_title.shows_index());

        let legacy = toml::from_str::<Root>(
            "[navigation]\ntab-title = 'path'\ntab-glyphs = 'ascii'\n\
             tab-animation = false\n",
        )
        .unwrap();
        assert_eq!(legacy.navigation.tab_title, TabTitle::Path);
        assert!(!legacy.navigation.tab_title.shows_index());
        assert_eq!(legacy.navigation.tab_glyphs, TabGlyphs::Ascii);
        assert!(!legacy.navigation.tab_animation);

        // Both enums round-trip through their own spelling.
        for mode in [TabTitle::IndexAndTitle, TabTitle::Title, TabTitle::Path] {
            assert_eq!(mode.as_str().parse::<TabTitle>().unwrap(), mode);
        }
        for glyphs in [TabGlyphs::Unicode, TabGlyphs::Ascii] {
            assert_eq!(glyphs.as_str().parse::<TabGlyphs>().unwrap(), glyphs);
        }
    }

    /// Asking for the old look must not require knowing the old template
    /// string — but an explicitly written template still wins.
    #[test]
    fn path_mode_only_swaps_an_untouched_default_template() {
        let default_content = crate::config::defaults::default_title_content();

        let path = Navigation {
            tab_title: TabTitle::Path,
            ..Navigation::default()
        };
        assert_eq!(
            path.title_template(&default_content),
            crate::config::defaults::legacy_title_content()
        );
        assert_eq!(path.title_template("{{ columns }}"), "{{ columns }}");

        for tab_title in [TabTitle::IndexAndTitle, TabTitle::Title] {
            let nav = Navigation {
                tab_title,
                ..Navigation::default()
            };
            assert_eq!(nav.title_template(&default_content), default_content);
        }
    }

    /// The ASCII set exists for fonts without the Unicode marks, so it
    /// has to be ASCII all the way down; the Unicode set is the one that
    /// needs a font with the glyphs.
    #[test]
    fn tab_glyph_sets_are_font_safe() {
        assert!(TabGlyphs::Ascii
            .spinner_frames()
            .iter()
            .all(|c| c.is_ascii()));
        assert!(!TabGlyphs::Ascii.spinner_frames().is_empty());
        assert!(TabGlyphs::Unicode
            .spinner_frames()
            .iter()
            .all(|c| !c.is_ascii()));
        assert!(!TabGlyphs::Unicode.spinner_frames().is_empty());
        // The two sets must not overlap, or `ascii` would be a no-op for
        // the users who need it: the frames are the whole difference.
        assert_ne!(
            TabGlyphs::Unicode.spinner_frames(),
            TabGlyphs::Ascii.spinner_frames()
        );
    }

    #[test]
    fn test_plain() {
        let content = r#"
            [navigation]
            mode = 'Plain'
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Plain);
        assert!(!decoded.navigation.clickable);
        assert!(decoded.navigation.color_automation.is_empty());
    }

    #[test]
    fn test_tab() {
        let content = r#"
            [navigation]
            mode = 'Tab'
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(decoded.navigation.color_automation.is_empty());
    }

    #[test]
    fn test_color_automation() {
        let content = r#"
            [navigation]
            mode = 'Tab'
            color-automation = [
                { program = 'vim', color = '#333333' }
            ]
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(!decoded.navigation.color_automation.is_empty());
        assert_eq!(
            decoded.navigation.color_automation[0].program,
            "vim".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[0].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[0].color,
            hex_to_color_arr("#333333")
        );
    }

    #[test]
    fn test_color_automation_arr() {
        let content = r#"
            [navigation]
            mode = 'Tab'
            color-automation = [
                { program = 'ssh', color = '#F1F1F1' },
                { program = 'tmux', color = '#333333' },
                { path = '/home', color = '#ffffff' },
                { program = 'nvim', path = '/usr', color = '#00b952' },
            ]
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(!decoded.navigation.color_automation.is_empty());

        assert_eq!(
            decoded.navigation.color_automation[0].program,
            "ssh".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[0].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[0].color,
            hex_to_color_arr("#F1F1F1")
        );

        assert_eq!(
            decoded.navigation.color_automation[1].program,
            "tmux".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[1].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[1].color,
            hex_to_color_arr("#333333")
        );

        assert_eq!(
            decoded.navigation.color_automation[2].program,
            String::new()
        );
        assert_eq!(
            decoded.navigation.color_automation[2].path,
            "/home".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[2].color,
            hex_to_color_arr("#ffffff")
        );

        assert_eq!(
            decoded.navigation.color_automation[3].program,
            "nvim".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[3].path,
            "/usr".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[3].color,
            hex_to_color_arr("#00b952")
        );
    }
}
