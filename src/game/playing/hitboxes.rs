use std::cell::Cell;

use macroquad::prelude::*;

use crate::types::{GameMode, GameState, ObjectStruct};

pub fn hitbox_collision(
    player: &mut Rect,
    centered_player: Rect,
    small_player: Rect,
    rotation: &mut f32,
    obj_grid: &Vec<ObjectStruct>,
    world_offset: f32,
    player_cam_y: f32,
    velocity_y: &Cell<f32>,
    gravity: &Cell<f32>,
    default_gravity: f32,
    jump_force: &Cell<f32>,
    default_jump_force: f32,
    movement_speed: &Cell<f32>,
    default_movement_speed: f32,
    kill_player: &mut bool,
    is_on_ground: &mut bool,
    touching_block_ceiling: &mut bool,
    on_orb: &mut bool,
    current_gamemode: &mut GameMode,
    cc_1001: &mut Color,
    cc_1002: &mut Color,
    cc_1003: &mut Color,
    game_state: &Cell<GameState>,
    on_pad: &mut bool
) {
    for object in obj_grid {
        let obj_y = ((screen_height() / 1.15 - 25.0) + (object.y as f32 - 500.0)) + 6.0;

        match object.id {
            1 => {
                *kill_player |= centered_player.overlaps(&Rect {
                    x: object.x as f32 - world_offset + 15.0,
                    y: obj_y as f32 + 5.0,
                    w: 10.0,
                    h: 20.0
                });
            }

            2 | 10 | 11 | 12 | 13 | 14 => {
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
                    if velocity_y.get() >= 0.0 {
                        *is_on_ground = true;
                        *rotation = 0.0;
                        player.y = obj_y as f32 - 19.0 - player_cam_y as f32;
                        velocity_y.set(0.0);
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
                    if velocity_y.get() <= 0.0 {
                        *is_on_ground = true;
                        *rotation = 0.0;
                        player.y = obj_y as f32 + 61.0 - player_cam_y as f32;
                        velocity_y.set(0.0);
                    } else {
                        *touching_block_ceiling = true;
                        player.y = obj_y as f32 + 65.0 - player_cam_y as f32;
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

            3 | 21 => {
                if centered_player.overlaps( &Rect {
                    x: object.x as f32 - world_offset,
                    y: obj_y as f32 - player_cam_y as f32 + 35.0,
                    w: 40.0,
                    h: 5.0
                }) {
                    if !*on_pad {
                        if object.id == 3 {
                            if gravity.get() > 0.0 {
                                velocity_y.set(-18.0)
                            } else {
                                velocity_y.set(18.0)
                            }
                        } else if object.id == 21 {
                            // *gravity = -*gravity;

                            if gravity.get() > 0.0 {
                                velocity_y.set(-7.0);
                                gravity.set(-default_gravity);
                                jump_force.set(-default_jump_force);
                            } else {
                                velocity_y.set(7.0);
                                gravity.set(default_gravity);
                                jump_force.set(default_jump_force)
                            }
                        }

                        *on_pad = true;
                        *is_on_ground = false
                    }
                }
            }

            4 | 22 => {
                if centered_player.overlaps(&Rect {
                    x: object.x as f32 - 10.0 - world_offset,
                    y: object.y as f32 - 10.0 - player_cam_y as f32,
                    w: 60.0,
                    h: 60.0
                }) {
                    if *on_orb && (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) {
                        if object.id == 4 {
                            if gravity.get() > 0.0 {
                                velocity_y.set(-16.0);
                            } else {
                                velocity_y.set(16.0);
                            }
                        } else if object.id == 22 {
                            if gravity.get() > 0.0 {
                                velocity_y.set(-7.0);
                                gravity.set(-default_gravity);
                                jump_force.set(-default_jump_force);
                            } else {
                                velocity_y.set(7.0);
                                gravity.set(default_gravity);
                                jump_force.set(default_jump_force)
                            }
                        }
                        *on_orb = false
                    }

                    *is_on_ground = false
                }
            }

            5 | 6 | 8 | 9 | 24 | 17 | 18 | 19 | 20 => {
                if centered_player.overlaps(&Rect {
                    x: object.x as f32 - world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                    y: obj_y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 0.0 } else { -31.0 } - player_cam_y as f32,
                    w: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                    h: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
                }) {
                    if object.id == 5 {
                        jump_force.set(-default_jump_force);
                        gravity.set(-default_gravity);
                    } else if object.id == 6 {
                        jump_force.set(default_jump_force);
                        gravity.set(default_gravity);
                    } else if object.id == 8 {
                        *current_gamemode = GameMode::Cube;
                        *cc_1003 = GREEN;
                    } else if object.id == 9 {
                        *current_gamemode = GameMode::Ship;
                        *cc_1003 = MAGENTA
                    } else if object.id == 24 {
                        *current_gamemode = GameMode::Ball;
                        *cc_1003 = RED
                    } else if object.id == 17 {
                        movement_speed.set(default_movement_speed * 0.85);
                    } else if object.id == 18 {
                        movement_speed.set(default_movement_speed);
                    } else if object.id == 19 {
                        movement_speed.set(default_movement_speed * 1.4);
                    } else if object.id == 20 {
                        movement_speed.set(default_movement_speed * 1.8);
                    }

                    *is_on_ground = false
                }
            }

            7 => {
                *kill_player |= centered_player.overlaps(&Rect {
                    x: object.x as f32 - world_offset + 20.0,
                    y: object.y as f32 + if object.rotation > 145 || object.rotation < -145 { 5.0 } else { 25.0 } - player_cam_y as f32,
                    w: 10.0,
                    h: 10.0
                });
            }

            15 => {
                if centered_player.overlaps(&Rect {
                    x: object.x as f32 - world_offset,
                    y: obj_y,
                    w: 40.0,
                    h: 40.0
                }) {
                    game_state.set(GameState::LevelComplete)
                }
            }

            23 => {
                if centered_player.overlaps(&Rect {
                    x: object.x as f32 - world_offset,
                    y: obj_y - player_cam_y,
                    w: 40.0,
                    h: 40.0
                }) {
                    let red: u8 = object.properties.clone().unwrap()[0].parse().unwrap();
                    let green: u8 = object.properties.clone().unwrap()[1].parse().unwrap();
                    let blue: u8 = object.properties.clone().unwrap()[2].parse().unwrap();
                    let color: &String = &object.properties.clone().unwrap()[3];

                    if color == "1" {
                        *cc_1001 = Color::from_rgba(red, green, blue, 255);
                    } else if color == "2" {
                        *cc_1002 = Color::from_rgba(red, green, blue, 255);
                    }
                }
            }

            _ => {}
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

        if object.id == 2
        || object.id == 11
        || object.id == 12
        || object.id == 13
        || object.id == 14 {
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

        if object.id == 3
        || object.id == 21 {
            draw_rectangle_lines(
                object.x as f32 - world_offset,
                obj_y as f32 - player_cam_y as f32 + 35.0,
                40.0,
                5.0,
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            );
        }

        if object.id == 4
        || object.id == 22 {
            draw_rectangle_lines(
                object.x as f32 - 10.0 - world_offset,
                object.y as f32 - 10.0 - player_cam_y as f32,
                60.0,
                60.0,
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            )
        }

        if object.id == 5 || object.id == 6
        || object.id == 8 || object.id == 9
        || object.id == 17 || object.id == 18 || object.id == 19 || object.id == 20 {
            draw_rectangle_lines(
                object.x as f32 - world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                obj_y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 0.0 } else { -31.0 } - player_cam_y as f32,
                if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 },
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            )
        }

        if object.id == 7 {
            draw_rectangle_lines(
                object.x as f32 - world_offset + 20.0,
                object.y as f32 + if object.rotation > 145 || object.rotation < -145 { 5.0 } else { 25.0 } - player_cam_y as f32,
                10.0,
                10.0,
                2.0,
                RED
            );
        }

        if object.id == 15 {
            draw_rectangle_lines(
                object.x as f32 - world_offset,
                obj_y - player_cam_y,
                40.0,
                40.0,
                2.0,
                Color::from_rgba(0, 255, 255, 255)
            );
        }
    }

    draw_rectangle_lines(player.x, player.y, 40.0, 40.0, 2.0, WHITE);
    draw_rectangle_lines(small_player.x, small_player.y, small_player.w, small_player.h, 2.0, WHITE);
}