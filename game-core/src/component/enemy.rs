use amethyst::ecs::{Component, NullStorage};
use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent},
};

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

impl Enemy {
    pub fn new(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
        let mut transform = Transform::default();
        transform.translation.x = 32.0 * 75.0;
        transform.translation.y = 32.0 * 50.0;

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        world
            .create_entity()
            .with(transform)
            .with(Enemy)
            .with(sprite)
            .with(Transparent)
            .build()
    }
}
