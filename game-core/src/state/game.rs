use amethyst::prelude::*;
use component::Player;
use crate::load;

pub struct Game;

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.add_resource(load::LoadedTextures::default());

        let player_sprite_sheet_handle = load::sprite_sheet(world, "FRONT.png", "FRONT.ron");
        let _penguin_sprite_sheet_handle =
            load::sprite_sheet(world, "penguinFront.png", "penguinFront.ron");

        crate::map::load_map_sprites(world);
        let parent = Player::new(world, &player_sprite_sheet_handle);
        init::camera(world, parent);
    }
}

mod init {
    use amethyst::utils::ortho_camera::CameraNormalizeMode;
    use amethyst::utils::ortho_camera::CameraOrtho;
    use amethyst::{core::Transform, ecs::Entity, prelude::*, renderer::Camera};

    pub fn camera(world: &mut World, parent: Entity) {
        let mut transform = {
            let transforms = world.read_storage::<Transform>();
            transforms.get(parent).unwrap().clone()
        };

        transform.translation.z = 2.0;
        transform.translation.x -= 256.0;
        transform.translation.y -= 256.0;
        transform.scale.x = 512.0;
        transform.scale.y = 512.0;

        world.register::<CameraOrtho>();

        world
            .create_entity()
            .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
            .with(Camera::standard_2d())
            .with(transform)
            .build();
    }
}
