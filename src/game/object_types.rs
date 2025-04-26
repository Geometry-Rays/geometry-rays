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
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        2,
        "Block",
        load_texture("./Resources/objects/block.png")
            .await.expect("Failed to load block texture"),
        2,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        3,
        "Jump Pad",
        load_texture("./Resources/objects/pads/pad.png")
            .await.expect("Failed to load pad texture"),
        3,
        obj_btn_offset
    ));

    obj_types.push(ObjectType::new(
        4,
        "Jump Orb",
        load_texture("./Resources/objects/orbs/orb.png")
            .await.expect("Failed to load orb texture"),
        4,
        obj_btn_offset
    ));
}