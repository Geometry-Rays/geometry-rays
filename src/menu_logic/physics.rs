use macroquad::prelude::*;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &mut f32,
    gravity: f32,
    jump_force: f32,
    on_ground: &mut bool,
) {
    player.y += *velocity_y;
    *velocity_y += gravity;

    if is_mouse_button_down(MouseButton::Left) && *on_ground {
        *velocity_y -= jump_force;
        *on_ground = false;
    }

    if player.y > screen_height() / 1.15 {
        player.y = screen_height() / 1.15;
        *velocity_y = 0.0;
        *on_ground = true
    }
}