use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::math::Vector3,
    input::{get_mouse_button, get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ecs::prelude::{Dispatcher, DispatcherBuilder, Component, DenseVecStorage, Entity},
};

use log::info;
use crate::components::*;
use std::collections::HashMap;
use crate::systems::{BirbGravity, ScrollScrollables};
use crate::play_state::PlayState;

#[derive(Default)]
pub struct ReadyState {
    sprites: Vec<Entity>,
}

impl ReadyState {

    fn init_sprites(&mut self, world: &mut World) {

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let sprites = world.try_fetch_mut::<HashMap<String, SpriteRender>>().unwrap().clone();

        let get_ready_text_sprite = sprites
            .get("get-ready-text").unwrap().clone();

        let tap_tap_dialogue_sprite = sprites
            .get("tap-tap-dialogue").unwrap().clone();

        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(3.0, 3.0, 3.0));

        transform.set_translation_xyz(dimensions.width()*0.5, dimensions.height()*0.8, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(get_ready_text_sprite.clone()) // Sprite Render
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(dimensions.width()*0.5, dimensions.height()*0.5, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(tap_tap_dialogue_sprite.clone()) // Sprite Render
            .with(transform.clone())
            .build());
    }
}
impl SimpleState for ReadyState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        ReadyState::init_sprites(self, world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {

    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {

        if let StateEvent::Ui(event) = &event {

            if event.event_type == UiEventType::Click {

            }

        }

        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Check if the window should be closed
            if is_key_down(&event, VirtualKeyCode::Space) {
                let world = data.world;
                for i in &self.sprites {
                    world.delete_entity(*i);
                }
                self.sprites.clear();

                return Trans::Push(Box::new(PlayState::default()));
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}