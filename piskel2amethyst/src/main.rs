#[macro_use]
extern crate serde_derive;

extern crate failure;
extern crate serde;
extern crate serde_json;

use failure::Error;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct PiskelSheet {
    frames: HashMap<String, PiskelImage>,
    meta: PiskelMeta,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct PiskelImage {
    frame: Piskelxywh,
    rotated: bool,
    trimmed: bool,
    spriteSourceSize: Piskelxywh,
    sourceSize: Piskelwh,
}

#[derive(Debug, Deserialize)]
struct PiskelMeta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: Piskelwh,
}

#[derive(Debug, Deserialize)]
struct Piskelxywh {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize)]
struct Piskelwh {
    w: u32,
    h: u32,
}

#[derive(Debug, Serialize)]
struct AmethystSheet {
    spritesheet_width: f64,
    spritesheet_height: f64,
    sprites: Vec<AmethystSprite>,
}

#[derive(Debug, Serialize)]
struct AmethystSprite {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn main() -> Result<(), Error> {
    // Open the file in read-only mode.
    let path = std::env::args().nth(1).unwrap();
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let u: PiskelSheet = serde_json::from_reader(file)?;

    let mut out = AmethystSheet {
        spritesheet_width: f64::from(u.meta.size.w),
        spritesheet_height: f64::from(u.meta.size.h),
        sprites: vec![],
    };

    for sprite in u.frames.values() {
        out.sprites.push(AmethystSprite {
            x: f64::from(sprite.frame.x),
            y: f64::from(sprite.frame.y),
            width: f64::from(sprite.frame.w),
            height: f64::from(sprite.frame.h),
        });
    }

    println!(
        "{}",
        ron::ser::to_string_pretty(&out, ron::ser::PrettyConfig::default())?
    );

    Ok(())
}
