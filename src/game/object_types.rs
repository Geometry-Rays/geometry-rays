use std::collections::HashMap;
use futures::executor::block_on;

use crate::types::ObjectType;

pub fn create_object_types(
    obj_types: &mut HashMap<u16, ObjectType>,
    obj_btn_offset: f32
) {
    obj_types.insert(
        1,
        block_on(
            ObjectType::new(
                1,
                "Spike",
                "./Resources/objects/spike.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        2,
        block_on(
            ObjectType::new(
                2,
                "Block",
                "./Resources/objects/blocks/block.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        3,
        block_on(
            ObjectType::new(
                3,
                "Jump Pad",
                "./Resources/objects/pads/pad.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        21,
        block_on(
            ObjectType::new(
                21,
                "Gravity Pad",
                "./Resources/objects/pads/gravity-pad.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        4,
        block_on(
            ObjectType::new(
                4,
                "Jump Orb",
                "./Resources/objects/orbs/orb.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        22,
        block_on(
            ObjectType::new(
                22,
                "Gravity Orb",
                "./Resources/objects/orbs/gravity-orb.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        5,
        block_on(
            ObjectType::new(
                5,
                "Upside Down Portal",
                "./Resources/objects/portals/upside-down-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        6,
        block_on(
            ObjectType::new(
                6,
                "Right Side Up Portal",
                "./Resources/objects/portals/right-side-up-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        7,
        block_on(
            ObjectType::new(
                7,
                "Short Spike",
                "./Resources/objects/short-spike.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        8,
        block_on(
            ObjectType::new(
                8,
                "Cube Portal",
                "./Resources/objects/portals/cube-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        9,
        block_on(
            ObjectType::new(
                9,
                "Ship Portal",
                "./Resources/objects/portals/ship-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        24,
        block_on(
            ObjectType::new(
                24,
                "Ball Portal",
                "./Resources/objects/portals/ball-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        25,
        block_on(
            ObjectType::new(
                25,
                "Wave Portal",
                "./Resources/objects/portals/wave-portal.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        10,
        block_on(
            ObjectType::new(
                10,
                "Outline Block 1",
                "./Resources/objects/blocks/outline-block.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    // This adds all the outline blocks
    // I was gonna add them all manually
    // But you gotta think smarter not harder
    for i in 0..4 {
        obj_types.insert(
            i + 11,
            block_on(
                ObjectType::new(
                    i + 11,
                    &format!("Outline Block {}", i + 2),
                    &format!("./Resources/objects/blocks/outline-block-variant{}.png", i + 1),
                    obj_btn_offset,
                    obj_types.len() as u16
                )
            )
        );
    }

    obj_types.insert(
        15,
        block_on(
            ObjectType::new(
                15,
                "End Trigger",
                "./Resources/objects/triggers/end-trigger.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        16,
        block_on(
            ObjectType::new(
                16,
                "Black Block",
                "./Resources/objects/blocks/black-block.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        17,
        block_on(
            ObjectType::new(
                17,
                "0.5x speed portal",
                "./Resources/objects/portals/speed/05x.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        18,
        block_on(
            ObjectType::new(
                18,
                "1x speed portal",
                "./Resources/objects/portals/speed/1x.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        19,
        block_on(
            ObjectType::new(
                19,
                "2x speed portal",
                "./Resources/objects/portals/speed/2x.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        20,
        block_on(
            ObjectType::new(
                20,
                "3x speed portal",
                "./Resources/objects/portals/speed/3x.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );

    obj_types.insert(
        23,
        block_on(
            ObjectType::new(
                23,
                "Color Trigger",
                "./Resources/objects/triggers/color-trigger.png",
                obj_btn_offset,
                obj_types.len() as u16
            )
        )
    );
}