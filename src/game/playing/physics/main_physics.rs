use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &Cell<f32>,
    on_ground: &mut bool,
    rotation: &mut f32,
    world_offset: &mut f32,
    movement_speed: f32,
    current_mode: &String,
    player_cam_y: &mut f32
) {
    player.y += velocity_y.get();
    // *velocity_y += gravity;

    // if is_mouse_button_down(MouseButton::Left) && *on_ground {
    //     *velocity_y -= jump_force;
    //     *on_ground = false;
    // }

    if player.y > screen_height() / 1.15 - 20.0 + *player_cam_y {
        player.y = screen_height() / 1.15 - 20.0;
        velocity_y.set(0.0);
        *on_ground = true;
        *rotation = 0.0
    } else if player.y < screen_height() / 1.15 - 21.0 + *player_cam_y {
        if is_key_down(KeyCode::Right) || current_mode == "1" {
            *rotation += 0.1
        } else if is_key_down(KeyCode::Left) {
            *rotation -= 0.1
        } else {
            *rotation = 0.0
        }
    }

    if current_mode == "2" {
        if is_key_down(KeyCode::Left) {
            *world_offset -= movement_speed
        } else if is_key_down(KeyCode::Right) {
            *world_offset += movement_speed
        }
    } else {
        *world_offset += movement_speed
    }

    if player.y < 20.0 {
        *player_cam_y -= velocity_y.get();
        player.y = 20.0
    } else if player.y > screen_height() / 1.15 - 20.0 {
        *player_cam_y -= velocity_y.get();
        player.y = screen_height() / 1.15 - 20.0
    }
}