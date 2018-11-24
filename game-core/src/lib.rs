#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::type_complexity, clippy::new_ret_no_self)
)]
extern crate amethyst;
extern crate rand;

mod component;
mod load;
mod map;
mod state;
mod system;

use amethyst::{
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage, ALPHA,
    },
    utils::application_root_dir,
};
use state::Menu;

pub fn run() -> amethyst::Result<()> {
    let root = format!("{}/resources", application_root_dir());
    let config = DisplayConfig::load(format!("{}/display_config.ron", root));
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite), // Tells the pipeline to respect sprite z-depth
            )),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<String, String>::new()
                .with_bindings_from_file(format!("{}/input.ron", root))?,
        )?.with(
            amethyst::utils::ortho_camera::CameraOrthoSystem::default(),
            "OrthoCamera",
            &[],
        ).with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;

    let mut game = Application::build(root, Menu)?.build(game_data)?;
    game.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
