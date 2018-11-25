extern crate game_core;

use std::env;

fn main() {
    if let Err(env::VarError::NotPresent) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "debug,gfx_device_gl=warn,amethyst_assets=warn");
    }

    env_logger::init();

    game_core::run().unwrap();
}
