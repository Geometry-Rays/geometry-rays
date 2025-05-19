use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    on_ground: &mut bool,
    velocity_y: &Cell<f32>,
    gravity: &Cell<f32>,
    player_y: &mut f32
) {
    velocity_y.set(velocity_y.get() + gravity.get());

    if *on_ground && (is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left)) {
        gravity.set(-gravity.get());
        *player_y = if gravity.get() > 0.0 { *player_y + 1.0 } else { *player_y - 1.0 };
        *on_ground = false;
    }
}