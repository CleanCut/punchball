pub mod arena;
pub mod event;
pub mod gamepad;
pub mod player;
pub mod setup;

use bevy::prelude::Color;
use palette::Srgb;

/// Take a color in non-linear srgb and convert it to the linear srgb format Bevy currently needs
pub fn color_from_f32(r: f32, g: f32, b: f32) -> Color {
    let l = Srgb::new(r, g, b).into_linear();
    Color::rgb(l.red, l.green, l.blue)
}

/// Take a color in non-linear srgb and convert it to the linear srgb format Bevy currently needs
pub fn color_from_u8(r: u8, g: u8, b: u8) -> Color {
    color_from_f32(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
