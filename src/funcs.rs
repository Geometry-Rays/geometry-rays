use macroquad::prelude::*;

pub fn draw_text_pro(
    text: &str,
    x: f32,
    y: f32,
    size: u8,
    color: Color,
    font: &Font
) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            color: color,
            ..Default::default()
        }
    );
}

pub fn measure_text_ex(
    text: &str,
    size: u8,
    font: &Font
) -> f32 {
    measure_text(text, Some(font), size as u16, 1.0).width
}