use std::{cell::Cell, rc::Rc};

use macroquad::prelude::{Rect, Color, Texture2D};

use crate::impl_lua_fields;

pub enum GameState {
    Menu,
    LevelSelect,
    Playing,
    CreatorMenu,
    Editor,
    LevelComplete,
    LevelSettings
}

pub struct Button {
    pub rect: Rect,
    pub rect_fn: Box<dyn Fn() -> Rect>,
    pub text: String,
    pub font_size: i32,
    pub base_color: Color,
    pub hover_scale: f32,
    pub press_offset: f32,
    pub is_pressed: bool,
    pub animation_timer: f32,
    pub is_disabled: bool,
}

pub struct TextBox {
    pub rect: Rect,
    pub text: String,
    pub text_size: u8,
    pub max_length: u8,
    pub spaces_allowed: bool,
    pub active: bool
}

#[allow(dead_code)]
pub struct ObjectStruct {
    pub x: i32,
    pub y: i32,
    pub rotation: i16,
    pub no_touch: u8,
    pub hide: u8,
    pub id: u16,
    pub selected: bool,
    pub properties: Option<Vec<String>>
}

pub struct ObjectType {
    pub id: u16,
    pub name: String,
    pub texture: Texture2D,
    pub button: Button
}

#[derive(Clone)]
pub struct MainLevel {
    pub name: String,
    pub difficulty: u8,
    pub song: String,
    pub artist: String,
    pub creator: String,
    pub completed: bool,
    pub data: String
}

pub enum GameMode {
    Cube,
    Ship
}

pub struct Timer {
    pub duration: f32,
    pub time: f32,
}

#[derive(Clone)]
pub struct Shared<T: Copy>(pub Rc<Cell<T>>);

impl_lua_fields!(f32);