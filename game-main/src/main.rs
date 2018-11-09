extern crate amethyst;
extern crate game_core;

use amethyst::{
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage, ALPHA,
    },
    utils::application_root_dir,
};
use game_core::Example;
use game_core::MovementSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
        )?.with(MovementSystem, "movement", &[])
        .with(
            amethyst::utils::ortho_camera::CameraOrthoSystem::default(),
            "OrthoCamera",
            &[],
        ).with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;

    let mut game = Application::build(root, Example)?.build(game_data)?;
    game.run();
    Ok(())
}
