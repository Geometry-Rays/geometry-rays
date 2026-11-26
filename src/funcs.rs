use macroquad::prelude::*;
use image::ImageReader;

use crate::types::*;

pub fn parse_level_packs_response(response: &str) -> Vec<LevelPack> {
    let response = response.trim();
    if response.is_empty() || response == "No packs found!" || response == "invalid_request" {
        return Vec::new();
    }

    response
        .split(';')
        .filter_map(|entry| {
            let mut fields = entry.splitn(3, ':');
            let id = fields.next()?.trim();
            let name = fields.next()?.trim();
            let difficulty = fields.next()?.trim().parse::<u8>().ok()?;

            if id.is_empty() || name.is_empty() || difficulty > 10 {
                return None;
            }

            Some(LevelPack { id: id.to_string(), name: name.to_string(), difficulty })
        })
        .collect()
}

// The game is always rendered inside a 16:9 canvas. If the window isn't 16:9
// the canvas is centered and the leftover space becomes black bars, so the
// layout never stretches and the aspect ratio never changes.
pub const GAME_ASPECT_RATIO: f32 = 16.0 / 9.0;

// Everything in the game (buttons, text, spacing, gameplay) is laid out at
// this design resolution and then scaled onto the canvas, so the UI grows and
// shrinks with the window instead of overlapping in a small window.
pub const GAME_DESIGN_WIDTH: f32 = 1280.0;
pub const GAME_DESIGN_HEIGHT: f32 = GAME_DESIGN_WIDTH / GAME_ASPECT_RATIO;

pub fn game_canvas() -> Rect {
    fit_canvas(screen_width(), screen_height())
}

fn fit_canvas(window_width: f32, window_height: f32) -> Rect {
    let mut canvas_width = window_width;
    let mut canvas_height = window_width / GAME_ASPECT_RATIO;

    if canvas_height > window_height {
        canvas_height = window_height;
        canvas_width = window_height * GAME_ASPECT_RATIO;
    }

    let canvas_width = canvas_width.floor().max(1.0);
    let canvas_height = canvas_height.floor().max(1.0);

    Rect::new(
        ((window_width - canvas_width) / 2.0).floor(),
        ((window_height - canvas_height) / 2.0).floor(),
        canvas_width,
        canvas_height
    )
}

// Width of the game in design units. Use this instead of screen_width().
pub fn game_width() -> f32 {
    GAME_DESIGN_WIDTH
}

// Height of the game in design units. Use this instead of screen_height().
pub fn game_height() -> f32 {
    GAME_DESIGN_HEIGHT
}

pub fn game_mouse_position() -> (f32, f32) {
    mouse_to_game(mouse_position(), game_canvas())
}

fn mouse_to_game(mouse: (f32, f32), canvas: Rect) -> (f32, f32) {
    let scale = canvas.w / GAME_DESIGN_WIDTH;

    ((mouse.0 - canvas.x) / scale, (mouse.1 - canvas.y) / scale)
}

// Camera that draws in canvas space and clips the game to the canvas, which is
// what makes the black bars around the game.
pub fn game_camera() -> Camera2D {
    canvas_camera(game_canvas(), screen_dpi_scale())
}

pub const GAMEPLAY_ZOOM: f32 = 1.5;

const GAMEPLAY_ANCHOR_X: f32 = 200.0;

pub fn gameplay_camera() -> Camera2D {
    canvas_camera_zoomed(
        game_canvas(),
        screen_dpi_scale(),
        GAMEPLAY_ZOOM,
        vec2(GAMEPLAY_ANCHOR_X, GAME_DESIGN_HEIGHT / 1.15)
    )
}

fn canvas_camera(canvas: Rect, dpi_scale: f32) -> Camera2D {
    canvas_camera_zoomed(
        canvas,
        dpi_scale,
        1.0,
        vec2(GAME_DESIGN_WIDTH / 2.0, GAME_DESIGN_HEIGHT / 2.0)
    )
}

