#![allow(dead_code, non_camel_case_types)]
//! this module simplifies the process of working with different color spaces. it consists of two
//! parts: color structs, and the color trait.
//!
//! # structs
//!
//! the structs in this module represent a color in a particular color space. all of them can be
//! cast into each other, as well as casting to and from a [f32; 3]
//!
//! ```
//! use tinyutils::color::srgb;
//!
//! let color0 = srgb{r: 1.0, g: 0.5, b: 0.25};
//! let color1: srgb = [1.0, 0.5, 0.25].into();
//!
//! assert_eq!(color0, color1);
//! ```
//!
//! # the color trait
//!
//! every color struct implements the color trait. the color trait ensures that a struct that
//! implements it can be cast to all the colors. when writing functions that take a color as an argument,
//! using a generic color allows the caller to store their colors in whatever format they would
//! like.
//!
//! ```
//! use tinyutils::color::{Color, srgb, rgb};
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

/// any struct that implements this trait must implement Into for all color structs in this
/// module.
pub trait Color: Into<srgb> + Into<rgb> {}

/// a color in the srgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

/// a color in the linear rgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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

impl From<rgb> for srgb {
    fn from(value: rgb) -> Self {
        Self {
            r: to_linear(value.r),
            g: to_linear(value.g),
            b: to_linear(value.b),
        }
    }
}

impl From<srgb> for rgb {
    fn from(value: srgb) -> Self {
        Self {
            r: from_linear(value.r),
            g: from_linear(value.g),
            b: from_linear(value.b),
        }
    }
}

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
