use amethyst::prelude::*;
use component::{Enemy, Player};
use crate::load;

pub struct Game;

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load::sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        let background_sprite_sheet_handle =
            load::sprite_sheet(world, "Background.png", "Background.ron");

        crate::map::load_map_sprites(world);
        let _background = init::background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init::reference_sprite(world, &circle_sprite_sheet_handle);
        let parent = Player::new(world, &circle_sprite_sheet_handle);
        let _enemy = Enemy::new(world, &circle_sprite_sheet_handle);
        init::camera(world, parent);
    }
}

mod init {
    use amethyst::utils::ortho_camera::CameraNormalizeMode;
    use amethyst::utils::ortho_camera::CameraOrtho;
    use amethyst::{
        core::{Parent, Transform},
        ecs::Entity,
        prelude::*,
        renderer::{Camera, SpriteRender, SpriteSheetHandle, Transparent},
    };
    // Initialize a background
    pub fn background_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
        let mut transform = Transform::default();
        transform.set_z(-10.0);

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };
        world.create_entity().with(transform).with(sprite).build()
    }

    // Initialize a sprite as a reference point at a fixed location
    pub fn reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
        let mut transform = Transform::default();
        transform.set_x(100.0);

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        world
            .create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build()
    }

    pub fn camera(world: &mut World, parent: Entity) {
        let mut transform = Transform::default();
        transform
            .set_z(2.0)
            .set_x(-256.0)
            .set_y(-256.0)
            .set_scale(512.0, 512.0, 1.0);

        world.register::<CameraOrtho>();

        world
            .create_entity()
            .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
            .with(Camera::standard_2d())
            .with(Parent { entity: parent })
            .with(transform)
            .build();
    }
}
