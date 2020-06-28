use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::math::Vector3,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ecs::prelude::{Dispatcher, DispatcherBuilder, Component, DenseVecStorage, Entity},
};

use log::info;
use crate::components::*;
use std::collections::HashMap;
use crate::systems::{BirbGravity, ScrollScrollables};

#[derive(Default)]
pub struct PlayState<'a, 'b> {

    // Custom dispatch systems for this state
    dispatcher: Option<Dispatcher<'a, 'b>>,

    sprites: Vec<Entity>,
}

impl<'a, 'b> PlayState<'a, 'b> {

    fn init_sprites(&mut self, world: &mut World) {

        let sprites = world.try_fetch_mut::<HashMap<String, SpriteRender>>().unwrap().clone();
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let birb_sprite = sprites
            .get("floppy").unwrap();

        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(3.0, 3.0, 3.0));
        transform.set_translation_xyz(dimensions.width()/2.0, dimensions.height()/2.0, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(birb_sprite.clone()) // Sprite Render
            .with(Birb {
                vertical_speed: 0.0,
                position: 0.0,
                starting_height: 0.0
            })
            .with(transform)
            .build());
    }
}

impl<'a, 'b> SimpleState for PlayState<'a, 'b> {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Create the `DispatcherBuilder` and register some `System`s that should only run for this `State`.
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(BirbGravity { fired: false }, "gravity", &[]);

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        PlayState::init_sprites(self, world);

    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {

        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::P) {
                let world = data.world;
                for i in &self.sprites {
                    world.delete_entity(*i);
                }
                self.sprites.clear();
                return Trans::Pop;
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}
