use amethyst::ecs::{Component, HashMapStorage};
use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent},
};
use crate::component::Animation;

pub struct Player {
    pub hp: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self { hp: 10 }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl Player {
    pub fn new(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
        let mut transform = Transform::default();
        transform.translation.x = 32.0 * 70.0;
        transform.translation.y = 32.0 * 50.0;

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 1,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let anim = Animation {
            total_frames: 8,
            max_count_till_next_frame: 0.5,
            frame_life_time_count: 0.5,
            current_frame: 0,
        };

        world
            .create_entity()
            .with(transform)
            .with(Player::default())
            .with(sprite)
            .with(Transparent)
            .with(anim)
            .build()
    }
}
