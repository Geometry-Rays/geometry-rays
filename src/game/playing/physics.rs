use macroquad::prelude::*;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &mut f32,
    on_ground: &mut bool,
    rotation: &mut f32,
    world_offset: &mut f32,
    movement_speed: f32
) {
    player.y += *velocity_y;
    // *velocity_y += gravity;

    // if is_mouse_button_down(MouseButton::Left) && *on_ground {
    //     *velocity_y -= jump_force;
    //     *on_ground = false;
    // }

    if player.y > screen_height() / 1.15 - 20.0 {
        player.y = screen_height() / 1.15 - 20.0;
        *velocity_y = 0.0;
        *on_ground = true;
        *rotation = 0.0
    } else if player.y < screen_height() / 1.15 - 21.0 {
        *rotation += 0.1
    }

    *world_offset += movement_speed
}

pub fn cube_physics(
    velocity_y: &mut f32,
    gravity: f32,
    on_ground: &mut bool,
    jump_force: f32
) {
    *velocity_y += gravity;

    if is_mouse_button_down(MouseButton::Left) && *on_ground {
        *velocity_y -= jump_force;
        *on_ground = false;
    }
}

pub fn ship_physics(
    touching_block_ceiling: bool,
    gravity: f32,
    velocity_y: &mut f32,
    ship_power: f32,
    ship_falling_speed: f32,
) {
    if !touching_block_ceiling {
        if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) {
            if gravity > 0.0 {
                if *velocity_y > -10.0 {
                    *velocity_y -= ship_power
                }
            } else {
                if *velocity_y < 10.0 {
                    *velocity_y += ship_power
                }
            }
        } else {
            if gravity > 0.0 {
                if *velocity_y < 10.0 {
                    *velocity_y += ship_falling_speed
                }
            } else {
                if *velocity_y > -10.0 {
                    *velocity_y -= ship_falling_speed
                }
            }
        }
    } else {
        *velocity_y = 0.0
    }
}