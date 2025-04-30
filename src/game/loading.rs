use std::collections::HashMap;

use macroquad::prelude::Color;

use crate::types::ObjectStruct;

pub fn load_level(
    level_data: String,
    obj_grid: &mut Vec<ObjectStruct>,

    cc_1001: &mut Color,
    cc_1002: &mut Color
) -> String {
    let parts: Vec<&str> = level_data.split(";;;").collect();

    let metadata_pairs: Vec<&str> = parts[0].split(";").collect();
    let objects: Vec<&str> = parts[1].split(";;").collect();

    // This isn't used yet
    // It will be useful once object data gets changed in a future update
    // Probably when groups get added
    let mut _level_version: &str = "";

    obj_grid.clear();
    for pair in metadata_pairs {
        let key: &str = pair.split(":").collect::<Vec<&str>>()[0];
        let value: &str = pair.split(":").collect::<Vec<&str>>()[1];

        if key == "version" {
            if value == "F-ALPHA" {
                _level_version = value;
            } else {
                return "invalid_version".to_string();
            }
        } else if key == "cc_1001" {
            let rgb: Vec<&str> = value.split(",").collect();

            *cc_1001 = Color {
                r: rgb[0].parse().unwrap(),
                g: rgb[1].parse().unwrap(),
                b: rgb[2].parse().unwrap(),
                a: 1.0
            }
        } else if key == "cc_1002" {
            let rgb: Vec<&str> = value.split(",").collect();

            *cc_1002 = Color {
                r: rgb[0].parse().unwrap(),
                g: rgb[1].parse().unwrap(),
                b: rgb[2].parse().unwrap(),
                a: 1.0
            }
        }
    }

    for object in objects {
        let object_data: Vec<&str> = object.split(";").collect();

        let mut object_values: HashMap<&str, i32> = HashMap::new();

        for pair in object_data {
            let key: &str = pair.split(":").collect::<Vec<&str>>()[0];
            let value: &str = pair.split(":").collect::<Vec<&str>>()[1];

            object_values.insert(key, value.parse().unwrap());
        }

        obj_grid.push(ObjectStruct {
            x: object_values["x"],
            y: object_values["y"],
            rotation: object_values["rot"] as i16,
            id: object_values["id"] as u16,
            selected: false
        });
    }

    return "ok".to_string();
}