#![allow(dead_code, non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl srgb {
    pub const RED: srgb = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
}
