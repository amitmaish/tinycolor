#![allow(dead_code, non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

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