fn canvas_camera_zoomed(
    canvas: Rect,
    dpi_scale: f32,
    zoom: f32,
    anchor: Vec2
) -> Camera2D {
    let center = vec2(GAME_DESIGN_WIDTH / 2.0, GAME_DESIGN_HEIGHT / 2.0);

    let target = anchor - (anchor - center) / zoom;

    Camera2D {
        target,
        // macroquad flips the Y axis of on screen 2D cameras, so a positive
        // zoom here keeps Y going down and puts the origin in the top left
        // corner of the canvas, just like the default camera
        zoom: vec2(
            2.0 / GAME_DESIGN_WIDTH * zoom,
            2.0 / GAME_DESIGN_HEIGHT * zoom
        ),
        offset: Vec2::ZERO,
        rotation: 0.0,
        render_target: None,
        viewport: Some((
            (canvas.x * dpi_scale).round() as i32,
            (canvas.y * dpi_scale).round() as i32,
            (canvas.w * dpi_scale).round() as i32,
            (canvas.h * dpi_scale).round() as i32
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW_SIZES: [(f32, f32); 6] = [
        (1280.0, 720.0),
        (1920.0, 1080.0),
        (2560.0, 1080.0),
        (1920.0, 1200.0),
        (800.0, 600.0),
        (1000.0, 1000.0)
    ];

    #[test]
    fn canvas_is_always_sixteen_by_nine_and_fits_inside_the_window() {
        for (window_width, window_height) in WINDOW_SIZES {
            let canvas = fit_canvas(window_width, window_height);

            assert!(
                (canvas.w / canvas.h - GAME_ASPECT_RATIO).abs() < 0.01,
                "{}x{} gave a {}x{} canvas",
                window_width, window_height, canvas.w, canvas.h
            );

            assert!(canvas.w <= window_width);
            assert!(canvas.h <= window_height);
        }
    }

    #[test]
    fn canvas_is_centered_so_the_bars_are_even() {
        for (window_width, window_height) in WINDOW_SIZES {
            let canvas = fit_canvas(window_width, window_height);

            assert!((canvas.x * 2.0 + canvas.w - window_width).abs() <= 1.0);
            assert!((canvas.y * 2.0 + canvas.h - window_height).abs() <= 1.0);
        }
    }

    #[test]
    fn camera_stretches_the_design_space_over_the_canvas() {
        let camera = canvas_camera(Rect::new(160.0, 90.0, 1280.0, 720.0), 1.0);
        let matrix = camera.matrix();

        let top_left = matrix.transform_point3(vec3(0.0, 0.0, 0.0));
        let bottom_right = matrix.transform_point3(vec3(
            GAME_DESIGN_WIDTH,
            GAME_DESIGN_HEIGHT,
            0.0
        ));

        // The design space has to fill the whole canvas exactly
        assert!((top_left.x + 1.0).abs() < 1e-4);
        assert!((bottom_right.x - 1.0).abs() < 1e-4);

        // Y has to go down, so (0, 0) must be the top left corner
        assert!((top_left.y - 1.0).abs() < 1e-4);
        assert!((bottom_right.y + 1.0).abs() < 1e-4);
    }

    #[test]
    fn gameplay_zoom_keeps_the_player_and_ground_in_place() {
        let canvas = Rect::new(0.0, 0.0, GAME_DESIGN_WIDTH, GAME_DESIGN_HEIGHT);
        let anchor = vec2(GAMEPLAY_ANCHOR_X, GAME_DESIGN_HEIGHT / 1.15);

        let ui_matrix = canvas_camera(canvas, 1.0).matrix();
        let play_matrix = canvas_camera_zoomed(canvas, 1.0, GAMEPLAY_ZOOM, anchor).matrix();

        let ui_anchor = ui_matrix.transform_point3(vec3(anchor.x, anchor.y, 0.0));
        let play_anchor = play_matrix.transform_point3(vec3(anchor.x, anchor.y, 0.0));

        // The player/ground line must not move when the zoom kicks in
        assert!((ui_anchor.x - play_anchor.x).abs() < 1e-4);
        assert!((ui_anchor.y - play_anchor.y).abs() < 1e-4);

        // And things next to the anchor have to appear GAMEPLAY_ZOOM times bigger
        let ui_point = ui_matrix.transform_point3(vec3(anchor.x + 64.0, anchor.y, 0.0));
        let play_point = play_matrix.transform_point3(vec3(anchor.x + 64.0, anchor.y, 0.0));

        assert!(
            (play_point.x - play_anchor.x - (ui_point.x - ui_anchor.x) * GAMEPLAY_ZOOM).abs() < 1e-4
        );
    }

    #[test]
    fn mouse_position_maps_back_into_design_space() {
        let canvas = fit_canvas(1920.0, 1200.0);
        let scale = canvas.w / GAME_DESIGN_WIDTH;

        let game_position = mouse_to_game(
            (canvas.x + 10.0 * scale, canvas.y + 20.0 * scale),
            canvas
        );

        assert!((game_position.0 - 10.0).abs() < 1e-3);
        assert!((game_position.1 - 20.0).abs() < 1e-3);
    }

    #[test]
    fn sixteen_by_nine_windows_have_no_bars() {
        let canvas = fit_canvas(1920.0, 1080.0);

        assert_eq!(canvas.x, 0.0);
        assert_eq!(canvas.y, 0.0);
        assert_eq!(canvas.w, 1920.0);
        assert_eq!(canvas.h, 1080.0);
    }
}

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

impl Button {
    pub fn new<Fx, Fy, Fw, Fh>(
        x: Fx,
        y: Fy,
        width: Fw,
        height: Fh,
        text: &str,
        font_size: i32,
        is_disabled: bool
    ) -> Self
    where
        Fx: 'static + Fn() -> f32 + Clone,
        Fy: 'static + Fn() -> f32 + Clone,
        Fw: 'static + Fn() -> f32 + Clone,
        Fh: 'static + Fn() -> f32 + Clone,
    {
        let x_clone = x.clone();
        let y_clone = y.clone();
        let w_clone = width.clone();
        let h_clone = height.clone();
        let rect_fn = Box::new(move || {
            Rect::new(x_clone(), y_clone(), w_clone(), h_clone())
        });

        Button {
            rect: Rect::new(x(), y(), width(), height()),
            rect_fn,
            text: text.to_string(),
            font_size,
            base_color: WHITE,
            hover_scale: 1.0,
            press_offset: 0.0,
            is_pressed: false,
            animation_timer: 0.0,
            is_disabled: is_disabled,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rect = (self.rect_fn)();

        let is_hovered = self.is_hovered();
        let is_pressed = is_hovered && is_mouse_button_down(MouseButton::Left);

        // Update hover animation
        let target_scale = if is_hovered { 1.1 } else { 1.0 };
        self.hover_scale += (target_scale - self.hover_scale) * (delta_time * 12.0);

        // Update press animation
        let target_offset = if is_pressed { 4.0 } else { 0.0 };
        self.press_offset += (target_offset - self.press_offset) * (delta_time * 15.0);

        // Update color animation
        if is_hovered {
            self.animation_timer = (self.animation_timer + delta_time * 8.0).min(1.0);
        } else {
            self.animation_timer = (self.animation_timer - delta_time * 8.0).max(0.0);
        }

        self.is_pressed = is_pressed;
    }

    pub fn draw(&self, use_image: bool, image: Option<&&Texture2D>, image_scale: f32, gray: bool, font: &Font) {
        let scale_offset_x = self.rect.w * (self.hover_scale - 1.0) * 0.5;
        let scale_offset_y = self.rect.h * (self.hover_scale - 1.0) * 0.5;

        let scaled_rect = Rect::new(
            self.rect.x - scale_offset_x,
            self.rect.y - scale_offset_y + self.press_offset,
            self.rect.w * self.hover_scale,
            self.rect.h * self.hover_scale,
        );

        // Draw main button body
        draw_rectangle(
            scaled_rect.x,
            scaled_rect.y,
            scaled_rect.w,
            scaled_rect.h,
            if self.is_disabled { BLACK } else if gray { GRAY } else { self.base_color },
        );

        // Old way of drawing button borders
        // d.draw_rectangle_lines(
        //     scaled_rect.x as i32,
        //     scaled_rect.y as i32,
        //     scaled_rect.width as i32,
        //     scaled_rect.height as i32,
        //     Color::new(0, 0, 0, 255),
        // );

        // Draw button borders
        draw_rectangle_lines(
            scaled_rect.x,
            scaled_rect.y,
            scaled_rect.w,
            scaled_rect.h,
            10.0,
            if self.is_disabled { WHITE } else { BLACK }
        );

        // d.draw_rectangle_lines_ex(scaled_rect, 5.0, if self.is_disabled { Color::WHITE } else { Color::BLACK });

        // Draw text with perfect centering
        let text_width = measure_text_ex(&self.text, self.font_size as u8, &font);
        let text_x = scaled_rect.x + ((scaled_rect.w - text_width) / 2.0);
        let text_y = scaled_rect.y as i32 + ((scaled_rect.h as i32 - self.font_size) / 2);

        if !use_image {
            draw_text_pro(
                &self.text,
                text_x,
                text_y as f32 + self.font_size as f32,
                self.font_size as u8,
                if self.is_disabled { WHITE } else { BLACK },
                &font
            );
        } else {
            draw_texture_ex(
                image.unwrap(),
                self.rect.x + self.rect.w / 2.0 - image.unwrap().width() as f32 * image_scale / 2.0,
                self.rect.y + self.rect.w / 2.0 - image.unwrap().height() as f32 * image_scale / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        image.unwrap().width() * image_scale,
                        image.unwrap().height() * image_scale
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

    pub fn is_hovered(&self) -> bool {
        self.rect.contains(game_mouse_position().into())
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_released(MouseButton::Left)
    }
}

impl ObjectType {
    pub async fn new(
        id: u16,
        name: &str,
        texture: &str,
        obj_btn_offset: f32,
        object_types_amount: u16
    ) -> ObjectType {
        let mut x_sort = 1;
        let mut y_sort = 0;

        for _ in 0..object_types_amount {
            x_sort += 1;

            if x_sort > 9 {
                x_sort = 1;
                y_sort += 1;
            }
        }

        ObjectType {
            id,
            name: name.to_string(),
            texture: load_texture(texture)
                .await.expect("Failed to load name texture"),
            button: Button::new(
                move || 140.0 + (x_sort as f32 * obj_btn_offset),
                move || game_height() - (190.0 - (y_sort as f32 * obj_btn_offset)),
                || 55.0,
                || 55.0,
                name,
                10,
                false
            )
        }
    }
}

impl TextBox {
    pub fn new<Fx, Fy, Fw, Fh>(
        x: Fx,
        y: Fy,
        width: Fw,
        height: Fh,
        text: &str,
        text_size: u8,
        max_length: u8,
        spaces_allowed: bool
    ) -> Self
    where
        Fx: 'static + Fn() -> f32 + Clone,
        Fy: 'static + Fn() -> f32 + Clone,
        Fw: 'static + Fn() -> f32 + Clone,
        Fh: 'static + Fn() -> f32 + Clone,
    {
        let x_clone = x.clone();
        let y_clone = y.clone();
        let w_clone = width.clone();
        let h_clone = height.clone();

        let rect_fn = Box::new(move || {
            Rect::new(x_clone(), y_clone(), w_clone(), h_clone())
        });

        TextBox {
            rect: Rect::new(x(), y(), width(), height()),
            rect_fn,
            text: text.to_string(),
            input: "".to_string(),
            text_size,
            max_length,
            spaces_allowed,
            active: false
        }
    }

    pub fn is_clicked(&self) -> bool {
        let mouse_pos = game_mouse_position();
        self.rect.contains(mouse_pos.into()) && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn is_not_clicked(&self) -> bool {
        let mouse_pos = game_mouse_position();
        !self.rect.contains(mouse_pos.into()) && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn draw(&self, font: &Font) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            Color::from_rgba(
                50,
                50,
                50,
                if self.active { 100 } else { 200 }
            )
        );

        draw_text_pro(
            if !self.input.is_empty() { self.input.as_str() } else { self.text.as_str() },
            self.rect.x + 10.0,
            self.rect.y + self.rect.h / 2.0 + self.text_size as f32 / 2.0,
            self.text_size,
            if !self.input.is_empty() { WHITE } else { GRAY },
            &font
        );
    }

    pub fn input(&mut self) {
        self.rect = (self.rect_fn)();

        if self.is_clicked() {
            self.active = true
        }

        if self.is_not_clicked() {
            self.active = false
        }

        if is_key_pressed(KeyCode::Backspace) && self.input.len() > 0 && self.active {
            self.input.pop();
        }

        if self.active && self.input.len() < self.max_length as usize {
            if is_key_pressed(KeyCode::A) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'A' } else { 'a' });
            }

            else if is_key_pressed(KeyCode::B) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'B' } else { 'b' });
            }

            else if is_key_pressed(KeyCode::C) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'C' } else { 'c' });
            }

            else if is_key_pressed(KeyCode::D) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'D' } else { 'd' });
            }

            else if is_key_pressed(KeyCode::E) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'E' } else { 'e' });
            }

            else if is_key_pressed(KeyCode::F) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'F' } else { 'f' });
            }

            else if is_key_pressed(KeyCode::G) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'G' } else { 'g' });
            }

            else if is_key_pressed(KeyCode::H) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'H' } else { 'h' });
            }

            else if is_key_pressed(KeyCode::I) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'I' } else { 'i' });
            }

            else if is_key_pressed(KeyCode::J) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'J' } else { 'j' });
            }

            else if is_key_pressed(KeyCode::K) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'K' } else { 'k' });
            }

            else if is_key_pressed(KeyCode::L) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'L' } else { 'l' });
            }

            else if is_key_pressed(KeyCode::M) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'M' } else { 'm' });
            }

            else if is_key_pressed(KeyCode::N) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'N' } else { 'n' });
            }

            else if is_key_pressed(KeyCode::O) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'O' } else { 'o' });
            }

            else if is_key_pressed(KeyCode::P) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'P' } else { 'p' });
            }

            else if is_key_pressed(KeyCode::Q) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'Q' } else { 'q' });
            }

            else if is_key_pressed(KeyCode::R) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'R' } else { 'r' });
            }

            else if is_key_pressed(KeyCode::S) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'S' } else { 's' });
            }

            else if is_key_pressed(KeyCode::T) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'T' } else { 't' });
            }

            else if is_key_pressed(KeyCode::U) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'U' } else { 'u' });
            }

            else if is_key_pressed(KeyCode::V) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'V' } else { 'v' });
            }

            else if is_key_pressed(KeyCode::W) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'W' } else { 'w' });
            }

            else if is_key_pressed(KeyCode::X) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'X' } else { 'x' });
            }

            else if is_key_pressed(KeyCode::Y) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'Y' } else { 'y' });
            }

            else if is_key_pressed(KeyCode::Z) {
                self.input.push(if is_key_down(KeyCode::LeftShift) { 'Z' } else { 'z' });
            }

            else if is_key_pressed(KeyCode::Space) && self.spaces_allowed {
                self.input.push(' ');
            }

            else if is_key_pressed(KeyCode::Key1) {
                self.input.push('1');
            }

            else if is_key_pressed(KeyCode::Key2) {
                self.input.push('2');
            }

            else if is_key_pressed(KeyCode::Key3) {
                self.input.push('3');
            }

            else if is_key_pressed(KeyCode::Key4) {
                self.input.push('4');
            }

            else if is_key_pressed(KeyCode::Key5) {
                self.input.push('5');
            }

            else if is_key_pressed(KeyCode::Key6) {
                self.input.push('6');
            }

            else if is_key_pressed(KeyCode::Key7) {
                self.input.push('7');
            }

            else if is_key_pressed(KeyCode::Key8) {
                self.input.push('8');
            }

            else if is_key_pressed(KeyCode::Key9) {
                self.input.push('9');
            }

            else if is_key_pressed(KeyCode::Key0) {
                self.input.push('0');
            }
        }
    }
}

