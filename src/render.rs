use ab_glyph::{FontArc, PxScale};
use chrono::Utc;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};

use crate::circle::CircleState;

pub fn render_frame(state: &CircleState, width: u32, height: u32, font: &FontArc) -> RgbImage {
    let mut img = RgbImage::from_pixel(width, height, Rgb([0, 0, 0]));

    draw_filled_circle_mut(
        &mut img,
        (state.x as i32, state.y as i32),
        state.radius,
        state.color,
    );

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let scale = PxScale::from(16.0);

    draw_text_mut(
        &mut img,
        Rgb([255, 255, 255]),
        10,
        10,
        scale,
        font,
        &timestamp,
    );

    img
}
