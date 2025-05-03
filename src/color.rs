#![allow(dead_code, non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
