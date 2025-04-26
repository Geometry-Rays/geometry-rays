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

pub fn hitbox_collision(
    player: &mut Rect,
    centered_player: Rect,
    small_player: Rect,
    rotation: &mut f32,
    obj_grid: &Vec<ObjectStruct>,
    world_offset: f32,
    player_cam_y: f32,
    velocity_y: &mut f32,
    gravity: &mut f32,
    default_gravity: f32,
    jump_force: &mut f32,
    default_jump_force: f32,
    kill_player: &mut bool,
    is_on_ground: &mut bool,
    touching_block_ceiling: &mut bool,
    on_orb: &mut bool
) {
    for object in obj_grid {
        let obj_y = ((screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0)) + 6.0;
        if object.id == 1 {
            *kill_player |= centered_player.overlaps(&Rect {
                x: object.x as f32 - world_offset + 15.0,
                y: obj_y as f32 + 5.0,
                w: 10.0,
                h: 20.0
            });
        }

        if object.id == 2 {
            *kill_player |= small_player.overlaps(&Rect {
                x: object.x as f32 - world_offset,
                y: object.y as f32 + 10.0 - player_cam_y as f32,
                w: 3.0,
                h: 20.0
            });

            if centered_player.overlaps(&Rect {
                x: object.x as f32 - world_offset + 3.0,
                y: obj_y as f32 + 1.0 - player_cam_y as f32,
                w: 37.0,
                h: 3.0
            }) {
                if *velocity_y >= 0.0 {
                    *is_on_ground = true;
                    *rotation = 0.0;
                    player.y = obj_y as f32 - 19.0 - player_cam_y as f32;
                    *velocity_y = 0.0;
                } else {
                    *touching_block_ceiling = true;
                    player.y = obj_y as f32 - 21.0 - player_cam_y as f32;
                }
            } else {
                *touching_block_ceiling = false;
            }

            if centered_player.overlaps(&Rect {
                x: object.x as f32 - world_offset + 3.0,
                y: obj_y as f32 + 38.0 - player_cam_y as f32,
                w: 37.0,
                h: 3.0
            }) {
                *rotation = 0.0;
                if *gravity < 0.0 {
                    *is_on_ground = true;
                    if !is_mouse_button_down(MouseButton::Left) {
                        player.y = obj_y as f32 + 61.0 - player_cam_y as f32;
                        *velocity_y = 0.0;
                    }
                } else {
                    *touching_block_ceiling = true;
                    *velocity_y = 0.0;
                    player.y = obj_y as f32 + 64.0 - player_cam_y as f32;
                }
            } else {
                *touching_block_ceiling = false;
            }

            if centered_player.overlaps(&Rect {
                x: object.x as f32 - world_offset + 80.0,
                y: obj_y as f32 - player_cam_y as f32 + 10.0,
                w: 3.0,
                h: 20.0,
            }) {
                *is_on_ground = false;
            }
        }

        if object.id == 3 {
            if centered_player.overlaps( &Rect {
                x: object.x as f32 - world_offset,
                y: obj_y as f32 - player_cam_y as f32 + 35.0,
                w: 40.0,
                h: 5.0
            }) {
                if *gravity > 0.0 {
                    *velocity_y = -18.0
                } else {
                    *velocity_y = 18.0
                }
            }
        }

        if object.id == 4 {
            if centered_player.overlaps(&Rect {
                x: object.x as f32 - 10.0 - world_offset,
                y: object.y as f32 - 10.0 - player_cam_y as f32,
                w: 60.0,
                h: 60.0
            }) {
                if *on_orb && (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space)) {
                    if object.id == 4 {
                        if *gravity > 0.0 {
                            *velocity_y = -13.0;
                        } else {
                            *velocity_y = 13.0
                        }
                    }
                    *on_orb = false
                }

                *is_on_ground = false
            }
        }

        if object.id == 5 || object.id == 6 {
            if centered_player.overlaps(&Rect {
                x: object.x as f32 - world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
                w: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                h: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
            }) {
                if object.id == 5 {
                    *jump_force = -default_jump_force;
                    *gravity = -default_gravity;
                } else {
                    *jump_force = default_jump_force;
                    *gravity = default_gravity;
                }

                *is_on_ground = false
            }
        }
    }
}

pub fn hitbox_draw(
    player: Rect,
    small_player: Rect,
    obj_grid: &Vec<ObjectStruct>,
    world_offset: f32,
    player_cam_y: f32,
) {
    for object in obj_grid {
        let obj_y = ((screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0)) + 6.0;
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

        if object.id == 2 {
            draw_rectangle_lines(
                object.x as f32 - world_offset,
                object.y as f32 + 10.0 - player_cam_y as f32,
                3.0,
                20.0,
                2.0,
                RED
            );

            draw_rectangle_lines(
                object.x as f32 - world_offset + 3.0,
                obj_y as f32 + 1.0 - player_cam_y as f32,
                37.0,
                3.0,
                2.0,
                BLUE
            );

            draw_rectangle_lines(
                object.x as f32 - world_offset + 3.0,
                obj_y as f32 + 38.0 - player_cam_y as f32,
                37.0,
                3.0,
                2.0,
                BLUE
            );

            draw_rectangle_lines(
                object.x as f32 - world_offset + 80.0,
                obj_y as f32 - player_cam_y as f32 + 10.0,
                3.0,
                20.0,
                2.0,
                GREEN
            )
        }

        if object.id == 3 {
            draw_rectangle_lines(
                object.x as f32 - world_offset,
                obj_y as f32 - player_cam_y as f32 + 35.0,
                40.0,
                5.0,
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            );
        }

        if object.id == 4 {
            draw_rectangle_lines(
                object.x as f32 - 10.0 - world_offset,
                object.y as f32 - 10.0 - player_cam_y as f32,
                60.0,
                60.0,
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            )
        }
    }

    draw_rectangle_lines(player.x, player.y, 40.0, 40.0, 2.0, WHITE);
    draw_rectangle_lines(small_player.x, small_player.y, small_player.w, small_player.h, 2.0, WHITE);
}