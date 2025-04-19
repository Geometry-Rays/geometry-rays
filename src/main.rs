use std::process::exit;

use macroquad::prelude::*;

mod funcs;
mod types;
use funcs::*;
use types::*;

#[macroquad::main("Geometry Rays")]
async fn main() {
    let font = load_ttf_font("./Resources/Acme 9 Regular.ttf").await.unwrap();

    // Buttons
    let mut play_button = Button::new(
        get_screen_data().width() as f32 / 2.0 - 100.0,
        300.0,
        200.0,
        100.0,
        "Play",
        20,
        false
    );

    // Important game variables
    let game_state: GameState = GameState::Menu;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            exit(0)
        }

        let delta_time: f32 = get_frame_time();

        match game_state {
            GameState::Menu => {
                play_button.update(delta_time);
                play_button.rect.x = get_screen_data().width() as f32 / 2.0 - 100.0;
                play_button.rect.y = 300.0;
            }

            GameState::LevelSelect => {}
        }

        // Drawing
        match game_state {
            GameState::Menu => {
                clear_background(BLACK);

                draw_text_pro(
                    "Geometry Rays",
                    get_screen_data().width() as f32 / 2.0 - measure_text_ex("Geometry Rays", 40, &font) / 2.0,
                    100.0 + get_screen_data().height() as f32 / 7.0,
                    40,
                    RED,
                    &font
                );

                play_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelSelect => {
                clear_background(BLACK);
            }
        }

        next_frame().await
    }
}