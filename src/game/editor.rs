use macroquad::prelude::*;
use crate::types::ObjectStruct;

pub fn keybind_handler(
    cam_pos_y: &mut f32,
    cam_pos_x: &mut f32
) {
    if is_key_down(KeyCode::Up) {
        *cam_pos_y += 1.0;
    }

    if is_key_down(KeyCode::Down) {
        *cam_pos_y -= 1.0;
    }

    if is_key_down(KeyCode::Left) {
        *cam_pos_x -= 1.0;
    }

    if is_key_down(KeyCode::Right) {
        *cam_pos_x += 1.0;
    }
}

pub fn object_ped(
    object_grid: &mut Vec<ObjectStruct>,

    snapped_x: i32,
    snapped_y: i32,
    cam_pos_x: f32,
    cam_pos_y: f32,
    grid_size: u8,

    current_tab: u8,
    current_obj: u16
) {
    if current_tab == 1 {
        object_grid.push(ObjectStruct {
            x: snapped_x,
            y: snapped_y,
            rotation: 0,
            id: current_obj,
            selected: false
        });
    } else if current_tab == 2 {
        let mouse_radius: Rect = Rect {
            x: mouse_position().0,
            y: mouse_position().1,
            w: grid_size as f32,
            h: grid_size as f32
        };

        let mut obj_index = 0;
        while obj_index < object_grid.len() {
            let obj_y = (screen_height() / 1.15 - 25.0) + (object_grid[obj_index].y as f32 - 500.0);
            if mouse_radius.contains(Vec2 {
                x: object_grid[obj_index].x as f32 - cam_pos_x * 5.0 + grid_size as f32 / 2.0,
                y: obj_y + cam_pos_y * 5.0 + grid_size as f32 / 2.0
            })
            && !object_grid[obj_index].selected {
                if !is_key_down(KeyCode::LeftShift) {
                    let mut objj_index = 0;
                    while objj_index < object_grid.len() {
                        object_grid[objj_index].selected = false;
                        objj_index += 1
                    }
                }

                object_grid[obj_index].selected = true;
                break;
            }

            obj_index += 1
        }
    } else {
        println!("wtf did you do bro");
    }
}