mod components;
mod systems;
mod ready_state;
mod play_state;
mod splash_state;

use amethyst::{
    input::{InputBundle, StringBindings},
    core::transform::TransformBundle,
    core::transform::Transform,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::*;
use crate::systems::*;
use std::path::PathBuf;
use std::str::FromStr;
use crate::splash_state::SplashState;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    // Gets the root directory of the application
    let mut app_root = PathBuf::from_str("/home/mrh/source/flappy-bird-rust/")?;

    // join on the resources path, and the config.
    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");
    let binding_path = resources.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
       // .with(System, "system", &["required_things"])
         .with(ScrollScrollables, "scroll", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config).unwrap()
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;


    // Creates the app with the startup state and bound game data
    let mut game = Application::new(resources, SplashState::default(), game_data)?;
    game.run();

    Ok(())
}
