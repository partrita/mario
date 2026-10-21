// Copyright (c) 2023-present, Raphael Amorim.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! Font concerns shared by Mario's renderers, with no rendering, GPU, or
//! windowing dependencies.
//!
//! - [`nerd_font`]: the Nerd Fonts patcher's per-glyph scaling and
//!   alignment rules (by way of ghostty's generated table), so every
//!   renderer places Powerline separators and icon glyphs identically.
//! - The symbols-only Nerd Font itself, embedded behind the
//!   `symbols-nerd-font` feature for renderers that want icon glyphs
//!   available with no font file shipped or installed (the same call
//!   libghostty makes).

#![forbid(unsafe_code)]

pub mod nerd_font;

use std::sync::LazyLock;

static COMPRESSED_JBD2: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/JBD2NF-Regular.ttf.deflate"));

static DECOMPRESSED_JBD2: LazyLock<&'static [u8]> = LazyLock::new(|| {
    let decompressed = miniz_oxide::inflate::decompress_to_vec(COMPRESSED_JBD2)
        .expect("corrupt embedded JBD2 font data");
    Box::leak(decompressed.into_boxed_slice())
});

/// Returns the decompressed JBD2 Nerd Font raw TTF bytes.
#[inline]
pub fn jbd2_nerd_font() -> &'static [u8] {
    *DECOMPRESSED_JBD2
}

/// JBD2 Nerd Font as raw TTF bytes (decompressed on first access).
pub static JBD2_NERD_FONT: LazyLock<&'static [u8]> = LazyLock::new(jbd2_nerd_font);

/// Default embedded Nerd Font as raw TTF bytes.
#[cfg(feature = "symbols-nerd-font")]
pub static SYMBOLS_NERD_FONT: LazyLock<&'static [u8]> = LazyLock::new(jbd2_nerd_font);
