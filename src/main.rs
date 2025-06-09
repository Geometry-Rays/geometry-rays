use std::{cell::Cell, collections::HashMap, rc::Rc};

use game::{loading::load_level, parsing::parse_level_download_response};
use macroquad::prelude::*;
use miniquad::conf::Icon;
use std::convert::TryInto;

use gr_rodio::rodio_raw::OutputStream;

use gr_rodio::*;

mod funcs;
mod types;
use funcs::*;
use mlua::{Function, Table};
use types::*;

mod game;
use game::*;

fn window_conf() -> Conf {
    let small_vec = get_resized_rgba_bytes("./Resources/logo.png", 16).unwrap();
    let medium_vec = get_resized_rgba_bytes("./Resources/logo.png", 32).unwrap();
    let big_vec = get_resized_rgba_bytes("./Resources/logo.png", 64).unwrap();

    let small: [u8; 16*16*4] = small_vec.try_into().unwrap();
    let medium: [u8; 32*32*4] = medium_vec.try_into().unwrap();
    let big: [u8; 64*64*4] = big_vec.try_into().unwrap();

    Conf {
        window_title: "Geometry Rays".into(),
        icon: Some(Icon { small, medium, big }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
        false
    );

    let mut editor_keybinds_button = Button::new(
        || screen_width() - 210.0,
        || 10.0,
        || 200.0,
        || 100.0,
        "Editor Keybinds",
        15,
        false
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

    let mut editor_upload_button = Button::new(
        || screen_width() - 160.0,
        || 275.0,
        || 150.0,
        || 75.0,
        "Upload",
        15,
        false
    );

    let mut edit_obj_button = Button::new(
        || screen_width() - 329.0,
        || 20.0,
        || 150.0,
        || 75.0,
        "Edit Obj",
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

    let mut bg_red_textbox = TextBox::new(
        || screen_width() - 360.0,
        || 10.0,
        || 110.0,
        || 50.0,
        "Red",
        25,
        3,
        false
    );

    let mut bg_green_textbox = TextBox::new(
        || screen_width() - 240.0,
        || 10.0,
        || 110.0,
        || 50.0,
        "Green",
        20,
        3,
        false
    );

    let mut bg_blue_textbox = TextBox::new(
        || screen_width() - 120.0,
        || 10.0,
        || 110.0,
        || 50.0,
        "Blue",
        25,
        3,
        false
    );

    let mut grnd_red_textbox = TextBox::new(
        || screen_width() - 360.0,
        || 80.0,
        || 110.0,
        || 50.0,
        "Red",
        25,
        3,
        false
    );

    let mut grnd_green_textbox = TextBox::new(
        || screen_width() - 240.0,
        || 80.0,
        || 110.0,
        || 50.0,
        "Green",
        20,
        3,
        false
    );

    let mut grnd_blue_textbox = TextBox::new(
        || screen_width() - 120.0,
        || 80.0,
        || 110.0,
        || 50.0,
        "Blue",
        25,
        3,
        false
    );

    let mut level_download_button = Button::new(
        || screen_width() - 140.0,
        || 75.0,
        || 130.0,
        || 75.0,
        "Download",
        15,
        false
    );

    let mut level_id_textbox = TextBox::new(
        || screen_width() - 140.0,
        || 10.0,
        || 130.0,
        || 55.0,
        "Level ID",
        18,
        6,
        false,
    );



    let mut level_play_button = Button::new(
        || screen_width() as f32 / 2.0 - 100.0,
        || screen_height() as f32 / 2.0 - 50.0,
        || 200.0,
        || 100.0,
        "Play",
        20,
        false
    );

    let mut account_button = Button::new(
        || screen_width() as f32 - 210.0,
        || 10.0,
        || 200.0,
        || 100.0,
        "Account",
        20,
        false
    );

    let mut username_textbox = TextBox::new(
        || screen_width() / 2.0 - (20.0 * 18.0) / 2.0,
        || screen_height() / 2.0 - 27.5 - 32.5,
        || 20.0 * 18.0,
        || 55.0,
        "Username",
        18,
        20,
        false
    );

    let mut password_textbox = TextBox::new(
        || screen_width() / 2.0 - (20.0 * 18.0) / 2.0,
        || screen_height() / 2.0 - 27.5 + 32.5,
        || 20.0 * 18.0,
        || 55.0,
        "Password",
        18,
        20,
        false
    );

    let mut login_button: Button = Button::new(
        || screen_width() / 2.0 - 100.0,
        || screen_height() - 200.0,
        || 200.0,
        || 100.0,
        "Login",
        20,
        false
    );

    let mut level_name_textbox = TextBox::new(
        || screen_width() as f32 / 2.0 - 20.0 * 30.0 / 1.9 / 2.0,
        || screen_height() as f32 / 2.0 - 50.0,
        || 20.0 * 30.0 / 1.9,
        || 50.0,
        "Name",
        20,
        20,
        true
    );

    let mut level_desc_textbox = TextBox::new(
        || screen_width() as f32 / 2.0 - 50.0 * 30.0 / 1.9 / 2.0,
        || screen_height() as f32 / 2.0 + 50.0,
        || 50.0 * 30.0 / 1.9,
        || 50.0,
        "Description",
        20,
        50,
        true
    );

    let mut upload_button: Button = Button::new(
        || screen_width() / 2.0 - 100.0,
        || screen_height() - 150.0,
        || 200.0,
        || 100.0,
        "Upload",
        20,
        false
    );

    let mut plat_classic_button = Button::new(
        || 20.0,
        || screen_height() - 170.0,
        || 150.0,
        || 150.0,
        "Classic",
        15,
        false
    );

    let mut bg_color_button = Button::new(
        || 20.0,
        || screen_height() - 170.0,
        || 150.0,
        || 150.0,
        "Bg",
        15,
        false
    );

    let mut grnd_color_button = Button::new(
        || 180.0,
        || screen_height() - 170.0,
        || 150.0,
        || 150.0,
        "Ground",
        15,
        false
    );

    // Url's for server requests
    let main_url = "http://georays.puppet57.xyz/php-code/".to_string();
    let latest_version_url: String = format!("{}get-latest-version.php", main_url).to_string();
    let download_url: String = format!("{}download-level.php", main_url);
    let upload_url: String = format!("{}upload-level.php", main_url).to_string();
    let login_url: String = format!("{}login.php", main_url);

    println!("Defining important game variables..");
    let game_state: Shared<GameState> = Shared::<GameState>(Rc::new(Cell::new(GameState::Menu)));
    let mut player: Rect = Rect { x: 200.0, y: screen_height() / 1.15, w: 40.0, h: 40.0 };
    let mut centered_player: Rect = Rect { x: 0.0, y: 0.0, w: 40.0, h: 40.0 };
    let mut small_player = player;
    let mut on_ground: bool = true;
    let mut touching_block_ceiling: bool = false;
    let mut obj_grid: Vec<ObjectStruct> = vec![];
    let mut debug_mode: bool = false;
    let mut world_offset: f32 = 0.0;
    let mut player_cam_y: f32 = 0.0;
    let mut kill_player: bool = false;
    let mut on_orb: bool = false;
    let mut current_gamemode: GameMode = GameMode::Cube;
    let master_volume: f32 = 2.0;

    println!("Defining object types..");
    let obj_btn_offset: f32 = 65.0;
    let mut obj_types: HashMap<u16, ObjectType> = HashMap::new();
    object_types::create_object_types(&mut obj_types, obj_btn_offset);
    println!("Last object id: {}", obj_types.len());

    println!("Defining physics values..");
    let velocity_y: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(0.0)));
    let gravity: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(1.0)));
    let default_gravity: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(gravity.0.get())));
    let jump_force: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(16.0)));
    let default_jump_force: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(jump_force.0.get())));
    let mut rotation: f32 = 0.0;
    let movement_speed: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(6.0)));
    let default_movement_speed: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(movement_speed.0.get())));
    let ship_power: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(0.7)));
    let ship_falling_speed: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(0.5)));
    let vertical_wave_speed: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(1.1)));
    let game_tps: Shared<f32> = Shared::<f32>(Rc::new(Cell::new(60.0)));

    println!("Setting up editor stuff..");
    let mut current_tab: u8 = 1;
    let mut cam_pos_y: f32 = 0.0;
    let mut cam_pos_x: f32 = 0.0;
    let mut current_obj: u16 = 1;
    let mut selected_obj: u16 = 0;
    let grid_size: u8 = 40;
    let mut level_mode: u8 = 1;
    let mut level_options_type: u8 = 1;

    println!("Getting latest version...");
    let version: &str = "F-1.0.2";
    let level_version: &str = "F-ALPHA";
    let latest_version: String = ureq::get(latest_version_url)
        .query("fyre", "fyre")
        .call()
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    println!("Preparing more values...");
    let default_level: &str = &format!(
        "version:{};song:./Resources/Music/main-level-songs/0.mp3;mode:1;cc_1001:0,0,0.392;cc_1002:0,0,0.392;;;x:400;y:480;rot:0;id:1",
        level_version
    );
    let mut main_levels: Vec<MainLevel> = vec![
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

        MainLevel::new(
            "Detorium",
            4,
            "./Resources/Music/main-level-songs/3.mp3",
            "Fluix",
            "Fluix",
            std::fs::read_to_string("./Resources/main-levels/3.txt").unwrap()
        ),

        MainLevel::new(
            "Foundry",
            2,
            "./Resources/Music/main-level-songs/4.mp3",
            "Fluix",
            "Puppet",
            std::fs::read_to_string("./Resources/main-levels/4.txt").unwrap()
        ),
    ];
    let mut current_level: u8 = 0;
    let mut current_song: String = main_levels[0].song.to_string();
    let mut current_song_index: u8 = 0;
    let hidden_obj_types: Vec<u16> = vec![
        15,
        23
    ];
    let mut on_pad_timer: Timer = Timer::new(0.1);
    let mut on_pad: bool = false;
    let mut player_trail: Vec<Vec2> = vec![];
    let mut stars: u32 = 0;
    let mut username: String = "0".to_string();
    let mut password: String = "0".to_string();
    let mut logged_in: bool = false;
    let mut current_difficulty: u8 = 0;
    let mut bg_offset: f32 = 0.0;
    let mut current_mode: String = "1".to_string();
    let mut online_levels_beaten: Vec<u16> = vec![];

    let mut cc_1001: Color = Color::new(0.0, 0.0, 0.2, 1.0);
    let mut cc_1002: Color = Color::new(0.0, 0.0, 0.3, 1.0);
    let mut cc_1003: Color = GREEN;

    println!("Loading textures...");
    let default_bg_no_gradient = load_texture("./Resources/default-bg-no-gradient.png")
        .await.expect("Failed to load background texture");
    let default_bg = load_texture("./Resources/default-bg.png")
        .await.expect("Failed to load background texture");
    let grnd_texture = load_texture("./Resources/ground.png")
        .await.expect("Failed to load ground texture");
    let star_texture = load_texture("./Resources/star.png")
        .await.expect("Failed to load star texture");

    let mut difficulties: Vec<Texture2D> = vec![];

    // This just puts all the difficulty face textures into a vec
    // This is so the game can easily show difficulties
    println!("Loading difficulty faces...");
    for i in 0..11 {
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

    // This handles changing save.txt to the default save file if it isn't already a save file
    let default_save_file: String = "stars:0;user:0;pass:0;;;0:0;1:0;2:0;3:0;4:0;;;0".to_string();
    match std::fs::read_to_string("./save-data/save.txt") {
        Ok(save_file) => {
            if !save_file.starts_with("stars:") {
                let _ = std::fs::write(
                    "./save-data/save.txt",
                    default_save_file
                );
            }
        }

        Err(error) => {
            println!("{}", error);
        }
    }

    // Values for server responses
    let mut level_download_response: String = "".to_string();
    let mut online_level_name: String = "".to_string();
    let mut online_level_desc: String = "".to_string();
    let mut online_level_data: String = "".to_string();
    let mut online_level_diff: u8 = 0;
    let mut online_level_rated: bool = false;
    let mut online_level_creator: String = "".to_string();
    let mut show_level_not_found: bool = false;
    let mut login_response: String = "".to_string();
    let mut level_upload_response: String = "".to_string();

    let save_file: String = std::fs::read_to_string("./save-data/save.txt")
        .expect("Failed to load save file");

    println!("Loading save data...");
    // Totally not copied from the old client at all
    let values_levels: Vec<&str> = save_file.split(";;;").collect();
    let save_pairs: Vec<&str> = values_levels[0].split(";").collect();
    let levels_completed: Vec<&str> = values_levels[1].split(";").collect();
    let online_levels_completed: Vec<&str> = values_levels[2].split(";").collect();
    for pair in save_pairs {
        let key_value: Vec<&str> = pair.split(":").collect();

        if key_value[0] == "stars" {
            stars = key_value[1].parse::<u32>().unwrap();
        }

        if key_value[0] == "user" {
            if key_value[1] != "0" {
                username = key_value[1].to_string();
            }
        }

        if key_value[0] == "pass" {
            if key_value[1] != "0" {
                password = key_value[1].to_string();
            }
        }
    }

    // This is for checking what main levels you have completed
    let mut level_index: u8 = 0;
    for level in levels_completed {
        let key_value: Vec<&str> = level.split(":").collect();
        if key_value[1] == "1" {
            main_levels[level_index as usize].completed = true
        }

        level_index += 1;
    }

    // This is for checking what online levels you have completed
    for level in online_levels_completed {
        online_levels_beaten.push(level.parse().unwrap());
    }

    // This is for auto login
    // Auto login only runs if you have already logged in using the login page
    if username != "0" && password != "0" {
        println!("Logging in...");
        login_response = ureq::post(&login_url)
            .send_form([
                ("user", &username),
                ("pass", &password),
            ])
            .unwrap()
            .into_body()
            .read_to_string()
            .unwrap();

        if login_response == "Logged in!" {
            logged_in = true;
            // TODO: Add level rating to fyre
            // if username == "Puppet" {
            //     is_mod = true
            // }
        }
    }

    println!("Loading mods...");
    let mod_paths_kinda = std::fs::read_dir("./mods").unwrap();
    let mut mod_contents: Vec<String> = vec![];
    let mut mods: Vec<Table> = vec![];

    // Change this to false to disable the mod loader
    let load_mods: bool = true;

    // This just puts the contents of all the lua files in a vec
    // TODO: Combine the for loops for this and actually loading the mods
    for path in mod_paths_kinda {
        let path_str = path.unwrap().path().to_str().unwrap().to_string();

        let mod_content: String = std::fs::read_to_string(path_str).unwrap();

        mod_contents.push(mod_content.clone());
    }

    let lua = mlua::Lua::new();

    // Exposing stuff to lua
    lua.globals().set("velocity_y", velocity_y.clone()).unwrap();
    lua.globals().set("gravity", gravity.clone()).unwrap();
    lua.globals().set("default_gravity", default_gravity.clone()).unwrap();
    lua.globals().set("jump_force", jump_force.clone()).unwrap();
    lua.globals().set("default_jump_force", default_jump_force.clone()).unwrap();
    lua.globals().set("movement_speed", movement_speed.clone()).unwrap();
    lua.globals().set("default_movement_speed", default_movement_speed.clone()).unwrap();
    lua.globals().set("ship_power", ship_power.clone()).unwrap();
    lua.globals().set("ship_falling_speed", ship_falling_speed.clone()).unwrap();
    lua.globals().set("game_state", game_state.clone()).unwrap();
    lua.globals().set("vertical_wave_speed", vertical_wave_speed.clone()).unwrap();
    lua.globals().set("game_tps", game_tps.clone()).unwrap();

    let font_clone = font.clone();
    let draw_text_lua_func = lua.create_function(move |_, (text, x, y, size, r, g, b, a): (String, f32, f32, u8, u8, u8, u8, u8)| {
        draw_text_pro(&text, x, y, size, Color::from_rgba(r, g, b, a), &font_clone);
        Ok(())
    }).unwrap();

    lua.globals().set("draw_text", draw_text_lua_func).unwrap();

    // Loads the lua mods
    for mod_data in mod_contents {
        let lua_mod: Table = lua.load(mod_data).eval().unwrap();

        mods.push(lua_mod);
    }

    // This runs all of the setup functions of all the mods
    for lua_mod in mods.clone() {
        let active: bool = lua_mod.get("enabled").unwrap();

        if active && load_mods {
            let setup_func: Function = lua_mod.get("setup").unwrap();

            setup_func.call::<()>(()).unwrap();
        }
    }

    println!("Preparing main loop...");
    play_audio_path("Resources/Music/menu-music.mp3", master_volume, true, &sink);
    loop {
        let frame_start = std::time::Instant::now();

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
        let screen_height_range = (screen_height() - 600.0) * (60.0 / (1005.0 - 600.0));
        let snapped_y_bugged = (((mouse_y - (snapped_cam_y * 5)) - (screen_height() - (600.0 + screen_height_range)) as i32) / grid_size as i32) * grid_size as i32;
        let snapped_y: i32 = snapped_y_bugged - if snapped_y_bugged <= -40 { 40 } else { 0 };

        // This runs the loop function of all the loaded mods
        for lua_mod in mods.clone() {
            let active: bool = lua_mod.get("enabled").unwrap();

            if active && load_mods {
                let loop_func: Function = lua_mod.get("loop").unwrap();

                loop_func.call::<()>(()).unwrap();
            }
        }

        match game_state.0.get() {
            GameState::Menu => {
                play_button.update(delta_time);
                creator_button.update(delta_time);
                account_button.update(delta_time);

                if play_button.is_clicked() {
                    game_state.0.set(GameState::LevelSelect)
                }

                if creator_button.is_clicked() {
                    game_state.0.set(GameState::CreatorMenu)
                }

                if account_button.is_clicked() {
                    game_state.0.set(GameState::AccountPage);
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
                        &mut current_mode,
                        &mut current_song,
                        false,
                        main_levels.clone()
                    );

                    stop_audio(&sink);
                    play_audio_path(
                        &main_levels[current_level as usize].song,
                        master_volume,
                        false,
                        &sink
                    );

                    if load_level_result == "ok" {
                        game_state.0.set(GameState::Playing)
                    } else {
                        println!("Problem loading level: {}", load_level_result);
                    }
                }

                if back_button.is_clicked() {
                    game_state.0.set(GameState::Menu)
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
                playing::physics::main_physics::physics_handle(
                    &mut player,
                    &velocity_y.0,
                    &mut on_ground,
                    &mut rotation,
                    &mut world_offset,
                    movement_speed.0.get(),
                    &current_mode,
                    &mut player_cam_y
                );

                playing::hitboxes::hitbox_collision(
                    &mut player,
                    centered_player,
                    small_player,
                    &mut rotation,
                    &obj_grid,
                    &mut world_offset,
                    player_cam_y,
                    &velocity_y.0,
                    &gravity.0,
                    default_gravity.0.get(),
                    &jump_force.0,
                    default_jump_force.0.get(),
                    &movement_speed.0,
                    default_movement_speed.clone().0.get(),
                    &mut kill_player,
                    &mut on_ground,
                    &mut touching_block_ceiling,
                    &mut on_orb,
                    &mut current_gamemode,
                    &mut cc_1001,
                    &mut cc_1002,
                    &mut cc_1003,
                    &game_state.0,
                    &mut on_pad,
                    &mut stars,
                    &mut main_levels,
                    level_mode,
                    current_level,
                    current_mode.clone(),
                    &mut online_levels_beaten,
                    online_level_diff,
                    online_level_rated,
                    if level_id_textbox.input == "" { 0 } else { level_id_textbox.input.parse().unwrap() },
                );

                match current_gamemode {
                    GameMode::Cube => {
                        playing::physics::cube::physics_handle(
                            &velocity_y.0,
                            gravity.0.get(),
                            &mut on_ground,
                            jump_force.0.get()
                        );
                    }

                    GameMode::Ship => {
                        playing::physics::ship::physics_handle(
                            touching_block_ceiling,
                            gravity.0.get(),
                            &velocity_y.0,
                            ship_power.0.get(),
                            ship_falling_speed.0.get()
                        );
                    }

                    GameMode::Ball => {
                        playing::physics::ball::physics_handle(
                            &mut on_ground,
                            &velocity_y.0,
                            &gravity.0,
                            &jump_force.0,
                            &mut player.y,
                        );
                    }

                    GameMode::Wave => {
                        playing::physics::wave::physics_handle(
                            &velocity_y.0,
                            &gravity.0,
                            &vertical_wave_speed.0,
                            movement_speed.0.get()
                        );
                    }
                }

                if kill_player {
                    player.y = screen_height() / 1.15;
                    world_offset = 0.0;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = GREEN;
                    velocity_y.0.set(0.0);
                    movement_speed.0.set(default_movement_speed.0.get());
                    gravity.0.set(default_gravity.0.get());
                    jump_force.0.set(default_jump_force.0.get());
                    player_cam_y = 0.0;
                    kill_player = false;
                    player_trail.clear();
                    restart_audio(&sink);
                }

                if is_mouse_button_released(MouseButton::Left)
                || is_key_released(KeyCode::Space)
                || is_key_released(KeyCode::Up) {
                    on_orb = true
                }

                if on_pad_timer.update() {
                    on_pad = false;
                }

                if level_mode == 2 || current_gamemode == GameMode::Wave {
                    player_trail.push(vec2(
                        player.x + world_offset,
                        player.y - player_cam_y
                    ));
                }

                if is_key_pressed(KeyCode::Backspace) {
                    player.y = screen_height() / 1.15;
                    world_offset = 0.0;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = GREEN;
                    velocity_y.0.set(0.0);
                    movement_speed.0.set(default_movement_speed.clone().0.get());
                    gravity.0.set(default_gravity.0.get());
                    jump_force.0.set(default_jump_force.0.get());
                    level_mode = 1;
                    player_cam_y = 0.0;

                    stop_audio(&sink);
                    play_audio_path("Resources/Music/menu-music.mp3", master_volume, true, &sink);
                    game_state.0.set(GameState::LevelSelect)
                }

                if movement_speed.0.get() > default_movement_speed.0.get() * 1.3 {
                    if current_mode == "2" {
                        if is_key_down(KeyCode::Left) {
                            bg_offset -= 0.4 * movement_speed.0.get()
                        } else if is_key_down(KeyCode::Right) {
                            bg_offset += 0.4 * movement_speed.0.get()
                        }
                    } else {
                        bg_offset += 0.4 * movement_speed.0.get()
                    }
                } else {
                    if current_mode == "2" {
                        if is_key_down(KeyCode::Left) {
                            bg_offset -= 0.2 * movement_speed.0.get()
                        } else if is_key_down(KeyCode::Right) {
                            bg_offset += 0.2 * movement_speed.0.get()
                        }
                    } else {
                        bg_offset += 0.2 * movement_speed.0.get()
                    }
                }
            }

            GameState::CreatorMenu => {
                back_button.update(delta_time);
                editor_keybinds_button.update(delta_time);

                featured_button.update(delta_time);
                create_button.update(delta_time);
                search_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state.0.set(GameState::Menu)
                }

                if editor_keybinds_button.is_clicked() {
                    game_state.0.set(GameState::EditorKeybinds);
                }

                if create_button.is_clicked() {
                    let level_data: String = std::fs::read_to_string("./save-data/level.txt").unwrap();

                    loading::load_level(
                        level_data,
                        &mut obj_grid,
                        &mut cc_1001,
                        &mut cc_1002,
                        &mut current_mode,
                        &mut current_song,
                        true,
                        main_levels.clone()
                    );

                    game_state.0.set(GameState::Editor)
                }

                if search_button.is_clicked() {
                    game_state.0.set(GameState::SearchPage);
                }
            }

            GameState::Editor => {
                editor_back_button.update(delta_time);
                editor_save_button.update(delta_time);
                editor_options_button.update(delta_time);
                editor_upload_button.update(delta_time);
                edit_obj_button.update(delta_time);
                build_tab_button.update(delta_time);
                edit_tab_button.update(delta_time);
                editor_playtest_button.update(delta_time);

                for object in &mut obj_types {
                    object.1.button.update(delta_time);
                }

                for object in &obj_types {
                    if object.1.button.is_clicked()
                    && current_tab == 1 {
                        current_obj = object.1.id
                    }
                }

                if editor_back_button.is_clicked() {
                    player_trail.clear();
                    game_state.0.set(GameState::CreatorMenu)
                }

                if editor_options_button.is_clicked() {
                    bg_red_textbox.input = (cc_1001.r * 255.0).floor().to_string();
                    bg_green_textbox.input = (cc_1001.g * 255.0).floor().to_string();
                    bg_blue_textbox.input = (cc_1001.b * 255.0).floor().to_string();

                    grnd_red_textbox.input = (cc_1002.r * 255.0).floor().to_string();
                    grnd_green_textbox.input = (cc_1002.g * 255.0).floor().to_string();
                    grnd_blue_textbox.input = (cc_1002.b * 255.0).floor().to_string();

                    level_options_type = 1;
                    game_state.0.set(GameState::LevelSettings)
                }

                if edit_obj_button.is_clicked() && selected_obj == 23 {
                    let mut color_trigger_color = Color::from_rgba(0, 0, 0, 255);
                    for object in &obj_grid {
                        if object.selected && object.id == 23 {
                            let red: u8 = object.properties.clone().unwrap()[0].parse().unwrap();
                            let green: u8 = object.properties.clone().unwrap()[1].parse().unwrap();
                            let blue: u8 = object.properties.clone().unwrap()[2].parse().unwrap();

                            color_trigger_color = Color::from_rgba(red, green, blue, 255)
                        }
                    }

                    bg_red_textbox.input = (color_trigger_color.r * 255.0).floor().to_string();
                    bg_green_textbox.input = (color_trigger_color.g * 255.0).floor().to_string();
                    bg_blue_textbox.input = (color_trigger_color.b * 255.0).floor().to_string();

                    level_options_type = 2;
                    game_state.0.set(GameState::LevelSettings)
                }

                if editor_upload_button.is_clicked() {
                    game_state.0.set(GameState::LevelUpload);
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
                    level_mode = 2;
                    player_trail.clear();
                    stop_audio(&sink);
                    play_audio_path(&current_song, master_volume, false, &sink);
                    game_state.0.set(GameState::Playing)
                }

                if editor_save_button.is_clicked() {
                    println!("Saving level...");

                    let level_string: String = saving::level_to_string(
                        &obj_grid,
                        level_version,
                        cc_1001,
                        cc_1002,
                        current_song.clone(),
                        current_mode.clone()
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
                && !editor_save_button.rect.contains(mouse_position().into())
                && !editor_options_button.rect.contains(mouse_position().into())
                && !editor_upload_button.rect.contains(mouse_position().into())
                && !edit_obj_button.rect.contains(mouse_position().into()) {
                    editor::object_ped(
                        &mut obj_grid,
                        snapped_x,
                        snapped_y,
                        cam_pos_x,
                        cam_pos_y,
                        grid_size,
                        current_tab,
                        current_obj,
                        &mut selected_obj
                    );
                }

                // All the keybinds for the editor are in this function
                editor::keybind_handler(
                    &mut cam_pos_y,
                    &mut cam_pos_x,
                    &mut obj_grid
                );

                if is_mouse_button_down(MouseButton::Middle) {
                    editor::panning(&mut cam_pos_x, &mut cam_pos_y);
                }
            }

            GameState::LevelComplete => {
                back_button.update(delta_time);

                if back_button.is_clicked() {
                    world_offset = 0.0;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = GREEN;
                    velocity_y.0.set(0.0);
                    movement_speed.0.set(default_movement_speed.clone().0.get());
                    gravity.0.set(default_gravity.0.get());
                    jump_force.0.set(default_jump_force.0.get());
                    level_mode = 1;
                    player_cam_y = 0.0;
                    player_trail.clear();

                    stop_audio(&sink);
                    play_audio_path("Resources/Music/menu-music.mp3", master_volume, true, &sink);
                    game_state.0.set(GameState::Menu)
                }
            }

            GameState::LevelSettings => {
                back_button.update(delta_time);
                plat_classic_button.update(delta_time);
                bg_color_button.update(delta_time);
                grnd_color_button.update(delta_time);

                if back_button.is_clicked() {
                    let mut bg_red_parse_success: bool = false;
                    match bg_red_textbox.input.parse::<u8>() {
                        Ok(value) => {
                            if level_options_type == 1 {
                                cc_1001.r = value as f32 / 255.0;
                            } else {
                                for object in &mut obj_grid {
                                    if object.selected && object.id == 23 {
                                        object.properties.as_mut().unwrap()[0] = value.to_string();
                                    }
                                }
                            }
                            bg_red_parse_success = true;
                        }

                        Err(error) => {
                            println!("Error parsing bg_red: {}", error);
                        }
                    }

                    let mut bg_green_parse_success: bool = false;
                    match bg_green_textbox.input.parse::<u8>() {
                        Ok(value) => {
                            if level_options_type == 1 {
                                cc_1001.g = value as f32 / 255.0;
                            } else {
                                for object in &mut obj_grid {
                                    if object.selected && object.id == 23 {
                                        object.properties.as_mut().unwrap()[1] = value.to_string();
                                    }
                                }
                            }
                            bg_green_parse_success = true;
                        }

                        Err(error) => {
                            println!("Error parsing bg_green: {}", error);
                        }
                    }

                    let mut bg_blue_parse_success: bool = false;
                    match bg_blue_textbox.input.parse::<u8>() {
                        Ok(value) => {
                            if level_options_type == 1 {
                                cc_1001.b = value as f32 / 255.0;
                            } else {
                                for object in &mut obj_grid {
                                    if object.selected && object.id == 23 {
                                        object.properties.as_mut().unwrap()[2] = value.to_string();
                                    }
                                }
                            }
                            bg_blue_parse_success = true;
                        }

                        Err(error) => {
                            println!("Error parsing bg_blue: {}", error);
                        }
                    }



                    let mut grnd_red_parse_success: bool = false;
                    if level_options_type == 1 {
                        match grnd_red_textbox.input.parse::<u8>() {
                            Ok(value) => {
                                cc_1002.r = value as f32 / 255.0;
                                grnd_red_parse_success = true;
                            }

                            Err(error) => {
                                println!("Error parsing grnd_red: {}", error);
                            }
                        }
                    } else {
                        grnd_red_parse_success = true
                    }

                    let mut grnd_green_parse_success: bool = false;
                    if level_options_type == 1 {
                        match grnd_green_textbox.input.parse::<u8>() {
                            Ok(value) => {
                                cc_1002.g = value as f32 / 255.0;
                                grnd_green_parse_success = true;
                            }

                            Err(error) => {
                                println!("Error parsing grnd_green: {}", error);
                            }
                        }
                    } else {
                        grnd_green_parse_success = true
                    }

                    let mut grnd_blue_parse_success: bool = false;
                    if level_options_type == 1 {
                        match grnd_blue_textbox.input.parse::<u8>() {
                            Ok(value) => {
                                cc_1002.b = value as f32 / 255.0;
                                grnd_blue_parse_success = true;
                            }

                            Err(error) => {
                                println!("Error parsing grnd_blue: {}", error);
                            }
                        }
                    } else {
                        grnd_blue_parse_success = true
                    }

                    if bg_red_parse_success
                    && bg_green_parse_success
                    && bg_blue_parse_success
                    && grnd_red_parse_success
                    && grnd_green_parse_success
                    && grnd_blue_parse_success {
                        game_state.0.set(GameState::Editor)
                    }
                }

                if is_key_pressed(KeyCode::Left) && current_song_index > 0 && level_options_type == 1 {
                    current_song_index -= 1;
                    current_song = main_levels[current_song_index as usize].song.clone();
                }

                if is_key_pressed(KeyCode::Right) && current_song_index < main_levels.len() as u8 - 1 && level_options_type == 1 {
                    current_song_index += 1;
                    current_song = main_levels[current_song_index as usize].song.clone();
                }

                if plat_classic_button.is_clicked() && level_options_type == 1 {
                    if current_mode == "1" {
                        plat_classic_button.text = "Plat".to_string();
                        current_mode = "2".to_string();
                    } else {
                        plat_classic_button.text = "Classic".to_string();
                        current_mode = "1".to_string();
                    }
                }

                if bg_color_button.is_clicked() && level_options_type == 2 {
                    for object in &mut obj_grid {
                        if object.selected && object.id == 23 {
                            object.properties.as_mut().unwrap()[3] = "1".to_string();
                        }
                    }
                }

                if grnd_color_button.is_clicked() && level_options_type == 2 {
                    for object in &mut obj_grid {
                        if object.selected && object.id == 23 {
                            object.properties.as_mut().unwrap()[3] = "2".to_string();
                        }
                    }
                }

                bg_red_textbox.input();
                bg_green_textbox.input();
                bg_blue_textbox.input();

                grnd_red_textbox.input();
                grnd_green_textbox.input();
                grnd_blue_textbox.input();
            }

            GameState::SearchPage => {
                back_button.update(delta_time);
                level_download_button.update(delta_time);

                if back_button.is_clicked() {
                    show_level_not_found = false;
                    game_state.0.set(GameState::CreatorMenu);
                }

                if level_download_button.is_clicked() {
                    level_download_response = ureq::get(download_url.clone())
                        .query("id", &level_id_textbox.input)
                        .call()
                        .unwrap()
                        .into_body()
                        .read_to_string()
                        .unwrap();

                    if level_download_response.clone().contains(";;;;;") {
                        parse_level_download_response(
                            level_download_response.clone(),
                            &mut online_level_name,
                            &mut online_level_desc,
                            &mut online_level_diff,
                            &mut online_level_rated,
                            &mut online_level_creator,
                            &mut online_level_data
                        );

                        show_level_not_found = false;
                        game_state.0.set(GameState::LevelPage);
                    } else {
                        show_level_not_found = true;
                    }
                }

                level_id_textbox.input();
            }

            GameState::LevelPage => {
                back_button.update(delta_time);
                level_play_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state.0.set(GameState::SearchPage);
                }

                if level_play_button.is_clicked() {
                    let load_level_result = load_level(
                        online_level_data.clone(),
                        &mut obj_grid,
                        &mut cc_1001,
                        &mut cc_1002,
                        &mut current_mode,
                        &mut current_song,
                        true,
                        main_levels.clone()
                    );

                    stop_audio(&sink);
                    play_audio_path(
                        &current_song,
                        master_volume,
                        false,
                        &sink
                    );

                    if load_level_result == "ok" {
                        level_mode = 3;
                        game_state.0.set(GameState::Playing);
                    } else {
                        println!("Problem loading level: {}", load_level_result);
                    }
                }
            }

            GameState::AccountPage => {
                back_button.update(delta_time);
                login_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state.0.set(GameState::Menu);
                }

                if login_button.is_clicked() {
                    login_response = ureq::post(&login_url)
                        .send_form([
                            ("user", username_textbox.input.clone().as_str()),
                            ("pass", password_textbox.input.clone().as_str()),
                        ])
                        .unwrap()
                        .into_body()
                        .read_to_string()
                        .unwrap();

                    if login_response == "Logged in!" {
                        username = username_textbox.input.clone();
                        password = password_textbox.input.clone();
                        logged_in = true;
                    }
                }

                username_textbox.input();
                password_textbox.input();
            }

            GameState::LevelUpload => {
                back_button.update(delta_time);
                upload_button.update(delta_time);

                if is_key_pressed(KeyCode::Left) && current_difficulty > 0 {
                    current_difficulty -= 1;
                } else if is_key_pressed(KeyCode::Right)
                && current_difficulty < 5 {
                    current_difficulty += 1;
                }

                if back_button.is_clicked() {
                    game_state.0.set(GameState::Editor);
                }

                if upload_button.is_clicked() && logged_in {
                    let level_data: String = saving::level_to_string(
                        &obj_grid,
                        level_version,
                        cc_1001,
                        cc_1002,
                        current_song.clone(),
                        current_mode.clone()
                    );

                    level_upload_response = ureq::post(&upload_url)
                        .send_form([
                            ("name", level_name_textbox.input.clone().as_str()),
                            ("desc", level_desc_textbox.input.clone().as_str()),
                            ("data", &level_data),
                            ("creator", &username),
                            ("pass", &password),
                            ("diff", &current_difficulty.to_string())
                        ])
                        .unwrap()
                        .into_body()
                        .read_to_string()
                        .unwrap();
                }

                level_name_textbox.input();
                level_desc_textbox.input();
            }

            GameState::EditorKeybinds => {
                back_button.update(delta_time);

                if back_button.is_clicked() {
                    game_state.0.set(GameState::CreatorMenu);
                }
            }
        }

        // Drawing
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

        match game_state.0.get() {
            GameState::Menu => {
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

                draw_text_pro(
                    &format!("Stars: {}", stars),
                    20.0,
                    120.0,
                    20,
                    RED,
                    &font
                );

                if cfg!(debug_assertions) {
                    draw_text_pro(
                        "Developer Build",
                        screen_width() - measure_text_ex("Developer Build", 30, &font) - 10.0,
                        screen_height() - 10.0,
                        30,
                        GREEN,
                        &font
                    );
                }

                play_button.draw(false, None, 1.0, false, &font);
                creator_button.draw(false, None, 1.0, false, &font);
                account_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelSelect => {
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
                        25,
                        &font
                    ) / 2.0,
                    100.0,
                    25,
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
                for i in -1..2 {
                    draw_texture_ex(
                        &default_bg,
                        (i * 1920) as f32 - (bg_offset % 1920.0),
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
                }

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
                        let rotation_f32: f32 = object.rotation as f32;
                        draw_texture_ex(
                            &obj_types[&(object.id)].texture,
                            object.x as f32 - if object.id == 8 || object.id == 9 || object.id == 24 || object.id == 25 { 40.0 } else { 0.0 } - world_offset as f32,
                            obj_y + 6.0 + player_cam_y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(
                                    obj_types[&(object.id)].texture.width() * if object.id == 17
                                    || object.id == 18
                                    || object.id == 19
                                    || object.id == 20 {
                                        0.1
                                    } else {
                                        0.05
                                    },
                                    obj_types[&(object.id)].texture.height() * if object.id == 17
                                    || object.id == 18
                                    || object.id == 19
                                    || object.id == 20 {
                                        0.1
                                    } else {
                                        0.05
                                    }
                                )),
                                source: None,
                                rotation: rotation_f32.to_radians(),
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                }

                // Draws the ground
                for i in -1..screen_width() as i32 / 160 + 2 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0 - (world_offset % 155.0),
                        screen_height() / 1.15 + player_cam_y,
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

                if level_mode == 2 || current_gamemode == GameMode::Wave {
                    for point in &player_trail {
                        if point.x - world_offset > -10.0 {
                            draw_circle(
                                point.x - world_offset,
                                point.y + player_cam_y,
                                5.0,
                                if level_mode != 2 {
                                    Color::from_rgba(0, 0, 255, 255)
                                } else {
                                    LIME
                                }
                            );
                        }
                    }
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
                back_button.draw(false, None, 1.0, false, &font);
                editor_keybinds_button.draw(false, None, 1.0, false, &font);
                featured_button.draw(false, None, 1.0, false, &font);
                create_button.draw(false, None, 1.0, false, &font);
                search_button.draw(false, None, 1.0, false, &font);
            }

            GameState::Editor => {
                // Draws the background
                for i in -1..2 {
                    draw_texture_ex(
                        &default_bg,
                        (i * 1920) as f32 - (cam_pos_x % 1920.0),
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
                }

                // Draws the ground
                for i in -1..screen_width() as i32 / 160 + 2 {
                    draw_texture_ex(
                        &grnd_texture,
                        i as f32 * 155.0 - (cam_pos_x*5.0 % 155.0),
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

                    let rotation_f32: f32 = object.rotation as f32;
                    draw_texture_ex(
                        &obj_types[&(object.id)].texture,
                        object.x as f32 - if object.id == 8 || object.id == 9 || object.id == 24 || object.id == 25 { 40.0 } else { 0.0 } - cam_pos_x * 5.0,
                        obj_y + cam_pos_y * 5.0,
                        if object.selected { GREEN } else { WHITE },
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                obj_types[&(object.id)].texture.width() * if object.id == 17
                                || object.id == 18
                                || object.id == 19
                                || object.id == 20 {
                                    0.1
                                } else {
                                    0.05
                                },
                                obj_types[&(object.id)].texture.height() * if object.id == 17
                                || object.id == 18
                                || object.id == 19
                                || object.id == 20 {
                                    0.1
                                } else {
                                    0.05
                                }
                            )),
                            source: None,
                            rotation: rotation_f32.to_radians(),
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

                for point in &player_trail {
                    if point.x - cam_pos_x * 5.0 > -10.0
                    && point.x - cam_pos_x * 5.0 < screen_width() + 10.0 {
                        draw_circle(point.x - cam_pos_x * 5.0, point.y + cam_pos_y * 5.0, 5.0, LIME);
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
                        object.1.button.draw(
                            true,
                            Some(&&object.1.texture),
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
                    &format!("Selected Object: {}", obj_types[&(current_obj)].name),
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

                    let correct_snapped_y = (screen_height() / 1.15 - 25.0) + (snapped_y as f32 - 500.0) as f32;
                    draw_rectangle_lines(
                        snapped_x as f32 - cam_pos_x * 5.0,
                        correct_snapped_y + cam_pos_y * 5.0,
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
                editor_upload_button.draw(false, None, 1.0, false, &font);
                edit_obj_button.draw(false, None, 1.0, false, &font);
                build_tab_button.draw(false, None, 1.0, false, &font);
                edit_tab_button.draw(false, None, 1.0, false, &font);
                editor_playtest_button.draw(false, None, 1.0, false, &font);
            }

            GameState::LevelComplete => {
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
                if level_options_type == 1 {
                    draw_text_pro(
                        &main_levels[current_song_index as usize].name,
                        screen_width() / 2.0 - measure_text_ex(&main_levels[current_song_index as usize].name, 30, &font) / 2.0,
                        screen_height() - 30.0,
                        30,
                        WHITE,
                        &font
                    );
                }

                editor::draw_color_preview_boxes(
                    &bg_red_textbox.input,
                    &bg_green_textbox.input,
                    &bg_blue_textbox.input,
                    &grnd_red_textbox.input,
                    &grnd_green_textbox.input,
                    &grnd_blue_textbox.input,
                    level_options_type
                );

                bg_red_textbox.draw(&font);
                bg_green_textbox.draw(&font);
                bg_blue_textbox.draw(&font);

                if level_options_type == 1 {
                    grnd_red_textbox.draw(&font);
                    grnd_green_textbox.draw(&font);
                    grnd_blue_textbox.draw(&font);
                }

                if level_options_type == 2 {
                    bg_color_button.draw(false, None, 1.0, false, &font);
                    grnd_color_button.draw(false, None, 1.0, false, &font);
                }

                back_button.draw(false, None, 1.0, false, &font);

                if level_options_type == 1 {
                    plat_classic_button.draw(false, None, 1.0, false, &font);
                }
            }

            GameState::SearchPage => {
                if show_level_not_found {
                    draw_text_pro(
                        &level_download_response,
                        screen_width() / 2.0 - measure_text_ex(&level_download_response, 30, &font) / 2.0,
                        screen_height() / 2.0,
                        30,
                        RED,
                        &font
                    );
                }

                back_button.draw(false, None, 1.0, false, &font);
                level_download_button.draw(false, None, 1.0, false, &font);
                level_id_textbox.draw(&font);
            }

            GameState::LevelPage => {
                draw_text_pro(
                    &online_level_name,
                    screen_width() / 2.0 - measure_text_ex(&online_level_name, 30, &font) / 2.0,
                    150.0,
                    30,
                    WHITE,
                    &font
                );

                draw_rectangle(
                    screen_width() / 2.0 - 394.0,
                    screen_height() / 2.0 + 100.0,
                    789.0,
                    50.0,
                    Color::from_rgba(
                        50,
                        50,
                        50,
                        100
                    )
                );

                draw_text_pro(
                    &online_level_desc,
                    screen_width() / 2.0 - measure_text_ex(&online_level_desc, 20, &font) / 2.0,
                    screen_height() / 2.0 + 100.0 + 35.0,
                    20,
                    WHITE,
                    &font
                );

                let weird_vec2_idk: Vec2 = if online_level_diff == 0 { Vec2::new(10.0, 60.0) } else { Vec2::new(-50.0, 0.0) };
                draw_texture_ex(
                    &difficulties[online_level_diff as usize],
                    weird_vec2_idk.x,
                    weird_vec2_idk.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            difficulties[online_level_diff as usize].width() * if online_level_diff == 0 { 0.3 } else { 0.2 },
                            difficulties[online_level_diff as usize].height() * if online_level_diff == 0 { 0.3 } else { 0.2 }
                        )),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None
                    }
                );

                if online_level_rated {
                    draw_text_pro(
                        &format!("{}", online_level_diff),
                        175.0,
                        300.0,
                        30,
                        WHITE,
                        &font
                    );

                    draw_texture_ex(
                        &star_texture,
                        30.0,
                        175.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(
                                star_texture.width() * 0.13,
                                star_texture.height() * 0.13
                            )),
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None
                        }
                    );
                }

                draw_text_pro(
                    &online_level_creator,
                    screen_width() / 2.0 - measure_text_ex(&online_level_creator, 50, &font) / 2.0,
                    80.0,
                    50,
                    WHITE,
                    &font
                );

                back_button.draw(false, None, 1.0, false, &font);
                level_play_button.draw(false, None, 1.0, false, &font);
            }

            GameState::AccountPage => {
                draw_text_pro(
                    &login_response,
                    screen_width() / 2.0 - measure_text_ex(&login_response, 20, &font) / 2.0,
                    200.0,
                    20,
                    RED,
                    &font
                );

                back_button.draw(false, None, 1.0, false, &font);
                login_button.draw(false, None, 1.0, false, &font);
                username_textbox.draw(&font);
                password_textbox.draw(&font);
            }

            GameState::LevelUpload => {
                draw_text_pro(
                    &level_upload_response,
                    screen_width() / 2.0 - measure_text_ex(&level_upload_response, 20, &font) / 2.0,
                    100.0,
                    20,
                    RED,
                    &font
                );

                draw_texture_ex(
                    &difficulties[current_difficulty as usize],
                    screen_width() / 2.0 - 200.0,
                    -30.0,
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
                upload_button.draw(false, None, 1.0, false, &font);
                level_name_textbox.draw(&font);
                level_desc_textbox.draw(&font);
            }

            GameState::EditorKeybinds => {
                back_button.draw(false, None, 1.0, false, &font);

                draw_text_pro(
                    "Use WASD to move objects one gridspace",
                    screen_width() / 2.0 - measure_text_ex("Use WASD to move objects one gridspace!", 20, &font) / 2.0,
                    200.0,
                    20,
                    RED,
                    &font
                );

                draw_text_pro(
                    "Use Shift + WASD to move objects off grid",
                    screen_width() / 2.0 - measure_text_ex("Use Shift + WASD to move objects off grid!", 20, &font) / 2.0,
                    250.0,
                    20,
                    RED,
                    &font
                );

                draw_text_pro(
                    "Use Q and E to rotate objects 90 degrees",
                    screen_width() / 2.0 - measure_text_ex("Use Q and E to rotate objects 90 degrees", 20, &font) / 2.0,
                    300.0,
                    20,
                    RED,
                    &font
                );

                draw_text_pro(
                    "Use Ctrl + D to duplicate objects",
                    screen_width() / 2.0 - measure_text_ex("Use Ctrl + D to duplicate objects", 20, &font) / 2.0,
                    350.0,
                    20,
                    RED,
                    &font
                );

                draw_text_pro(
                    "Use Alt + D to deselect all objects",
                    screen_width() / 2.0 - measure_text_ex("Use Alt + D to deselect all objects", 20, &font) / 2.0,
                    400.0,
                    20,
                    RED,
                    &font
                );
            }
        }

        // Runs the draw function of every mod
        for lua_mod in mods.clone() {
            let active: bool = lua_mod.get("enabled").unwrap();

            if active && load_mods {
                let draw_func: Function = lua_mod.get("draw").unwrap();

                draw_func.call::<()>(()).unwrap();
            }
        }

        next_frame().await;

        let frame_time = frame_start.elapsed();
        let frame_duration: std::time::Duration = std::time::Duration::from_micros(1_000_000 / game_tps.0.get() as u64);
        if frame_time < frame_duration {
            let sleep_time = frame_duration - frame_time;
            std::thread::sleep(sleep_time);
        }
    }

    let mut save_string = format!(
        "stars:{};user:{};pass:{};;;",

        stars,
        username,
        password
    );

    let mut saving_index: u8 = 0;
    for main_level in main_levels {
        if main_level.completed {
            save_string.push_str(&format!("{}:1;", saving_index));
        } else {
            save_string.push_str(&format!("{}:0;", saving_index));
        }

        saving_index += 1
    }

    save_string.push_str(";;0;");
    for id in online_levels_beaten.iter().skip(1) {
        save_string.push_str(&format!("{};", id));
    }

    save_string.pop();

    let _ = std::fs::write("./save-data/save.txt", save_string);
}