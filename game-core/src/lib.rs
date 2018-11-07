extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::{Component, Entity, Join, NullStorage, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.translation.x += x_move as f32 * 5.0;
            transform.translation.y += y_move as f32 * 5.0;
        }
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    let texture_id = material_texture_set.len() as u64;
    material_texture_set.insert(texture_id, texture_handle);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_id,
        (),
        &sprite_sheet_store,
    )
}

// Initialize a background
fn init_background_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.translation.z = -10.0;

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };
    world.create_entity().with(transform).with(sprite).build()
}

// Initialize a sprite as a reference point at a fixed location
fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.translation.x = 100.0;
    transform.translation.y = 0.0;
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

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
        flip_horizontal: false,
        flip_vertical: false,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .build()
}

fn init_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, 250.0, -250.0,
        ))).with(Parent { entity: parent })
        .with(transform)
        .build();
}

pub struct Example;

impl<'a, 'b> SimpleState<'a, 'b> for Example {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        let background_sprite_sheet_handle =
            load_sprite_sheet(world, "Background.png", "Background.ron");

        let _background = init_background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init_reference_sprite(world, &circle_sprite_sheet_handle);
        let parent = init_player(world, &circle_sprite_sheet_handle);
        init_camera(world, parent);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
