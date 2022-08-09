// src/utils/mod.rs

#[allow(dead_code)]
pub fn color_from_bytes(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, a / 255.0]
}