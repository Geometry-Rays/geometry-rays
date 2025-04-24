use macroquad::prelude::*;
use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;

mod funcs;
mod types;
use funcs::*;
use menu_logic::editor::object_ped;
use menu_logic::physics::hitbox_draw;
use types::*;

mod menu_logic;
use menu_logic::*;

#[macroquad::main("Geometry Rays")]
async fn main() {
    // This just loads the font used for the game.
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

    let mut editor_back_button = Button::new(
        screen_width() - 160.0,
        20.0,
        150.0,
        75.0,
        "Back",
        15,
        false
    );

    // Url's for server requests
    let main_url = "http://georays.puppet57.xyz/php-code/".to_string();
    let latest_version_url: String = format!("{}get-latest-version.php", main_url).to_string();

    // Important game variables
    let mut game_state: GameState = GameState::Menu;
    let mut player: Rect = Rect { x: 200.0, y: screen_height() / 1.15, w: 40.0, h: 40.0 };
    let mut centered_player: Rect = Rect { x: 0.0, y: 0.0, w: 40.0, h: 40.0 };
    let mut small_player = player;
    let mut on_ground: bool = true;
    let mut touching_block_ceiling: bool = false;
    let mut obj_grid: Vec<ObjectStruct> = vec![];
    let mut debug_mode: bool = false;
    let mut world_offset: f32 = 0.0;
    let player_cam_y: f32 = 0.0;
    let mut kill_player: bool = false;

    let obj_btn_offset: f32 = 70.0;
    let mut obj_types: Vec<ObjectType> = vec![
        ObjectType::new(
            1,
            "Spike",
            load_texture("./Resources/objects/spike.png")
                .await.expect("Failed to load spike texture"),
            obj_btn_offset
        ),

        ObjectType::new(
            2,
            "Block",
            load_texture("./Resources/objects/block.png")
                .await.expect("Failed to load block texture"),
            obj_btn_offset
        ),

        ObjectType::new(
            3,
            "Jump Pad",
            load_texture("./Resources/objects/pads/pad.png")
                .await.expect("Failed to load pad texture"),
            obj_btn_offset
        ),
    ];

    // Physics values
    let mut velocity_y: f32 = 0.0;
    let gravity: f32 = 1.0;
    let jump_force: f32 = 16.0;
    let mut rotation: f32 = 0.0;
    let movement_speed: f32 = 6.0;

    // Editor variables
    let mut current_tab: u8 = 1;
    let mut cam_pos_y: f32 = 0.0;
    let mut cam_pos_x: f32 = 0.0;
    let mut current_obj: u16 = 1;
    let grid_size: u8 = 40;

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
        // This is so if you hit escape in the game then the game loop stops
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let delta_time: f32 = get_frame_time();

        let mouse_x = mouse_position().0 as i32;
        let mouse_y = mouse_position().1 as i32;
        let snapped_cam_x = cam_pos_x as i32;
        let snapped_cam_y = cam_pos_y as i32;
        let snapped_x = ((mouse_x + (snapped_cam_x * 5)) / grid_size as i32) * grid_size as i32;
        let snapped_y = ((mouse_y - (snapped_cam_y * 5)) / grid_size as i32) * grid_size as i32;

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

                if is_key_pressed(KeyCode::Slash) {
                    debug_mode = true;
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
                centered_player = Rect {
                    x: player.x - 20.0,
                    y: player.y - 20.0,
                    w: player.w,
                    h: player.h
                };

                small_player = centered_player;
                small_player.x = centered_player.x + 15.0;
                small_player.y = centered_player.y + 10.0;
                small_player.w = 20.0;
                small_player.h = 20.0;

                // The function for handling the physics of the game
                physics::physics_handle(
                    &mut player,
                    &mut velocity_y,
                    gravity,
                    jump_force,
                    &mut on_ground,
                    &mut rotation,
                    &mut world_offset,
                    movement_speed
                );

                physics::hitbox_collision(
                    &mut player,
                    centered_player,
                    small_player,
                    &mut rotation,
                    &obj_grid,
                    world_offset,
                    player_cam_y,
                    &mut velocity_y,
                    gravity,
                    &mut kill_player,
                    &mut on_ground,
                    &mut touching_block_ceiling
                );

                if kill_player {
                    player.y = screen_height() / 1.15;
                    world_offset = 0.0;
                    kill_player = false
                }

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
                editor_back_button.update(delta_time);
                build_tab_button.update(delta_time);
                edit_tab_button.update(delta_time);

                build_tab_button.rect.y = screen_height() - 190.0;
                edit_tab_button.rect.y = screen_height() - 100.0;

                editor_back_button.rect.x = screen_width() - 160.0;

                for object in &mut obj_types {
                    object.button.update(delta_time);
                }

                for object in &obj_types {
                    if object.button.is_clicked()
                    && current_tab == 1 {
                        current_obj = object.id
                    }
                }

                if editor_back_button.is_clicked() {
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

                if mouse_position().1 < screen_height() - 200.0
                && is_mouse_button_pressed(MouseButton::Left)
                && !editor_back_button.rect.contains(mouse_position().into()) {
                    object_ped(
                        &mut obj_grid,
                        snapped_x,
                        snapped_y,
                        current_tab,
                        current_obj
                    );
                }

                // All the keybinds for the editor are in this function
                editor::keybind_handler(
                    &mut cam_pos_y,
                    &mut cam_pos_x
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

                for object in &obj_grid {
                    let obj_y = (screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0);
                    draw_texture_ex(
                        &obj_types[object.id as usize - 1].texture,
                        object.x as f32 - world_offset as f32,
                        obj_y + 6.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                obj_types[object.id as usize - 1].texture.width() * 0.05,
                                obj_types[object.id as usize - 1].texture.height() * 0.05
                            )),
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: Some(vec2(0.5, 0.5))
                        }
                    );
                }

                // Draws the ground
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

                if debug_mode {
                    hitbox_draw(
                        centered_player,
                        small_player,
                        &obj_grid,
                        world_offset,
                        player_cam_y
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

                // Draws the background
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

                // Draws the ground
                for i in 0..screen_width() as i32 / 160 + 1 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0,
                        screen_height() / 1.15 + cam_pos_y * 5.0 - 7.0,
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

                for object in &obj_grid {
                    let obj_y = (screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0);
                    draw_texture_ex(
                        &obj_types[object.id as usize - 1].texture,
                        object.x as f32 - cam_pos_x * 5.0,
                        obj_y + cam_pos_y * 5.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                obj_types[object.id as usize - 1].texture.width() * 0.05,
                                obj_types[object.id as usize - 1].texture.height() * 0.05
                            )),
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: Some(vec2(0.5, 0.5))
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

                for object in &obj_types {
                    if current_tab == 1 {
                        object.button.draw(
                            true,
                            Some(&&object.texture),
                            0.04,
                            true,
                            &font
                        );
                    }
                }

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

                draw_text_pro(
                    &format!("Selected Object: {}", obj_types[current_obj as usize - 1].name),
                    10.0,
                    30.0,
                    20,
                    WHITE,
                    &font
                );

                if debug_mode {
                    draw_text_pro(
                        &format!("Cam x: {}", cam_pos_x),
                        10.0,
                        70.0,
                        20,
                        GREEN,
                        &font
                    );

                    draw_text_pro(
                        &format!("Cam y: {}", cam_pos_y),
                        10.0,
                        110.0,
                        20,
                        GREEN,
                        &font
                    );

                    draw_rectangle_lines(
                        snapped_x as f32,
                        snapped_y as f32,
                        grid_size as f32,
                        grid_size as f32,
                        2.0,
                        WHITE
                    );
                }

                editor_back_button.draw(false, None, 1.0, false, &font);
                build_tab_button.draw(false, None, 1.0, false, &font);
                edit_tab_button.draw(false, None, 1.0, false, &font);
            }
        }

        next_frame().await
    }
}