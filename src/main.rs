use game::loading::load_level;
use game::playing::physics::ship_physics;
use macroquad::prelude::*;

use gr_rodio::rodio_raw::OutputStream;

use gr_rodio::*;

mod funcs;
mod types;
use funcs::*;
use types::*;

mod game;
use game::*;

#[macroquad::main("Geometry Rays")]
async fn main() {
    // This just loads the font used for the game.
    let font: Font = load_ttf_font("./Resources/Acme 9 Regular.ttf").await.unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio_raw::Sink::try_new(&stream_handle).unwrap();

    // Buttons
    let mut play_button = Button::new(
        || screen_width() as f32 / 2.0 - 100.0,
        || screen_height() as f32 / 2.0 - 50.0,
        || 200.0,
        || 100.0,
        "Play",
        20,
        false
    );

    let mut creator_button = Button::new(
        || screen_width() as f32 / 2.0 - 100.0,
        || screen_height() as f32 / 2.0 + 70.0,
        || 200.0,
        || 100.0,
        "Custom Levels",
        15,
        false
    );

    let mut back_button = Button::new(
        || 20.0,
        || 20.0,
        || 150.0,
        || 75.0,
        "Back",
        15,
        false
    );

    let creator_menu_button_gap: f32 = 115.0;

    let mut featured_button = Button::new(
        || screen_width() / 2.0 - 100.0,
        || screen_height() / 2.0 - 100.0,
        || 200.0,
        || 200.0,
        "Featured",
        20,
        true
    );

    let mut create_button = Button::new(
        move || screen_width() / 2.0 - creator_menu_button_gap - 200.0,
        || screen_height() / 2.0 - 100.0,
        || 200.0,
        || 200.0,
        "Create",
        20,
        false
    );

    let mut search_button = Button::new(
        move || screen_width() / 2.0 + creator_menu_button_gap,
        || screen_height() / 2.0 - 100.0,
        || 200.0,
        || 200.0,
        "Search",
        20,
        true
    );

    let mut build_tab_button = Button::new(
        || 15.0,
        || screen_height() - 190.0,
        || 170.0,
        || 80.0,
        "Build",
        20,
        false
    );

    let mut edit_tab_button = Button::new(
        || 15.0,
        || screen_height() - 100.0,
        || 170.0,
        || 80.0,
        "Edit",
        20,
        true
    );

    let mut editor_back_button = Button::new(
        || screen_width() - 160.0,
        || 20.0,
        || 150.0,
        || 75.0,
        "Back",
        15,
        false
    );

    let mut editor_save_button = Button::new(
        || screen_width() - 160.0,
        || 105.0,
        || 150.0,
        || 75.0,
        "Save",
        15,
        false
    );

    let mut editor_options_button = Button::new(
        || screen_width() - 160.0,
        || 190.0,
        || 150.0,
        || 75.0,
        "Options",
        15,
        false
    );

    let mut editor_playtest_button = Button::new(
        || 20.0,
        || screen_height() / 2.0 - 65.0,
        || 130.0,
        || 130.0,
        "Playtest",
        15,
        false
    );

    let mut bg_red_textbox = TextBox {
        rect: Rect {
            x: screen_width() - 120.0,
            y: 10.0,
            w: 110.0,
            h: 50.0
        },
        text: "Red".to_string(),
        text_size: 25,
        max_length: 3,
        spaces_allowed: false,
        active: false
    };

    let mut bg_green_textbox = TextBox {
        rect: Rect {
            x: screen_width() - 240.0,
            y: 10.0,
            w: 110.0,
            h: 50.0
        },
        text: "Green".to_string(),
        text_size: 20,
        max_length: 3,
        spaces_allowed: false,
        active: false
    };

    let mut bg_blue_textbox = TextBox {
        rect: Rect {
            x: screen_width() - 360.0,
            y: 10.0,
            w: 110.0,
            h: 50.0
        },
        text: "Blue".to_string(),
        text_size: 25,
        max_length: 3,
        spaces_allowed: false,
        active: false
    };

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
    let mut on_orb: bool = false;
    let mut current_gamemode: GameMode = GameMode::Cube;
    let master_volume: f32 = 2.0;

    let obj_btn_offset: f32 = 70.0;
    let mut obj_types: Vec<ObjectType> = vec![];
    object_types::create_object_types(&mut obj_types, obj_btn_offset).await;

    // Physics values
    let mut velocity_y: f32 = 0.0;
    let mut gravity: f32 = 1.0;
    let default_gravity: f32 = gravity;
    let mut jump_force: f32 = 16.0;
    let default_jump_force: f32 = jump_force;
    let mut rotation: f32 = 0.0;
    let movement_speed: f32 = 6.0;
    let ship_power: f32 = 0.7;
    let ship_falling_speed: f32 = 0.5;

    // Editor variables
    let mut current_tab: u8 = 1;
    let mut cam_pos_y: f32 = 0.0;
    let mut cam_pos_x: f32 = 0.0;
    let mut current_obj: u16 = 1;
    let grid_size: u8 = 40;
    let mut been_to_editor: bool = false;

    // More variables
    let version: &str = "F-ALPHA";
    let level_version: &str = "F-ALPHA";
    let latest_version: String = ureq::get(latest_version_url)
        .query("fyre", "fyre")
        .call()
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    let default_level: &str = &format!(
        "version:{};song:./Resources/Music/main-level-songs/0.mp3;cc_1001:0,0,0.2;cc_1002:0,0,0.3;;;x:400;y:480;rot:0;id:1",
        level_version
    );
    let main_levels: Vec<MainLevel> = vec![
        MainLevel::new(
            "Plummet",
            1,
            "./Resources/Music/main-level-songs/0.mp3",
            "1f1n1ty",
            "Puppet",
            std::fs::read_to_string("./Resources/main-levels/0.txt").unwrap()
        ),

        MainLevel::new(
            "Color Blockade",
            3,
            "./Resources/Music/main-level-songs/1.mp3",
            "Waterflame",
            "Puppet",
            std::fs::read_to_string("./Resources/main-levels/1.txt").unwrap()
        ),

        MainLevel::new(
            "Ultimate Destruction",
            2,
            "./Resources/Music/main-level-songs/2.mp3",
            "TMM43",
            "Puppet",
            std::fs::read_to_string("./Resources/main-levels/2.txt").unwrap()
        ),
    ];
    let mut current_level: u8 = 0;
    let mut current_song: String = main_levels[0].song.to_string();
    let mut current_song_index: u8 = 0;
    let hidden_obj_types: Vec<u16> = vec![
        15
    ];

    let mut cc_1001: Color = Color::new(0.0, 0.0, 0.2, 1.0);
    let mut cc_1002: Color = Color::new(0.0, 0.0, 0.3, 1.0);
    let mut cc_1003: Color = GREEN;

    // Textures
    let default_bg_no_gradient = load_texture("./Resources/default-bg-no-gradient.png")
        .await.expect("Failed to load background texture");
    let default_bg = load_texture("./Resources/default-bg.png")
        .await.expect("Failed to load background texture");
    let grnd_texture = load_texture("./Resources/ground.png")
        .await.expect("Failed to load ground texture");

    let mut difficulties: Vec<Texture2D> = vec![];

    for i in 0..10 {
        difficulties.push(
            load_texture(&format!("./Resources/difficulties/{}.png", i))
                .await.expect("Failed to load difficulty face")
        );
    }

    // This handles changing level.txt to the default level if it isn't already a level
    match std::fs::read_to_string("./save-data/level.txt") {
        Ok(level_file) => {
            if !level_file.starts_with("version:") {
                let _ = std::fs::write(
                    "./save-data/level.txt",
                    default_level
                );
            }
        }

        Err(error) => {
            println!("{}", error);
        }
    }

    // Values for textboxes
    let mut bg_red: String = "".to_string();
    let mut bg_green: String = "".to_string();
    let mut bg_blue: String = "".to_string();

    play_audio_path("Resources/Music/menu-music.mp3", master_volume, &sink);
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
        let screen_height_range = (screen_height() - 600.0) * (40.0 / (1005.0 - 600.0));
        let snapped_y = (((mouse_y - (snapped_cam_y * 5)) - (screen_height() - (600.0 + screen_height_range)) as i32) / grid_size as i32) * grid_size as i32;

        match game_state {
            GameState::Menu => {
                play_button.update(delta_time);
                creator_button.update(delta_time);

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
                    let load_level_result = load_level(
                        main_levels[current_level as usize].data.clone(),
                        &mut obj_grid,
                        &mut cc_1001,
                        &mut cc_1002,
                        &mut current_song,
                        false
                    );

                    stop_audio(&sink);
                    play_audio_path(
                        &main_levels[current_level as usize].song,
                        master_volume,
                        &sink
                    );

                    if load_level_result == "ok" {
                        game_state = GameState::Playing
                    } else {
                        println!("Problem loading level: {}", load_level_result);
                    }
                }

                if back_button.is_clicked() {
                    game_state = GameState::Menu
                }

                if is_key_pressed(KeyCode::Left) && current_level > 0 {
                    current_level -= 1;
                }

                if is_key_pressed(KeyCode::Right) && current_level < main_levels.len() as u8 - 1 {
                    current_level += 1;
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
                playing::physics::physics_handle(
                    &mut player,
                    &mut velocity_y,
                    &mut on_ground,
                    &mut rotation,
                    &mut world_offset,
                    movement_speed
                );

                playing::hitboxes::hitbox_collision(
                    &mut player,
                    centered_player,
                    small_player,
                    &mut rotation,
                    &obj_grid,
                    world_offset,
                    player_cam_y,
                    &mut velocity_y,
                    &mut gravity,
                    default_gravity,
                    &mut jump_force,
                    default_jump_force,
                    &mut kill_player,
                    &mut on_ground,
                    &mut touching_block_ceiling,
                    &mut on_orb,
                    &mut current_gamemode,
                    &mut cc_1003,
                    &mut game_state
                );

                match current_gamemode {
                    GameMode::Cube => {
                        playing::physics::cube_physics(
                            &mut velocity_y,
                            gravity,
                            &mut on_ground,
                            jump_force
                        );
                    }

                    GameMode::Ship => {
                        ship_physics(
                            touching_block_ceiling,
                            gravity,
                            &mut velocity_y,
                            ship_power,
                            ship_falling_speed
                        );
                    }
                }

                if kill_player {
                    player.y = screen_height() / 1.15;
                    world_offset = 0.0;
                    kill_player = false;
                    restart_audio(&sink);
                }

                if is_mouse_button_released(MouseButton::Left) ||
                is_key_released(KeyCode::Space) {
                    on_orb = true
                }

                if is_key_pressed(KeyCode::Backspace) {
                    stop_audio(&sink);
                    play_audio_path("Resources/Music/menu-music.mp3", master_volume, &sink);
                    game_state = GameState::LevelSelect
                }
            }

            GameState::CreatorMenu => {
                back_button.update(delta_time);

                featured_button.update(delta_time);
                create_button.update(delta_time);
                search_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state = GameState::Menu
                }

                if create_button.is_clicked() {
                    if !been_to_editor {
                        let level_data: String = std::fs::read_to_string("./save-data/level.txt").unwrap();

                        loading::load_level(
                            level_data,
                            &mut obj_grid,
                            &mut cc_1001,
                            &mut cc_1002,
                            &mut current_song,
                            true
                        );
                    }

                    been_to_editor = true;
                    game_state = GameState::Editor
                }
            }

            GameState::Editor => {
                editor_back_button.update(delta_time);
                editor_save_button.update(delta_time);
                editor_options_button.update(delta_time);
                build_tab_button.update(delta_time);
                edit_tab_button.update(delta_time);
                editor_playtest_button.update(delta_time);

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

                if editor_options_button.is_clicked() {
                    game_state = GameState::LevelSettings
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

                if editor_playtest_button.is_clicked() {
                    stop_audio(&sink);
                    play_audio_path(&current_song, master_volume, &sink);
                    game_state = GameState::Playing
                }

                if editor_save_button.is_clicked() {
                    println!("Saving level...");

                    let level_string: String = saving::level_to_string(
                        &obj_grid,
                        level_version,
                        cc_1001,
                        cc_1002,
                        current_song.clone()
                    );

                    let save_result: Result<(), std::io::Error> = std::fs::write(
                        "./save-data/level.txt",
                        level_string
                    );

                    match save_result {
                        Ok(_) => {
                            println!("Saving successful!");
                        }

                        Err(error) => {
                            println!("Error while saving level: {}", error);
                        }
                    }
                }

                if mouse_position().1 < screen_height() - 200.0
                && is_mouse_button_pressed(MouseButton::Left)
                && !editor_back_button.rect.contains(mouse_position().into())
                && !editor_playtest_button.rect.contains(mouse_position().into())
                && !editor_save_button.rect.contains(mouse_position().into()) {
                    editor::object_ped(
                        &mut obj_grid,
                        snapped_x,
                        snapped_y,
                        cam_pos_x,
                        cam_pos_y,
                        grid_size,
                        current_tab,
                        current_obj
                    );
                }

                // All the keybinds for the editor are in this function
                editor::keybind_handler(
                    &mut cam_pos_y,
                    &mut cam_pos_x,
                    &mut obj_grid
                );
            }

            GameState::LevelComplete => {
                back_button.update(delta_time);

                if back_button.is_clicked() {
                    stop_audio(&sink);
                    play_audio_path("Resources/Music/menu-music.mp3", master_volume, &sink);
                    game_state = GameState::Menu
                }
            }

            GameState::LevelSettings => {
                back_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state = GameState::Editor
                }

                if is_key_pressed(KeyCode::Left) && current_song_index > 0 {
                    current_song_index -= 1;
                    current_song = main_levels[current_song_index as usize].song.clone();
                }

                if is_key_pressed(KeyCode::Right) && current_song_index < main_levels.len() as u8 - 1 {
                    current_song_index += 1;
                    current_song = main_levels[current_song_index as usize].song.clone();
                }

                if bg_red_textbox.is_clicked() {
                    bg_red_textbox.active = true
                }

                if bg_red_textbox.is_not_clicked() {
                    bg_red_textbox.active = false
                }

                if bg_green_textbox.is_clicked() {
                    bg_green_textbox.active = true
                }

                if bg_green_textbox.is_not_clicked() {
                    bg_green_textbox.active = false
                }

                if bg_blue_textbox.is_clicked() {
                    bg_blue_textbox.active = true
                }

                if bg_blue_textbox.is_not_clicked() {
                    bg_blue_textbox.active = false
                }

                bg_red_textbox.input(&mut bg_red);
                bg_green_textbox.input(&mut bg_green);
                bg_blue_textbox.input(&mut bg_blue);
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

                draw_text_pro(
                    &format!("Creator: {}", &main_levels[current_level as usize].creator),
                    10.0,
                    screen_height() - 35.0,
                    15,
                    RED,
                    &font
                );

                draw_text_pro(
                    &format!("Artist: {}", &main_levels[current_level as usize].artist),
                    10.0,
                    screen_height() - 10.0,
                    15,
                    RED,
                    &font
                );

                draw_text_pro(
                    &main_levels[current_level as usize].name,
                    screen_width() / 2.0 - measure_text_ex(
                        &main_levels[current_level as usize].name,
                        40,
                        &font
                    ) / 2.0,
                    100.0,
                    40,
                    RED,
                    &font
                );

                draw_text_pro(
                    &main_levels[current_level as usize].difficulty.to_string(),
                    200.0,
                    350.0,
                    40,
                    RED,
                    &font
                );

                draw_texture_ex(
                    &difficulties[main_levels[current_level as usize].difficulty as usize],
                    0.0,
                    40.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            400.0,
                            400.0
                        )),
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
                    cc_1001,
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
                        color: cc_1003
                    }
                );

                for object in &obj_grid {
                    let obj_y = (screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0);

                    if !hidden_obj_types.contains(&object.id) {
                        draw_texture_ex(
                            &obj_types[object.id as usize - 1].texture,
                            object.x as f32 - if object.id == 8 || object.id == 9 { 40.0 } else { 0.0 } - world_offset as f32,
                            obj_y + 6.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(
                                    obj_types[object.id as usize - 1].texture.width() * 0.05,
                                    obj_types[object.id as usize - 1].texture.height() * 0.05
                                )),
                                source: None,
                                rotation: (object.rotation as f64 * std::f64::consts::PI / 180.0) as f32,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                }

                // Draws the ground
                for i in 0..screen_width() as i32 / 160 + 1 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0,
                        screen_height() / 1.15,
                        cc_1002,
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
                    playing::hitboxes::hitbox_draw(
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
                    cc_1001,
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
                        cc_1002,
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
                        object.x as f32 - if object.id == 8 || object.id == 9 { 40.0 } else { 0.0 } - cam_pos_x * 5.0,
                        obj_y + cam_pos_y * 5.0,
                        if object.selected { GREEN } else { WHITE },
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                obj_types[object.id as usize - 1].texture.width() * 0.05,
                                obj_types[object.id as usize - 1].texture.height() * 0.05
                            )),
                            source: None,
                            rotation: (object.rotation as f64 * std::f64::consts::PI / 180.0) as f32,
                            flip_x: false,
                            flip_y: false,
                            pivot: None
                        }
                    );

                    if debug_mode {
                        draw_circle(
                            object.x as f32 - cam_pos_x * 5.0 + grid_size as f32 / 2.0,
                            obj_y + cam_pos_y * 5.0 + grid_size as f32 / 2.0,
                            5.0,
                            RED
                        );
                    }
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

                    draw_text_pro(
                        &format!("Snapped x: {}", snapped_x),
                        10.0,
                        150.0,
                        20,
                        GREEN,
                        &font
                    );

                    draw_text_pro(
                        &format!("Snapped y: {}", snapped_y),
                        10.0,
                        190.0,
                        20,
                        GREEN,
                        &font
                    );

                    draw_rectangle_lines(
                        snapped_x as f32 - cam_pos_x * 5.0,
                        snapped_y as f32 + cam_pos_y * 5.0,
                        grid_size as f32,
                        grid_size as f32,
                        2.0,
                        WHITE
                    );

                    draw_rectangle(
                        mouse_position().0 - grid_size as f32 / 2.0,
                        mouse_position().1 - grid_size as f32 / 2.0,
                        grid_size as f32,
                        grid_size as f32,
                        Color::from_rgba(0, 255, 0, 150)
                    );
                }

                editor_back_button.draw(false, None, 1.0, false, &font);
                editor_save_button.draw(false, None, 1.0, false, &font);
                editor_options_button.draw(false, None, 1.0, false, &font);
                build_tab_button.draw(false, None, 1.0, false, &font);
                edit_tab_button.draw(false, None, 1.0, false, &font);
                editor_playtest_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelComplete => {
                clear_background(BLACK);

                draw_texture_ex(
                    &default_bg_no_gradient,
                    -50.0,
                    -75.0,
                    Color::from_rgba(0, 50, 0, 255),
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
                    "Level Complete!",
                    screen_width() / 2.0 - measure_text_ex("Level Complete!", 40, &font) / 2.0,
                    200.0,
                    40,
                    RED,
                    &font
                );

                back_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelSettings => {
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
                    &main_levels[current_song_index as usize].name,
                    screen_width() / 2.0 - measure_text_ex(&main_levels[current_song_index as usize].name, 30, &font) / 2.0,
                    screen_height() - 30.0,
                    30,
                    WHITE,
                    &font
                );

                bg_red_textbox.draw(bg_red.clone(), &font);
                bg_green_textbox.draw(bg_green.clone(), &font);
                bg_blue_textbox.draw(bg_blue.clone(), &font);

                back_button.draw(false, None, 1.0, false, &font);
            }
        }

        next_frame().await
    }
}