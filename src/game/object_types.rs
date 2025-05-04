use macroquad::prelude::load_texture;

use crate::types::ObjectType;

pub async fn create_object_types(
    obj_types: &mut Vec<ObjectType>,
    obj_btn_offset: f32
) {
    obj_types.push(ObjectType::new(
        1,
        "Spike",
        load_texture("./Resources/objects/spike.png")
            .await.expect("Failed to load spike texture"),
        1,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        2,
        "Block",
        load_texture("./Resources/objects/block.png")
            .await.expect("Failed to load block texture"),
        2,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        3,
        "Jump Pad",
        load_texture("./Resources/objects/pads/pad.png")
            .await.expect("Failed to load pad texture"),
        3,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        4,
        "Jump Orb",
        load_texture("./Resources/objects/orbs/orb.png")
            .await.expect("Failed to load orb texture"),
        4,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        5,
        "Upside Down Portal",
        load_texture("./Resources/objects/portals/upside-down-portal.png")
            .await.expect("Failed to load upside down portal texture"),
        5,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        6,
        "Right Side Up Portal",
        load_texture("./Resources/objects/portals/right-side-up-portal.png")
            .await.expect("Failed to load right side up portal texture"),
        6,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        7,
        "Short Spike",
        load_texture("./Resources/objects/short-spike.png")
            .await.expect("Failed to load short spike texture"),
        7,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        8,
        "Cube Portal",
        load_texture("./Resources/objects/portals/cube-portal.png")
            .await.expect("Failed to load cube portal texture"),
        8,
        0,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        9,
        "Ship Portal",
        load_texture("./Resources/objects/portals/ship-portal.png")
            .await.expect("Failed to load ship portal texture"),
        1,
        1,
        obj_btn_offset
    ));
}