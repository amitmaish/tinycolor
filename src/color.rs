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
