use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    touching_block_ceiling: bool,
    gravity: f32,
    velocity_y: &Cell<f32>,
    ship_power: f32,
    ship_falling_speed: f32,
) {
    if !touching_block_ceiling {
        if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up) {
            if gravity > 0.0 {
                if velocity_y.get() > -10.0 {
                    velocity_y.set(velocity_y.get() - ship_power)
                }
            } else {
                if velocity_y.get() < 10.0 {
                    velocity_y.set(velocity_y.get() + ship_power)
                }
            }
        } else {
            if gravity > 0.0 {
                if velocity_y.get() < 10.0 {
                    velocity_y.set(velocity_y.get() + ship_falling_speed);
                }
            } else {
                if velocity_y.get() > -10.0 {
                    velocity_y.set(velocity_y.get() - ship_falling_speed);
                }
            }
        }
    } else {
        velocity_y.set(0.0);
    }
}