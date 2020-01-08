use amethyst::{
    core::SystemDesc,
    core::timing::Time,
    core::transform::{Transform, TransformBundle},
    derive::SystemDesc,
    ecs::prelude::{},
    ecs::prelude::{
        Component, DenseVecStorage, Entity, Join, Read,
        ReadStorage, System, SystemData, WriteStorage
    },
    input::{InputHandler, StringBindings},
};

use log::info;
use crate::components::*;

pub struct ScrollScrollables;

// This system iterates all the objects with transform (and falling object) component
impl<'a> System<'a> for ScrollScrollables {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, TiledScroller>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, mut scrolling, time): Self::SystemData) {
        for (mut transform, mut object) in (&mut transforms, &mut scrolling).join() {

            // I need to tile it by it's width
            transform.prepend_translation_x(object.speed * time.delta_seconds());
            if transform.translation().x+144.0*1.5 < 0.0 {
                transform.set_translation_x(144.0*3.0/2.0*3.0);
            }
        }
    }
}

pub struct BirbGravity {
    pub fired: bool,
}

// This system iterates all the objects with transform (and falling object) component
impl<'a> System<'a> for BirbGravity {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Birb>,
        Read<'a, Time>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut scrolling, time, input): Self::SystemData) {
        for (mut transform, mut object) in (&mut transforms, &mut scrolling).join() {

            if input.action_is_down("flap").expect("No action") {
                object.vertical_speed = 600.0;
            }
            object.vertical_speed -= 1500.0 * time.delta_seconds();
            transform.prepend_translation_y(object.vertical_speed * time.delta_seconds());
        }
    }
}