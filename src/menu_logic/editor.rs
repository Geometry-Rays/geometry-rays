use macroquad::prelude::*;

pub fn keybind_handler(
    cam_pos_y: &mut f32
) {
    if is_key_down(KeyCode::Up) {
        *cam_pos_y += 5.0;
    }

    if is_key_down(KeyCode::Down) {
        *cam_pos_y -= 5.0;
    }
}