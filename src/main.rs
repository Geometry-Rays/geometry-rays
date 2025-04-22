use macroquad::prelude::*;
use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;

mod funcs;
mod types;
use funcs::*;
use types::*;

mod menu_logic;
use menu_logic::*;

#[macroquad::main("Geometry Rays")]
async fn main() {
    let font: Font = load_ttf_font("./Resources/Acme 9 Regular.ttf").await.unwrap();

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

    let mut creator_button = Button::new(
        screen_width() as f32 / 2.0 - 100.0,
        screen_height() as f32 / 2.0 + 70.0,
        200.0,
        100.0,
        "Custom Levels",
        15,
        false
    );

    let mut back_button = Button::new(
        20.0,
        20.0,
        150.0,
        75.0,
        "Back",
        15,
        false
    );

    let mut featured_button = Button::new(
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 - 100.0,
        200.0,
        200.0,
        "Featured",
        20,
        true
    );

    let mut create_button = Button::new(
        screen_width() / 2.0 - 310.0,
        screen_height() / 2.0 - 100.0,
        200.0,
        200.0,
        "Create",
        20,
        false
    );

    let mut search_button = Button::new(
        screen_width() / 2.0 + 110.0,
        screen_height() / 2.0 - 100.0,
        200.0,
        200.0,
        "Search",
        20,
        true
    );

    let mut build_tab_button = Button::new(
        10.0,
        screen_height() - 190.0,
        150.0,
        80.0,
        "Build",
        20,
        false
    );

    let mut edit_tab_button = Button::new(
        10.0,
        screen_height() - 100.0,
        150.0,
        80.0,
        "Edit",
        20,
        true
    );

    // Url's for server requests
    let main_url = "http://georays.puppet57.xyz/php-code/".to_string();
    let latest_version_url: String = format!("{}get-latest-version.php", main_url).to_string();

    // Important game variables
    let mut game_state: GameState = GameState::Menu;
    let mut player: Rect = Rect { x: 200.0, y: screen_height() / 1.15, w: 50.0, h: 50.0 };
    let mut on_ground: bool = true;

    // Physics values
    let mut velocity_y: f32 = 0.0;
    let gravity: f32 = 1.0;
    let jump_force: f32 = 16.0;
    let mut rotation: f32 = 0.0;

    // Editor variables
    let mut current_tab: u8 = 1;
    let mut cam_pos_y: f32 = 0.0;

    // More variables
    let version: &str = "F-ALPHA";
    let latest_version: String = ureq::get(latest_version_url)
        .query("fyre", "fyre")
        .call()
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();

    // Textures
    let default_bg_no_gradient = load_texture("./Resources/default-bg-no-gradient.png")
        .await.expect("Failed to load background texture");
    let default_bg = load_texture("./Resources/default-bg.png")
        .await.expect("Failed to load background texture");
    let grnd_texture = load_texture("./Resources/ground.png")
        .await.expect("Failed to load ground texture");

    // Sounds
    let menu_loop_sound = macroquad::audio::load_sound("./Resources/menu-music.ogg").await.unwrap();

    play_sound(&menu_loop_sound, PlaySoundParams { looped: true, volume: 2.0 });
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let delta_time: f32 = get_frame_time();

        match game_state {
            GameState::Menu => {
                play_button.update(delta_time);
                creator_button.update(delta_time);
                play_button.rect.x = screen_width() as f32 / 2.0 - 100.0;
                play_button.rect.y = screen_height() as f32 / 2.0 - 50.0;

                creator_button.rect.x = screen_width() as f32 / 2.0 - 100.0;
                creator_button.rect.y = screen_height() as f32 / 2.0 + 60.0;

                if play_button.is_clicked() {
                    game_state = GameState::LevelSelect
                }

                if creator_button.is_clicked() {
                    game_state = GameState::CreatorMenu
                }
            }

            GameState::LevelSelect => {
                back_button.update(delta_time);

                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Playing
                }

                if back_button.is_clicked() {
                    game_state = GameState::Menu
                }
            }

            GameState::Playing => {
                physics::physics_handle(
                    &mut player,
                    &mut velocity_y,
                    gravity,
                    jump_force,
                    &mut on_ground,
                    &mut rotation
                );

                if is_key_pressed(KeyCode::Backspace) {
                    game_state = GameState::LevelSelect
                }
            }

            GameState::CreatorMenu => {
                back_button.update(delta_time);

                featured_button.update(delta_time);
                create_button.update(delta_time);
                search_button.update(delta_time);

                featured_button.rect.x = screen_width() / 2.0 - 100.0;
                featured_button.rect.y = screen_height() / 2.0 - 100.0;

                create_button.rect.x = screen_width() / 2.0 - 310.0;
                create_button.rect.y = screen_height() / 2.0 - 100.0;

                search_button.rect.x = screen_width() / 2.0 + 110.0;
                search_button.rect.y = screen_height() / 2.0 - 100.0;

                if back_button.is_clicked() {
                    game_state = GameState::Menu
                }

                if create_button.is_clicked() {
                    game_state = GameState::Editor
                }
            }

            GameState::Editor => {
                back_button.update(delta_time);
                build_tab_button.update(delta_time);
                edit_tab_button.update(delta_time);

                build_tab_button.rect.y = screen_height() - 190.0;
                edit_tab_button.rect.y = screen_height() - 100.0;

                if back_button.is_clicked() {
                    game_state = GameState::CreatorMenu
                }

                if build_tab_button.is_clicked() {
                    current_tab = 1;
                    build_tab_button.is_disabled = false;
                    edit_tab_button.is_disabled = true;
                }

                if edit_tab_button.is_clicked() {
                    current_tab = 2;
                    build_tab_button.is_disabled = true;
                    edit_tab_button.is_disabled = false;
                }

                editor::keybind_handler(
                    &mut cam_pos_y
                );
            }
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

                draw_text_pro(
                    &format!("Version: {}", version),
                    20.0,
                    40.0,
                    20,
                    RED,
                    &font
                );

                draw_text_pro(
                    &format!("Latest Version: {}", latest_version),
                    20.0,
                    80.0,
                    20,
                    RED,
                    &font
                );

                play_button.draw(false, None, 1.0, false, &font);
                creator_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelSelect => {
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

                back_button.draw(false, None, 1.0, false, &font);
            }

            GameState::Playing => {
                clear_background(BLACK);
                draw_texture_ex(
                    &default_bg,
                    0.0,
                    0.0,
                    Color::from_rgba(0, 0, 50, 255),
                    DrawTextureParams {
                        dest_size: None,
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None
                    }
                );

                draw_rectangle_ex(
                    player.x,
                    player.y,
                    player.w,
                    player.h,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation,
                        color: GREEN
                    }
                );

                for i in 0..screen_width() as i32 / 160 + 1 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0,
                        screen_height() / 1.15,
                        Color::from_rgba(0, 0, 100, 255),
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                160.0,
                                160.0
                            )),
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None
                        }
                    );
                }
            }

            GameState::CreatorMenu => {
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

                back_button.draw(false, None, 1.0, false, &font);
                featured_button.draw(false, None, 1.0, false, &font);
                create_button.draw(false, None, 1.0, false, &font);
                search_button.draw(false, None, 1.0, false, &font);
            }

            GameState::Editor => {
                clear_background(BLACK);

                draw_texture_ex(
                    &default_bg,
                    0.0,
                    0.0,
                    Color::from_rgba(0, 0, 50, 255),
                    DrawTextureParams {
                        dest_size: None,
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None
                    }
                );

                for i in 0..screen_width() as i32 / 160 + 1 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0,
                        screen_height() / 1.15 + cam_pos_y,
                        Color::from_rgba(0, 0, 100, 255),
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                160.0,
                                160.0
                            )),
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None
                        }
                    );
                }

                draw_rectangle(
                    0.0,
                    screen_height() - 200.0,
                    screen_width(),
                    200.0,
                    Color::from_rgba(10, 10, 10, 100)
                );

                draw_line(
                    200.0,
                    screen_height() - 200.0,
                    200.0,
                    screen_height(),
                    3.0,
                    WHITE
                );

                if current_tab == 2 {
                    draw_text_pro(
                        "Click on an object to select it!",
                        screen_width() / 2.0 - measure_text_ex("Click on an object to select it!", 20, &font) / 2.0 + 50.0,
                        screen_height() - 100.0,
                        20,
                        WHITE,
                        &font
                    );
                }

                back_button.draw(false, None, 1.0, false, &font);
                build_tab_button.draw(false, None, 1.0, false, &font);
                edit_tab_button.draw(false, None, 1.0, false, &font);
            }
        }

        next_frame().await
    }
}