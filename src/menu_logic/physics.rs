use macroquad::prelude::*;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &mut i16,
    gravity: f32,
    jump_force: i16,
    on_ground: &mut bool,
) {
    player.y += *velocity_y as f32;
    *velocity_y += gravity as i16;

    if is_mouse_button_down(MouseButton::Left) && *on_ground {
        *velocity_y -= jump_force;
        *on_ground = false;
    }

    if player.y > screen_height() / 1.15 {
        player.y = screen_height() / 1.15;
        *velocity_y = 0;
        *on_ground = true
    }
}