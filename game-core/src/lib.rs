extern crate amethyst;

use amethyst::utils::ortho_camera::CameraNormalizeMode;
use amethyst::utils::ortho_camera::CameraOrtho;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::{Component, Entity, Join, NullStorage, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, Transparent,
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

use amethyst::renderer::Sprite;
use std::path::Path;

fn load_map_sprites(world: &mut World) {
    let file = Path::new("./resources/testmap.tmx");
    let map = tiled::parse_file(&file).unwrap();

    let tileset = &map.tilesets[0];
    let image = &tileset.images[0];

    let sprite_cords = |sprite_id| {
        let width = image.width as u32 - 2 * tileset.margin + tileset.spacing;
        let cols = width / (tileset.tile_width + tileset.spacing);

        let col = sprite_id % cols;
        let row = sprite_id / cols;

        let left = tileset.margin + tileset.tile_width * col + tileset.spacing * col;
        let top = tileset.margin + tileset.tile_height * row + tileset.spacing * row;

        (left, top)
    };

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            image.source.clone(),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let texture_id = {
        let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
        let texture_id = material_texture_set.len() as u64;
        material_texture_set.insert(texture_id, texture_handle);
        texture_id
    };

    let handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

        let mut sprites: Vec<Sprite> = Vec::with_capacity(tileset.tiles.len());

        for tile in &tileset.tiles {
            let (left, top) = sprite_cords(tile.id);

            let sprite = Sprite::from_pixel_values(
                image.width as u32,
                image.height as u32,
                tileset.tile_width,
                tileset.tile_height,
                left,
                top,
                [0, 0],
            );

            sprites.push(sprite);
        }

        loader.load_from_data(
            SpriteSheet {
                texture_id,
                sprites: sprites,
            },
            (),
            &sprite_sheet_store,
        )
    };

    let mut left = 0.0;
    let mut top = 0.0;

    let layer = &map.layers[0];
    for row in &layer.tiles {
        for tile_id in row {
            if *tile_id != 30 && *tile_id != 0 {
                let mut transform = Transform::default();
                transform.translation.z = -1.0;
                transform.translation.x = left;
                transform.translation.y = top;

                let sprite = SpriteRender {
                    sprite_sheet: handle.clone(),
                    sprite_number: *tile_id as usize - 1,
                    flip_horizontal: false,
                    flip_vertical: false,
                };

                world.create_entity().with(transform).with(sprite).build();
            }

            left += tileset.tile_width as f32;
        }
        left = 0.0;
        top -= tileset.tile_height as f32;
    }
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
    transform.translation.z = 2.0;
    transform.translation.x = -256.0;
    transform.translation.y = -256.0;
    transform.scale.x = 512.0;
    transform.scale.y = 512.0;

    world.register::<CameraOrtho>();

    world
        .create_entity()
        .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
        .with(Camera::standard_2d())
        .with(Parent { entity: parent })
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

        load_map_sprites(world);
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
