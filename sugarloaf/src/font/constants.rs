use std::sync::LazyLock;

static COMPRESSED_CASCADIA_CODE_NF: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/CascadiaCodeNF.ttf.deflate"));

static COMPRESSED_CASCADIA_CODE_NF_ITALIC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/CascadiaCodeNFItalic.ttf.deflate"));

static DECOMPRESSED_CASCADIA_CODE_NF: LazyLock<&'static [u8]> = LazyLock::new(|| {
    let decompressed = miniz_oxide::inflate::decompress_to_vec(COMPRESSED_CASCADIA_CODE_NF)
        .expect("corrupt embedded CascadiaCodeNF font data");
    Box::leak(decompressed.into_boxed_slice())
});

static DECOMPRESSED_CASCADIA_CODE_NF_ITALIC: LazyLock<&'static [u8]> = LazyLock::new(|| {
    let decompressed = miniz_oxide::inflate::decompress_to_vec(COMPRESSED_CASCADIA_CODE_NF_ITALIC)
        .expect("corrupt embedded CascadiaCodeNFItalic font data");
    Box::leak(decompressed.into_boxed_slice())
});

pub const DEFAULT_FONT_FAMILY: &str = "jbd2nf";

/// JBD2 Nerd Font Regular.
#[inline]
pub fn font_jbd2_nf() -> &'static [u8] {
    mario_fonts::jbd2_nerd_font()
}

pub static FONT_JBD2_NF: LazyLock<&'static [u8]> =
    LazyLock::new(font_jbd2_nf);

/// Cascadia Code Nerd Font, upright, variable `wght` axis (200–700).
#[inline]
pub fn font_cascadia_code_nf() -> &'static [u8] {
    *DECOMPRESSED_CASCADIA_CODE_NF
}

pub static FONT_CASCADIA_CODE_NF: LazyLock<&'static [u8]> =
    LazyLock::new(font_cascadia_code_nf);

/// Cascadia Code Nerd Font, italic, variable `wght` axis (200–700).
#[inline]
pub fn font_cascadia_code_nf_italic() -> &'static [u8] {
    *DECOMPRESSED_CASCADIA_CODE_NF_ITALIC
}

pub static FONT_CASCADIA_CODE_NF_ITALIC: LazyLock<&'static [u8]> =
    LazyLock::new(font_cascadia_code_nf_italic);

/// Default `wght` axis value used when a slot wants regular weight.
pub const WGHT_REGULAR: f32 = 400.0;

/// `wght` axis value used when a slot wants bold weight (matches the
/// Cascadia Code variable font's bold instance).
pub const WGHT_BOLD: f32 = 700.0;
