use amethyst::{
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


// Falling object component to bucket us into something the system can manipulate
#[derive(Clone)]
pub struct TiledScroller {
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub position: f32,
}
impl Component for TiledScroller {
    type Storage = DenseVecStorage<Self>;
}

// Falling object component to bucket us into something the system can manipulate
pub struct Birb {
    pub vertical_speed: f32,
    pub starting_height: f32,
    pub position: f32,
}
impl Component for Birb {
    type Storage = DenseVecStorage<Self>;
}
