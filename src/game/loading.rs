use std::collections::HashMap;

use macroquad::prelude::Color;

use crate::types::{MainLevel, ObjectStruct};

pub fn load_level(
    level_data: String,
    obj_grid: &mut Vec<ObjectStruct>,

    cc_1001: &mut Color,
    cc_1002: &mut Color,

    current_song: &mut String,
    load_song: bool,
    main_levels: Vec<MainLevel>
) -> String {
    let parts: Vec<&str> = level_data.split(";;;").collect();

    let metadata_pairs: Vec<&str> = parts[0].split(";").collect();
    let objects: Vec<&str> = parts[1].split(";;").collect();
    let legacy_objects: Vec<&str> = parts[1].split(";").collect();

    // This isn't used yet
    // It will be useful once object data gets changed in a future update
    // Probably when groups get added
    let mut level_version: &str = "";

    let valid_versions: Vec<&str> = vec![
        "BETA",
        "1.3",
        "1.4",
        "1.5",
        "1.6",
        "F-ALPHA"
    ];

    obj_grid.clear();
    for pair in metadata_pairs {
        let key: &str = pair.split(":").collect::<Vec<&str>>()[0];
        let value: &str = pair.split(":").collect::<Vec<&str>>()[1];

        if key == "version" {
            if valid_versions.contains(&value) {
                level_version = value;
            } else {
                return "invalid_version".to_string();
            }
        } else if key == "cc_1001"
        || key == "c1001" {
            let rgb: Vec<&str> = value.split(",").collect();

            *cc_1001 = Color {
                r: if key == "cc_1001" { rgb[0].parse().unwrap() } else { rgb[0].parse::<f32>().unwrap() / 255.0 },
                g: if key == "cc_1001" { rgb[1].parse().unwrap() } else { rgb[1].parse::<f32>().unwrap() / 255.0 },
                b: if key == "cc_1001" { rgb[2].parse().unwrap() } else { rgb[2].parse::<f32>().unwrap() / 255.0 },
                a: 1.0
            }
        } else if key == "cc_1002"
        || key == "c1002" {
            let rgb: Vec<&str> = value.split(",").collect();

            *cc_1002 = Color {
                r: if key == "cc_1002" { rgb[0].parse().unwrap() } else { rgb[0].parse::<f32>().unwrap() / 255.0 },
                g: if key == "cc_1002" { rgb[1].parse().unwrap() } else { rgb[1].parse::<f32>().unwrap() / 255.0 },
                b: if key == "cc_1002" { rgb[2].parse().unwrap() } else { rgb[2].parse::<f32>().unwrap() / 255.0 },
                a: 1.0
            }
        } else if key == "song" && load_song {
            *current_song = if level_version.starts_with("F-") {
                value.to_string()
            } else {
                main_levels[value.parse::<usize>().unwrap()].song.to_string()
            }
        }
    }

    if level_version.starts_with("F-") {
        for object in objects {
            let object_data: Vec<&str> = object.split(";").collect();

            let mut object_values: HashMap<&str, &str> = HashMap::new();

            for pair in object_data {
                let key: &str = pair.split(":").collect::<Vec<&str>>()[0];
                let value: &str = pair.split(":").collect::<Vec<&str>>()[1];

                object_values.insert(key, value);
            }

            let mut properties_vec: Vec<&str> = vec![];

            if object_values["id"] == "23" {
                properties_vec = object_values["props"].split(",").collect();
            }

            obj_grid.push(ObjectStruct {
                x: object_values["x"].parse().unwrap(),
                y: object_values["y"].parse().unwrap(),
                rotation: object_values["rot"].parse().unwrap(),
                no_touch: 0,
                hide: 0,
                id: object_values["id"].parse().unwrap(),
                selected: false,
                properties: if object_values["id"] == "23" {
                    Some(
                        vec![
                            properties_vec[0].to_string(),
                            properties_vec[1].to_string(),
                            properties_vec[2].to_string(),
                            properties_vec[3].to_string()
                        ]
                    )
                } else {
                    None
                }
            });
        }
    } else {
        println!("Loading a level made in the old client!");

        for object in legacy_objects {
            let xyrid: Vec<&str> = object.split(':').collect();
            let obj_id_whar: u16 = if level_version == "BETA" { xyrid[3].parse().unwrap() } else { xyrid[5].parse().unwrap() };

            let obj_id: u16 = if obj_id_whar == 20 {
                17
            } else if obj_id_whar == 19 {
                20
            } else if obj_id_whar == 17 {
                18
            } else if obj_id_whar == 18 {
                19
            } else {
                obj_id_whar
            };

            if !parts[1].is_empty() {
                obj_grid.push(ObjectStruct {
                    y: xyrid[0].parse::<i32>().unwrap(),
                    x: xyrid[1].parse::<i32>().unwrap(),
                    rotation: xyrid[2].parse::<i16>().unwrap(),
                    no_touch: if level_version == "BETA" { 0 } else { xyrid[3].parse().unwrap() },
                    hide: if level_version == "BETA" { 0 } else { xyrid[4].parse().unwrap() },
                    id: obj_id,
                    selected: false,
                    properties: if obj_id == 23 && level_version != "BETA" {Some(
                        vec![
                            xyrid[6].to_string(),
                            xyrid[7].to_string(),
                            xyrid[8].to_string(),
                            xyrid[9].to_string()
                        ]
                    )} else {
                        None
                    }
                });
            }
        }
    }

    return "ok".to_string();
}