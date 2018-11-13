use amethyst::utils::ortho_camera::CameraNormalizeMode;
use amethyst::utils::ortho_camera::CameraOrtho;
use amethyst::{
    assets::AssetStorage,
    core::{Parent, Transform},
    ecs::Entity,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
};
use component::Player;

pub struct Loader;

impl<'a, 'b> SimpleState<'a, 'b> for Loader {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        let background_sprite_sheet_handle =
            load_sprite_sheet(world, "Background.png", "Background.ron");

        crate::map::load_map_sprites(world);
        let _background = init_background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init_reference_sprite(world, &circle_sprite_sheet_handle);
        let parent = init_player(world, &circle_sprite_sheet_handle);
        init_camera(world, parent);
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_id = load_texture(world, png_path);

    let loader = world.read_resource::<amethyst::assets::Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_id,
        (),
        &sprite_sheet_store,
    )
}

/// Loads texture into world and returns texture id.
pub fn load_texture(world: &mut World, png_path: &str) -> u64 {
    let texture_handle = {
        let loader = world.read_resource::<amethyst::assets::Loader>();
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

    texture_id
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
    transform.translation.x = 32.0 * 70.0;
    transform.translation.y = 32.0 * 50.0;

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