impl MainLevel {
    pub fn new(
        name: &str,
        difficulty: u8,
        song: &str,
        artist: &str,
        creator: &str,
        data: String
    ) -> Self {
        return MainLevel {
            name: name.to_string(),
            difficulty,
            song: song.to_string(),
            artist: artist.to_string(),
            creator: creator.to_string(),
            completed: false,
            data: data.to_string()
        }
    }
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Self { duration, time: 0.0 }
    }

    pub fn update(&mut self) -> bool {
        self.time += get_frame_time();
        if self.time >= self.duration {
            self.time = 0.0;
            return true;
        }
        false
    }
}

impl GameState {
    pub fn to_string(&self) -> String {
        match &self {
            GameState::Menu => return "Menu".to_string(),
            GameState::LevelSelect => return "LevelSelect".to_string(),
            GameState::Playing => return "Playing".to_string(),
            GameState::CreatorMenu => return "CreatorMenu".to_string(),
            GameState::Editor => return "Editor".to_string(),
            GameState::LevelComplete => return "LevelComplete".to_string(),
            GameState::LevelSettings => return "LevelSettings".to_string(),
            GameState::SearchPage => return "SearchPage".to_string(),
            GameState::LevelPage => return "LevelPage".to_string(),
            GameState::AccountPage => return "AccountPage".to_string(),
            GameState::LevelUpload => return "LevelUpload".to_string(),
            GameState::EditorKeybinds => return "EditorKeybinds".to_string(),
            GameState::ChatMenu => return "ChatMenu".to_string(),
            GameState::LevelPacksMenu => return "LevelPacksMenu".to_string()
        }
    }
}

#[macro_export]
macro_rules! impl_lua_fields {
    ($type:ty) => {
        impl mlua::UserData for Shared<$type> {
            fn add_methods<'lua, M: mlua::UserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("get", |_, this, ()| Ok(this.0.get()));
                methods.add_method("set", |_, this, val: f32| {
                    this.0.set(val);
                    Ok(())
                });
            }
        }
    };
}

pub fn get_resized_rgba_bytes(path: &str, size: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?.to_rgba8();
    let resized = image::imageops::resize(&img, size, size, image::imageops::Lanczos3);
    Ok(resized.into_raw())
}
