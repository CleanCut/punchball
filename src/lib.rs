pub mod arena;
pub mod event;
pub mod gamepad;
pub mod physics;
pub mod player;
pub mod setup;

use bevy::prelude::Color;

/// Take a color in non-linear srgb and convert it to the linear srgb format Bevy currently needs
pub fn color_from_f32(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b))
}

/// Take a color in non-linear srgb and convert it to the linear srgb format Bevy currently needs
pub fn color_from_u8(r: u8, g: u8, b: u8) -> Color {
    color_from_f32(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

// From https://github.com/bevyengine/bevy/issues/585#issuecomment-699675135
fn srgb_to_linear(u: f32) -> f32 {
    if u <= 0.04045 {
        u / 12.92
    } else {
        f32::powf((u + 0.055) / 1.055, 2.4)
    }
}

// fn linear_to_srgb(u: f32) -> f32 {
//     if u <= 0.0031308 {
//         12.92 * u
//     } else {
//         1.055 * f32::powf(u, 0.416666) - 0.055
//     }
// }
