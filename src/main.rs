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
        screen_width() as f32 / 2.0 - 100.0,
        screen_height() as f32 / 2.0 - 50.0,
        200.0,
        100.0,
        "Play",
        20,
        false
    );

    // Important game variables
    let game_state: GameState = GameState::Menu;

    // Textures
    let default_bg_no_gradient = load_texture("./Resources/default-bg-no-gradient.png")
        .await.expect("Failed to load background texture");

    loop {
        if is_key_pressed(KeyCode::Escape) {
            exit(0)
        }

        let delta_time: f32 = get_frame_time();

        match game_state {
            GameState::Menu => {
                play_button.update(delta_time);
                play_button.rect.x = screen_width() as f32 / 2.0 - 100.0;
                play_button.rect.y = screen_height() as f32 / 2.0 - 50.0;
            }

            GameState::LevelSelect => {}
        }

        // Drawing
        match game_state {
            GameState::Menu => {
                clear_background(BLACK);
                draw_texture_ex(
                    &default_bg_no_gradient,
                    -50.0,
                    -75.0,
                    Color::from_rgba(20, 20, 20, 255),
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: default_bg_no_gradient.width() * screen_width() as f32 * 0.0008,
                            y: default_bg_no_gradient.height() * screen_width() as f32 * 0.0008
                        }),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None
                    }
                );

                draw_text_pro(
                    "Geometry Rays",
                    screen_width() as f32 / 2.0 - measure_text_ex("Geometry Rays", 40, &font) / 2.0,
                    100.0 + screen_height() as f32 / 7.0,
                    40,
                    RED,
                    &font
                );

                draw_text_pro(
                    "Fyre",
                    screen_width() as f32 / 2.0 - measure_text_ex("Fyre", 20, &font) / 2.0,
                    150.0 + screen_height() as f32 / 7.0,
                    20,
                    ORANGE,
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