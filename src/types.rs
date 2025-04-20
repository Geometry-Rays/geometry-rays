use macroquad::prelude::{Rect, Color};

pub enum GameState {
    Menu,
    LevelSelect,
    Playing,
    CreatorMenu,
    Editor
}

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub font_size: i32,
    pub base_color: Color,
    pub hover_scale: f32,
    pub press_offset: f32,
    pub is_pressed: bool,
    pub animation_timer: f32,
    pub is_disabled: bool,
}