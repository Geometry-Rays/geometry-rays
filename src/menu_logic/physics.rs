use macroquad::prelude::*;

use crate::types::ObjectStruct;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &mut f32,
    gravity: f32,
    jump_force: f32,
    on_ground: &mut bool,
    rotation: &mut f32,
    world_offset: &mut f32,
    movement_speed: f32
) {
    player.y += *velocity_y;
    *velocity_y += gravity;

    if is_mouse_button_down(MouseButton::Left) && *on_ground {
        *velocity_y -= jump_force;
        *on_ground = false;
    }

    if player.y > screen_height() / 1.15 - 25.0 {
        player.y = screen_height() / 1.15 - 25.0;
        *velocity_y = 0.0;
        *on_ground = true;
        *rotation = 0.0
    } else if player.y < screen_height() / 1.15 - 26.0 {
        *rotation += 0.1
    }

    *world_offset += movement_speed
}

pub fn hitbox_collision(
    player: Rect,
    obj_grid: &Vec<ObjectStruct>,
    world_offset: f32,
    kill_player: &mut bool
) {
    for object in obj_grid {
        let obj_y = (screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0);
        if object.id == 1 {
            *kill_player = player.overlaps(&Rect {
                x: object.x as f32 - world_offset + 15.0,
                y: obj_y as f32 + 5.0,
                w: 10.0,
                h: 20.0
            });
        }
    }
}

pub fn hitbox_draw(
    player: Rect,
    obj_grid: &Vec<ObjectStruct>,
    world_offset: f32
) {
    for object in obj_grid {
        let obj_y = (screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0);
        if object.id == 1 {
            draw_rectangle_lines(
                object.x as f32 - world_offset + 15.0,
                obj_y as f32 + 5.0,
                10.0,
                20.0,
                2.0,
                RED
            );
        }
    }

    draw_rectangle_lines(player.x, player.y, 50.0, 50.0, 2.0, WHITE);
}