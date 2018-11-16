use amethyst::renderer::Sprite;
use amethyst::utils::application_root_dir;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};
use crate::load;
use std::path::Path;

pub fn load_map_sprites(world: &mut World) {
    let fname = format!("{}/resources/testmap.tmx", application_root_dir());
    let file = Path::new(&fname);
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

    let texture = load::texture(world, &image.source);

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
                texture,
                sprites: sprites,
            },
            (),
            &sprite_sheet_store,
        )
    };

    let mut left = (tileset.tile_width / 2) as f32;
    let mut top = (tileset.tile_height / 2) as f32;

    let layer = &map.layers[0];
    let mut passable: Vec<Vec<bool>> = Vec::with_capacity(layer.tiles.len());

    for row in layer.tiles.iter().rev() {
        let mut passable_row: Vec<bool> = Vec::with_capacity(row.len());

        for tile_id in row {
            passable_row.push(*tile_id != 30);

            if *tile_id != 30 && *tile_id != 0 {
                let mut transform = Transform::default();
                transform.set_z(-1.0).set_x(left).set_y(top);

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

        passable.push(passable_row);

        left = (tileset.tile_width / 2) as f32;
        top += tileset.tile_height as f32;
    }

    world.add_resource(PassableTiles {
        tile_matrix: passable,
    });
}

pub struct PassableTiles {
    pub tile_matrix: Vec<Vec<bool>>,
}
