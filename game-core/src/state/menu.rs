use amethyst::{
    core::Transform,
    ecs::prelude::*,
    ecs::Entity,
    prelude::*,
    renderer::Camera,
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho},
};
use component::{Animation, Player};
use crate::load;
use crate::state::Game;
use crate::system::*;

pub struct Menu;

impl SimpleState<'static, 'static> for Menu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.register::<Player>();
        world.register::<Animation>();

        world.add_resource(load::LoadedTextures::default());

        let player_sprite_sheet_handle = load::sprite_sheet(world, "FRONT.png", "FRONT.ron");
        let _penguin_sprite_sheet_handle =
            load::sprite_sheet(world, "penguinFront.png", "penguinFront.ron");
        let _ = load::sprite_sheet(world, "bubble.png", "bubble.ron");

        crate::map::load_map_sprites(world);
        let parent = Player::new(world, &player_sprite_sheet_handle);
        init_camera(world, parent);
    }

    fn update(
        &mut self,
        data: &mut StateData<GameData>,
    ) -> Trans<GameData<'static, 'static>, StateEvent> {
        let world = &mut data.world;
        let mut dispatcher: Dispatcher = DispatcherBuilder::new()
            .with(player::Movement, "player-movement", &[])
            .with(enemy::Movement, "enemy-movement", &[])
            .with(camera::Movement, "camera-movement", &[])
            .with(enemy::Spawner, "enemy-spawner", &[])
            .with(ally::Movement, "ally-movement", &[])
            .with(ally::Grouper, "ally-grouper", &[])
            .with(ally::Spawner, "ally-spawner", &[])
            .with(player::Attack, "player-attack", &[])
            .with(animation::Frame, "frame-animation", &[])
            .with(projectile::Movement, "projectile-movement", &[])
            .build();
        dispatcher.setup(&mut world.res);
        Trans::Push(Box::new(Game { dispatcher }))
    }
}

pub fn init_camera(world: &mut World, parent: Entity) {
    let mut transform = {
        let transforms = world.read_storage::<Transform>();
        transforms.get(parent).unwrap().clone()
    };

    world.register::<CameraOrtho>();

    transform.translation.z = 2.0;
    transform.translation.x -= 256.0;
    transform.translation.y -= 256.0;
    transform.scale.x = 512.0;
    transform.scale.y = 512.0;

    world
        .create_entity()
        .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
        .with(Camera::standard_2d())
        .with(transform)
        .build();
}
