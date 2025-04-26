use macroquad::prelude::*;
use crate::types::ObjectStruct;

pub fn keybind_handler(
    cam_pos_y: &mut f32,
    cam_pos_x: &mut f32
) {
    if is_key_down(KeyCode::Up) {
        *cam_pos_y += 1.0;
    }

    if is_key_down(KeyCode::Down) {
        *cam_pos_y -= 1.0;
    }

    if is_key_down(KeyCode::Left) {
        *cam_pos_x -= 1.0;
    }

    if is_key_down(KeyCode::Right) {
        *cam_pos_x += 1.0;
    }
}

pub fn object_ped(
    object_grid: &mut Vec<ObjectStruct>,

    snapped_x: i32,
    snapped_y: i32,

    current_tab: u8,
    current_obj: u16
) {
    if current_tab == 1 {
        object_grid.push(ObjectStruct {
            x: snapped_x,
            y: snapped_y,
            rotation: 0,
            id: current_obj,
            selected: false
        });
    }
}