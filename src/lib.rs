#![allow(dead_code, non_camel_case_types, clippy::excessive_precision)]
//! this crate simplifies the process of working with different color spaces, it consists of two parts: color structs, and the color trait.
//!
//! # structs
//!
//! the structs in this module represent a color in a particular color space. all structs can be cast into each other, as well as casting to and from a [f32; 3]
//!
//! ```
//! use tinycolors::srgb;
//!
//! let color0 = srgb{r: 1.0, g: 0.5, b: 0.25};
//! let color1: srgb = [1.0, 0.5, 0.25].into();
//!
//! assert_eq!(color0, color1);
//! ```
//!
//! # the color trait
//!
//! every color struct implements the color trait. the color trait ensures that every thruct that implements it can be cast to all the colors. when writing functions that requre a color, using a generic color allows the caller to store their colors in whatever format thay want.
//! like.
//!
//! ```
//! use tinycolors::{Color, srgb, rgb};
//!
//! fn any_color_as_rgb<T: Color>(color: T) -> rgb {
//!     color.into()
//! }
//!
//! let color_srgb = srgb::WHITE;
//! let color_rgb = any_color_as_rgb(color_srgb);
//! assert_eq!(color_rgb, rgb::from(color_srgb));
//! ```
//!
//! > **_NOTE:_** conversions to and from okhsl and okhsv are thin wrappers for the `okhsl` crate. if that's all you're using, that crate might be a better choice for you.

use serde::{Deserialize, Serialize};

/// any struct that implements this trait must implement Into for all color structs in this module
pub trait Color:
    Into<srgb> + Into<rgb> + Into<oklab> + Into<okhsl> + Into<okhsv> + Into<hsl> + Into<hsv>
{
}

/// a color in the srgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl srgb {
    pub const WHITE: srgb = srgb {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLACK: srgb = srgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const RED: srgb = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    pub const YELLOW: srgb = srgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    pub const GREEN: srgb = srgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub const AQUA: srgb = srgb {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLUE: srgb = srgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub const PURPLE: srgb = srgb {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };
}
impl Color for srgb {}

impl From<[f32; 3]> for srgb {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<srgb> for [f32; 3] {
    fn from(value: srgb) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<rgb> for srgb {
    fn from(value: rgb) -> Self {
        Self {
            r: rgb::from_linear(value.r),
            g: rgb::from_linear(value.g),
            b: rgb::from_linear(value.b),
        }
    }
}

impl From<oklab> for srgb {
    fn from(value: oklab) -> Self {
        rgb::from(value).into()
    }
}

impl From<okhsl> for srgb {
    fn from(value: okhsl) -> Self {
        oklab::from(value).into()
    }
}

impl From<okhsv> for srgb {
    fn from(value: okhsv) -> Self {
        oklab::from(value).into()
    }
}

impl From<hsl> for srgb {
    fn from(value: hsl) -> Self {
        let (h, s, l) = (value.h, value.s, value.l);

        let r;
        let g;
        let b;

        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
                let mut t = t;
                if t < 0.0 {
                    t += 1.0;
                }
                if t > 1.0 {
                    t -= 1.0;
                }
                if t < 1.0 / 6.0 {
                    return p + (q - p) * 6.0 * t;
                }
                if t < 0.5 {
                    return q;
                }
                if t < 2.0 / 3.0 {
                    return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
                }
                p
            }

            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            r = hue_to_rgb(p, q, h + 1.0 / 3.0);
            g = hue_to_rgb(p, q, h);
            b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        }

        Self { r, g, b }
    }
}

impl From<hsv> for srgb {
    fn from(value: hsv) -> Self {
        let h = value.h;
        let s = value.s;
        let v = value.v;

        let i = f32::floor(h * 6.0);
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        let i = i as i32;

        let r;
        let g;
        let b;

        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => {
                // unreachable because of the % 6
                r = 0.0;
                g = 0.0;
                b = 0.0;
            }
        }

        Self { r, g, b }
    }
}

/// a color in the linear rgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl rgb {
    fn from_linear(x: f32) -> f32 {
        let x = x as f64;
        if x >= 0.0031308 {
            ((1.055) * x.powf(1.0 / 2.4) - 0.055) as f32
        } else {
            (12.92 * x) as f32
        }
    }

    fn to_linear(x: f32) -> f32 {
        let x = x as f64;
        if x >= 0.04045 {
            (((x + 0.055) / (1.055)).powf(2.4)) as f32
        } else {
            (x / 12.92) as f32
        }
    }
}
impl Color for rgb {}

