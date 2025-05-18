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
//!
//! assert_eq!(color_rgb, rgb::from(color_srgb));
//! ```

use std::f32::consts::PI;

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
            r: rgb::to_linear(value.r),
            g: rgb::to_linear(value.g),
            b: rgb::to_linear(value.b),
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
        if x >= 0.0031308 {
            (1.055) * x.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * x
        }
    }

    fn to_linear(x: f32) -> f32 {
        if x >= 0.04045 {
            ((x + 0.055) / (1.0 + 0.055)).powf(2.4)
        } else {
            x / 12.92
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
            r: rgb::from_linear(value.r),
            g: rgb::from_linear(value.g),
            b: rgb::from_linear(value.b),
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
    fn from(value: okhsl) -> Self {
        let h = value.h;
        let s = value.s;
        let l = value.l;

        // if l == 1.0 {
        //     return srgb::WHITE.into();
        // } else if l == 0.0 {
        //     return srgb::BLACK.into();
        // }

        let a_ = f32::cos(2.0 * PI * h);
        let b_ = f32::sin(2.0 * PI * h);
        let l_ = if l != 0.0 { toe_inv(l) } else { 0.0 };

        let (c0, c_mid, c_max) = get_cs(l, a_, b_);

        let t;
        let k0;
        let k1;
        let k2;
        if s < 0.8 {
            t = 1.25 * s;
            k0 = 0.0;
            k1 = 0.8 * c0;
            k2 = 1.0 - k1 / c_mid;
        } else {
            t = 5.0 * (s - 0.8);
            k0 = c_mid;
            k1 = 0.2 * c_mid * c_mid * 1.25 * 1.25 / c0;
            k2 = 1.0 - (k1 / (c_max - c_mid));
        }

        let c = k0 + t * k1 / (1.0 - k2 * t);

        Self {
            l: l_,
            a: c * a_,
            b: c * b_,
        }
    }
}

impl From<okhsv> for oklab {
    fn from(value: okhsv) -> Self {
        let h = value.h;
        let s = value.s;
        let v = value.v;

        let a_ = f32::cos(2.0 * PI * h);
        let b_ = f32::sin(2.0 * PI * h);

        let (s_max, t) = get_st_max(a_, b_, None);
        let s0 = 0.5;
        let k = 1.0 - s0 / s_max;

        let l_v = 1.0 - s * s0 / (s0 + t - t * k * s);
        let c_v = s * t * s0 / (s0 + t - t * k * s);

        let mut l = v * l_v;
        let mut c = v * c_v;

        let l_vt = toe_inv(l_v);
        let c_vt = c_v * l_vt / l_v;

        let l_new = toe_inv(l);
        c = c * l_new / l;
        l = l_new;

        let rgb_scale = rgb::from(oklab {
            l: l_vt,
            a: a_ * c_vt,
            b: b_ * c_vt,
        });
        let scale_l = f32::cbrt(1.0 / rgb_scale.r.max(rgb_scale.g.max(rgb_scale.b.max(0.0))));

        l *= scale_l;
        c *= scale_l;

        Self {
            l,
            a: c * a_,
            b: c * b_,
        }
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
    fn from(value: oklab) -> Self {
        let l_ = value.l;
        let a = value.a;
        let b = value.b;

        let c = f32::sqrt(a * a + b * b);
        let a_ = a / c;
        let b_ = b / c;

        let h = 0.5 + 0.5 * f32::atan2(-b, -a) / PI;

        let (c0, c_mid, c_max) = get_cs(l_, a_, b_);

        let s = if c < c_mid {
            let k0 = 0.0;
            let k1 = 0.8 * c0;
            let k2 = 1.0 - k1 / c_mid;

            let t = (c - k0) / (k1 + k2 * (c - k0));
            t * 0.8
        } else {
            let k0 = c_mid;
            let k1 = 0.2 * c_mid * c_mid * 1.25 * 1.25 / c0;
            let k2 = 1.0 - (k1) / (c_max - c_mid);

            let t = (c - k0) / (k1 + k2 * (c - k0));
            0.8 + 0.2 * t
        };

        let l = toe(l_);

        Self { h, s, l }
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
    fn from(value: oklab) -> Self {
        let mut l = value.l;
        let a = value.a;
        let b = value.b;

        let mut c = f32::sqrt(a * a + b * b);
        let a_ = a / c;
        let b_ = b / c;

        let h = 0.5 + 0.5 * f32::atan2(-b, -a) / PI;

        let (s_max, t) = get_st_max(a_, b_, None);
        let s0 = 0.5;
        let k = 1.0 - s0 / s_max;

        let t_ = t / (c + l * t);
        let l_v = t_ * l;
        let c_v = t_ * c;

        let l_vt = toe_inv(l_v);
        let c_vt = c_v * l_vt / l_v;

        let rgb_scale = rgb::from(oklab {
            l: l_vt,
            a: a_ * c_vt,
            b: b_ * c_vt,
        });
        let scale_l = f32::cbrt(1.0 / rgb_scale.r.max(rgb_scale.g.max(rgb_scale.b.max(0.0))));

        l /= scale_l;
        c /= scale_l;

        c = c * toe(l) / l;
        l = toe(l);

        let v = l / l_v;
        let s = (s0 + t) * c_v / ((t * s0) + t * k * c_v);

        Self { h, s, v }
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
                (g - b) / (d + (if g < b { 6.0 } else { 0.0 }))
            } else if max == g {
                (b - r) / (d + 2.0)
            } else if max == b {
                (r - g) / (d + 4.0)
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

        let h;
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

fn toe(x: f32) -> f32 {
    const K1: f32 = 0.206;
    const K2: f32 = 0.03;
    const K3: f32 = (1.0 + K1) / (1.0 + K2);

    0.5 * (K3 * x - K1 + f32::sqrt((K3 * x - K1) * (K3 * x - K1) + 4.0 * K2 * K3 * x))
}

fn toe_inv(x: f32) -> f32 {
    const K1: f32 = 0.206;
    const K2: f32 = 0.03;
    const K3: f32 = (1.0 + K1) / (1.0 + K2);

    (x * x + K1 * x) / (K3 * (x * K2))
}

fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let k0;
    let k1;
    let k2;
    let k3;
    let k4;
    let wl;
    let wm;
    let ws;

    if -1.88170328 * a - 0.80936493 * b > 1.0 {
        // Red component
        k0 = 1.19086277;
        k1 = 1.76576728;
        k2 = 0.59662641;
        k3 = 0.75515197;
        k4 = 0.56771245;
        wl = 4.0767416621;
        wm = -3.3077115913;
        ws = 0.2309699292;
    } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
        // Green component
        k0 = 0.73956515;
        k1 = -0.45954404;
        k2 = 0.08285427;
        k3 = 0.12541070;
        k4 = 0.14503204;
        wl = -1.2684380046;
        wm = 2.6097574011;
        ws = -0.3413193965;
    } else {
        // Blue component
        k0 = 1.35733652;
        k1 = -0.00915799;
        k2 = -1.15130210;
        k3 = -0.50559606;
        k4 = 0.00692167;
        wl = -0.0041960863;
        wm = -0.7034186147;
        ws = 1.7076147010;
    }
    let k_l = 0.3963377774 * a + 0.2158037573 * b;
    let k_m = -0.1055613458 * a - 0.0638541728 * b;
    let k_s = -0.0894841775 * a - 1.2914855480 * b;

    // Approximate max saturation using a polynomial:
    let s = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    let l_ = 1.0 + s * k_l;
    let m_ = 1.0 + s * k_m;
    let s_ = 1.0 + s * k_s;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let l_ds = 3.0 * k_l * l_ * l_;
    let m_ds = 3.0 * k_m * m_ * m_;
    let s_ds = 3.0 * k_s * s_ * s_;

    let l_ds2 = 6.0 * k_l * k_l * l_;
    let m_ds2 = 6.0 * k_m * k_m * m_;
    let s_ds2 = 6.0 * k_s * k_s * s_;

    let f = wl * l + wm * m + ws * s;
    let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
    let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

    s - f * f1 / (f1 * f1 - 0.5 * f * f2)
}
fn find_cusp(a: f32, b: f32) -> (f32, f32) {
    // First, find the maximum saturation (saturation S = C/L)
    let s_cusp = compute_max_saturation(a, b);

    // Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
    let rgb_at_max: rgb = oklab::from([1.0, s_cusp * a, s_cusp * b]).into();
    // let l_cusp = Math.cbrt(1 / Math.max(Math.max(rgb_at_max[0], rgb_at_max[1]), rgb_at_max[2]));
    let l_cusp = 1.0 / rgb_at_max.r.max(rgb_at_max.g.max(rgb_at_max.b));
    let c_cusp = l_cusp * s_cusp;

    (l_cusp, c_cusp)
}
fn find_gamut_intersection(
    a: f32,
    b: f32,
    l1: f32,
    c1: f32,
    l0: f32,
    cusp: Option<(f32, f32)>,
) -> f32 {
    let cusp = if let Some((l_cusp, c_cusp)) = cusp {
        (l_cusp, c_cusp)
    } else {
        find_cusp(a, b)
    };

    let mut t = if ((l1 - l0) * cusp.1 - (cusp.0 - l0) * c1) <= 0.0 {
        cusp.1 * l0 / (c1 * cusp.0 + cusp.1 * (l0 - l1))
    } else {
        cusp.1 * (l0 - 1.0) / (c1 * (cusp.0 - 1.0) + cusp.1 * (l0 - l1))
    };

    let dl = l1 - l0;
    let dc = c1;

    let k_l = 0.3963377774 * a + 0.2158037573 * b;
    let k_m = -0.1055613458 * a - 0.0638541728 * b;
    let k_s = -0.0894841775 * a - 1.2914855480 * b;

    let l_dt = dl + dc * k_l;
    let m_dt = dl + dc * k_m;
    let s_dt = dl + dc * k_s;

    {
        let l = l0 * (1.0 - t) + t * l1;
        let c = t * c1;

        let l_ = l + c * k_l;
        let m_ = l + c * k_m;
        let s_ = l + c * k_s;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let ldt = 3.0 * l_dt * l_ * l_;
        let mdt = 3.0 * m_dt * m_ * m_;
        let sdt = 3.0 * s_dt * s_ * s_;

        let ldt2 = 6.0 * l_dt * l_dt * l_;
        let mdt2 = 6.0 * m_dt * m_dt * m_;
        let sdt2 = 6.0 * s_dt * s_dt * s_;

        let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.0;
        let r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
        let r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

        let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
        let mut t_r = -r * u_r;

        let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.0;
        let g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
        let g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

        let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
        let mut t_g = -g * u_g;

        let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.0;
        let b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
        let b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

        let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
        let mut t_b = -b * u_b;

        t_r = if u_r >= 0.0 { t_r } else { 10e5 };
        t_g = if u_g >= 0.0 { t_g } else { 10e5 };
        t_b = if u_b >= 0.0 { t_b } else { 10e5 };

        t += f32::min(t_r, f32::min(t_g, t_b));
    }
    t
}
fn get_st_max(a: f32, b: f32, cusp: Option<(f32, f32)>) -> (f32, f32) {
    let cusp = if let Some((l_cusp, c_cusp)) = cusp {
        (l_cusp, c_cusp)
    } else {
        find_cusp(a, b)
    };
    let l = cusp.0;
    let c = cusp.1;
    (c / l, c / (1.0 - l))
}
fn get_st_mid(a: f32, b: f32) -> (f32, f32) {
    let s = 0.11516993
        + 1.0
            / (7.44778970
                + 4.15901240 * b
                + a * (-2.19557347
                    + 1.75198401 * b
                    + a * (-2.13704948 - 10.02301043 * b
                        + a * (-4.24894561 + 5.38770819 * b + 4.69891013 * a))));

    let t = 0.11239642
        + 1.0
            / (1.61320320 - 0.68124379 * b
                + a * (0.40370612
                    + 0.90148123 * b
                    + a * (-0.27087943
                        + 0.61223990 * b
                        + a * (0.00299215 - 0.45399568 * b - 0.14661872 * a))));

    (s, t)
}
fn get_cs(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let cusp = find_cusp(a, b);

    let c_max = find_gamut_intersection(a, b, l, 1.0, l, Some(cusp));
    let st_max = get_st_max(a, b, Some(cusp));

    let s_mid = 0.11516993
        + 1.0
            / (7.44778970
                + 4.15901240 * b
                + a * (-2.19557347
                    + 1.75198401 * b
                    + a * (-2.13704948 - 10.02301043 * b
                        + a * (-4.24894561 + 5.38770819 * b + 4.69891013 * a))));

    let t_mid = 0.11239642
        + 1.0
            / (1.61320320 - 0.68124379 * b
                + a * (0.40370612
                    + 0.90148123 * b
                    + a * (-0.27087943
                        + 0.61223990 * b
                        + a * (0.00299215 - 0.45399568 * b - 0.14661872 * a))));

    let k = c_max / f32::min(l * st_max.0, (1.0 - l) * st_max.1);

    let c_mid;
    {
        let c_a = l * s_mid;
        let c_b = (1.0 - l) * t_mid;

        c_mid = 0.9
            * k
            * f32::sqrt(f32::sqrt(
                1.0 / (1.0 / (c_a * c_a * c_a * c_a) + 1.0 / (c_b * c_b * c_b * c_b)),
            ));
    }

    let c0;
    {
        let c_a = l * 0.4;
        let c_b = (1.0 - l) * 0.8;

        c0 = f32::sqrt(1.0 / (1.0 / (c_a * c_a) + 1.0 / (c_b * c_b)));
    }

    (c0, c_mid, c_max)
}
