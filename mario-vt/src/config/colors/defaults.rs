use crate::config::colors::{ColorArray, ColorBuilder, ColorComposition, Format};

// These functions are expected to panic if cannot convert the hex string

#[inline]
pub fn background() -> ColorComposition {
    let cb = ColorBuilder::from_hex(String::from("#FAF4ED"), Format::SRGB0_1).unwrap();
    (cb.to_arr(), cb.to_wgpu())
}

#[inline]
pub fn cursor() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn vi_cursor() -> ColorArray {
    ColorBuilder::from_hex(String::from("#286983"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn tabs() -> ColorArray {
    ColorBuilder::from_hex(String::from("#F2E9DE"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn tabs_active() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn foreground() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn green() -> ColorArray {
    ColorBuilder::from_hex(String::from("#286983"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn red() -> ColorArray {
    ColorBuilder::from_hex(String::from("#B4637A"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn blue() -> ColorArray {
    ColorBuilder::from_hex(String::from("#907AA9"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn yellow() -> ColorArray {
    ColorBuilder::from_hex(String::from("#EA9D34"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn black() -> ColorArray {
    ColorBuilder::from_hex(String::from("#F2E9DE"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn cyan() -> ColorArray {
    ColorBuilder::from_hex(String::from("#56949F"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn magenta() -> ColorArray {
    ColorBuilder::from_hex(String::from("#D7827E"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn white() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn default_light_black() -> ColorArray {
    ColorBuilder::from_hex(String::from("#9893A5"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_blue() -> ColorArray {
    ColorBuilder::from_hex(String::from("#907AA9"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_cyan() -> ColorArray {
    ColorBuilder::from_hex(String::from("#56949F"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_green() -> ColorArray {
    ColorBuilder::from_hex(String::from("#286983"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_magenta() -> ColorArray {
    ColorBuilder::from_hex(String::from("#D7827E"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_red() -> ColorArray {
    ColorBuilder::from_hex(String::from("#B4637A"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_white() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn default_light_yellow() -> ColorArray {
    ColorBuilder::from_hex(String::from("#EA9D34"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn split() -> ColorArray {
    ColorBuilder::from_hex(String::from("#CECACD"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn split_active() -> ColorArray {
    ColorBuilder::from_hex(String::from("#907AA9"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn selection_foreground() -> ColorArray {
    ColorBuilder::from_hex(String::from("#575279"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn selection_background() -> ColorArray {
    ColorBuilder::from_hex(String::from("#DFDAD9"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}

#[inline]
pub fn search_match_background() -> ColorArray {
    ColorBuilder::from_hex(String::from("#56949F"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn search_match_foreground() -> ColorArray {
    ColorBuilder::from_hex(String::from("#FAF4ED"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn search_focused_match_background() -> ColorArray {
    ColorBuilder::from_hex(String::from("#EA9D34"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn search_focused_match_foreground() -> ColorArray {
    ColorBuilder::from_hex(String::from("#FAF4ED"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn hint_foreground() -> ColorArray {
    ColorBuilder::from_hex(String::from("#FAF4ED"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
#[inline]
pub fn hint_background() -> ColorArray {
    ColorBuilder::from_hex(String::from("#EA9D34"), Format::SRGB0_1)
        .unwrap()
        .to_arr()
}
