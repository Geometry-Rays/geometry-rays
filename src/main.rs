use std::process::exit;

use macroquad::prelude::*;

mod funcs;
use funcs::*;

#[macroquad::main("Geometry Rays")]
async fn main() {
    let font = load_ttf_font("./Resources/Acme 9 Regular.ttf").await.unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            exit(0)
        }

        clear_background(BLACK);

        draw_text_pro(
            "Geometry Rays",
            get_screen_data().width() as f32 / 2.0 - measure_text_ex("Geometry Rays", 40, &font) / 2.0,
            100.0 + get_screen_data().width() as f32 / 7.0,
            40,
            RED,
            &font
        );

        next_frame().await
    }
}