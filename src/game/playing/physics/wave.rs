use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    velocity_y: &Cell<f32>,
    gravity: &Cell<f32>,
    vertical_wave_speed: &Cell<f32>,
    movement_speed: f32
) {
    let wave_go_up_or_down: f32 = if gravity.get() > 0.0 {
        -vertical_wave_speed.get()
    } else {
        vertical_wave_speed.get()
    };

    if is_mouse_button_down(MouseButton::Left)
    || is_key_down(KeyCode::Space)
    || is_key_down(KeyCode::Up) {
        velocity_y.set(wave_go_up_or_down * movement_speed);
    } else {
        velocity_y.set(-(wave_go_up_or_down * movement_speed));
    }
}