impl From<[f32; 3]> for rgb {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<rgb> for [f32; 3] {
    fn from(value: rgb) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<srgb> for rgb {
    fn from(value: srgb) -> Self {
        Self {
            r: rgb::to_linear(value.r),
            g: rgb::to_linear(value.g),
            b: rgb::to_linear(value.b),
        }
    }
}

impl From<oklab> for rgb {
    fn from(value: oklab) -> Self {
        let l_ = value.l + 0.3963377774 * value.a + 0.2158037573 * value.b;
        let m_ = value.l - 0.1055613458 * value.a - 0.0638541728 * value.b;
        let s_ = value.l - 0.0894841775 * value.a - 1.2914855480 * value.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        Self {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        }
    }
}

impl From<okhsl> for rgb {
    fn from(value: okhsl) -> Self {
        oklab::from(value).into()
    }
}

impl From<okhsv> for rgb {
    fn from(value: okhsv) -> Self {
        oklab::from(value).into()
    }
}

impl From<hsl> for rgb {
    fn from(value: hsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsv> for rgb {
    fn from(value: hsv) -> Self {
        srgb::from(value).into()
    }
}

/// a color in the oklab color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct oklab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}
impl Color for oklab {}

impl From<[f32; 3]> for oklab {
    fn from(value: [f32; 3]) -> Self {
        Self {
            l: value[0],
            a: value[1],
            b: value[2],
        }
    }
}

impl From<oklab> for [f32; 3] {
    fn from(value: oklab) -> Self {
        [value.l, value.a, value.b]
    }
}

impl From<rgb> for oklab {
    fn from(value: rgb) -> Self {
        let l = 0.4122214708 * value.r + 0.5363325363 * value.g + 0.051445;
        let m = 0.2119034982 * value.r + 0.6806995451 * value.g + 0.107396;
        let s = 0.0883024619 * value.r + 0.2817188376 * value.g + 0.629978;

        let l_ = f32::cbrt(l);
        let m_ = f32::cbrt(m);
        let s_ = f32::cbrt(s);

        Self {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

impl From<srgb> for oklab {
    fn from(value: srgb) -> Self {
        rgb::from(value).into()
    }
}

impl From<okhsl> for oklab {
    fn from(okhsl { h, s, l }: okhsl) -> Self {
        let ::okhsl::Oklab { l, a, b } =
            ::okhsl::okhsl_to_oklab(::okhsl::Okhsl { h: h as f64, s, l });

        Self { l, a, b }
    }
}

impl From<okhsv> for oklab {
    fn from(okhsv { h, s, v }: okhsv) -> Self {
        let ::okhsl::Oklab { l, a, b } =
            ::okhsl::okhsv_to_oklab(::okhsl::Okhsv { h: h as f64, s, v });

        Self { l, a, b }
    }
}

impl From<hsl> for oklab {
    fn from(value: hsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsv> for oklab {
    fn from(value: hsv) -> Self {
        srgb::from(value).into()
    }
}

/// a color in the okhsl color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct okhsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}
impl Color for okhsl {}

impl From<[f32; 3]> for okhsl {
    fn from(value: [f32; 3]) -> Self {
        Self {
            h: value[0],
            s: value[1],
            l: value[2],
        }
    }
}

impl From<okhsl> for [f32; 3] {
    fn from(value: okhsl) -> Self {
        [value.h, value.s, value.l]
    }
}

impl From<srgb> for okhsl {
    fn from(value: srgb) -> Self {
        oklab::from(value).into()
    }
}

impl From<rgb> for okhsl {
    fn from(value: rgb) -> Self {
        oklab::from(value).into()
    }
}

impl From<oklab> for okhsl {
    fn from(oklab { l, a, b }: oklab) -> Self {
        let ::okhsl::Okhsl { h, s, l } = ::okhsl::oklab_to_okhsl(::okhsl::Oklab { l, a, b });

        Self { h: h as f32, s, l }
    }
}

impl From<okhsv> for okhsl {
    fn from(value: okhsv) -> Self {
        oklab::from(value).into()
    }
}

impl From<hsl> for okhsl {
    fn from(value: hsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsv> for okhsl {
    fn from(value: hsv) -> Self {
        srgb::from(value).into()
    }
}

/// a color in the okhsv color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct okhsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}
impl Color for okhsv {}

impl From<[f32; 3]> for okhsv {
    fn from(value: [f32; 3]) -> Self {
        Self {
            h: value[0],
            s: value[1],
            v: value[2],
        }
    }
}

impl From<okhsv> for [f32; 3] {
    fn from(value: okhsv) -> Self {
        [value.h, value.s, value.v]
    }
}

impl From<srgb> for okhsv {
    fn from(value: srgb) -> Self {
        oklab::from(value).into()
    }
}

impl From<rgb> for okhsv {
    fn from(value: rgb) -> Self {
        oklab::from(value).into()
    }
}

impl From<oklab> for okhsv {
    fn from(oklab { l, a, b }: oklab) -> Self {
        let ::okhsl::Okhsv { h, s, v } = ::okhsl::oklab_to_okhsv(::okhsl::Oklab { l, a, b });

        Self { h: h as f32, s, v }
    }
}

impl From<okhsl> for okhsv {
    fn from(value: okhsl) -> Self {
        oklab::from(value).into()
    }
}

impl From<hsl> for okhsv {
    fn from(value: hsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsv> for okhsv {
    fn from(value: hsv) -> Self {
        srgb::from(value).into()
    }
}

/// a color in the hsl color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}
impl Color for hsl {}

impl From<[f32; 3]> for hsl {
    fn from(value: [f32; 3]) -> Self {
        Self {
            h: value[0],
            s: value[1],
            l: value[2],
        }
    }
}

impl From<hsl> for [f32; 3] {
    fn from(value: hsl) -> Self {
        [value.h, value.s, value.l]
    }
}

impl From<srgb> for hsl {
    fn from(value: srgb) -> Self {
        let r = value.r;
        let g = value.g;
        let b = value.b;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));

        let l = (max + min) / 2.0;

        let mut h;
        let s;

        if max == min {
            h = 0.0;
            s = 0.0;
        } else {
            let d = max - min;
            s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };
            h = if max == r {
                (g - b) / (d) + (if g < b { 6.0 } else { 0.0 })
            } else if max == g {
                (b - r) / (d) + 2.0
            } else if max == b {
                (r - g) / (d) + 4.0
            } else {
                0.0
            };
        }

        h /= 6.0;

        Self { h, s, l }
    }
}

impl From<rgb> for hsl {
    fn from(value: rgb) -> Self {
        srgb::from(value).into()
    }
}

impl From<oklab> for hsl {
    fn from(value: oklab) -> Self {
        srgb::from(value).into()
    }
}

impl From<okhsl> for hsl {
    fn from(value: okhsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<okhsv> for hsl {
    fn from(value: okhsv) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsv> for hsl {
    fn from(value: hsv) -> Self {
        srgb::from(value).into()
    }
}

/// a color in the hsv color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}
impl Color for hsv {}

impl From<[f32; 3]> for hsv {
    fn from(value: [f32; 3]) -> Self {
        Self {
            h: value[0],
            s: value[1],
            v: value[2],
        }
    }
}

impl From<hsv> for [f32; 3] {
    fn from(value: hsv) -> Self {
        [value.h, value.s, value.v]
    }
}

impl From<srgb> for hsv {
    fn from(value: srgb) -> Self {
        let r = value.r;
        let g = value.g;
        let b = value.b;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));

        let mut h;
        let v = max;

        let d = max - min;
        let s = if max == 0.0 { 0.0 } else { d / max };

        if max == min {
            h = 0.0;
        } else if max == r {
            h = (g - b) / d + if g < b { 6.0 } else { 0.0 };
        } else if max == g {
            h = (b - r) / d + 2.0;
        } else if max == b {
            h = (r - g) / d + 4.0;
        } else {
            h = 0.0;
        }

        h /= 6.0;

        Self { h, s, v }
    }
}

impl From<rgb> for hsv {
    fn from(value: rgb) -> Self {
        srgb::from(value).into()
    }
}

impl From<oklab> for hsv {
    fn from(value: oklab) -> Self {
        srgb::from(value).into()
    }
}

impl From<okhsl> for hsv {
    fn from(value: okhsl) -> Self {
        srgb::from(value).into()
    }
}

impl From<okhsv> for hsv {
    fn from(value: okhsv) -> Self {
        srgb::from(value).into()
    }
}

impl From<hsl> for hsv {
    fn from(value: hsl) -> Self {
        srgb::from(value).into()
    }
}
