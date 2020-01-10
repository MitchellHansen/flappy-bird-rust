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

#[derive(Default)]
pub struct SplashState {
    sprites: Vec<Entity>,
    persistent_sprites: Vec<Entity>,
}

impl SplashState {

    fn init_sprites(&mut self, world: &mut World) {
        let sprites = world.try_fetch_mut::<HashMap<String, SpriteRender>>().unwrap().clone();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let flappy_bird_text_sprite = sprites
            .get("flappy-bird-text").unwrap().clone();
        let play_button_sprite = sprites
            .get("play-button").unwrap().clone();
        let leaderboard_button_sprite = sprites
            .get("leaderboard-button").unwrap().clone();
        let background_sprite = sprites
            .get("day-background").unwrap().clone();
        let night_background_sprite = sprites
            .get("night-background").unwrap().clone();
        let ground_sprite = sprites
            .get("ground").unwrap().clone();


        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(3.0, 3.0, 3.0));
        transform.set_translation_xyz(3.0*143.0/2.0, 3.0*256.0/2.0, 0.0);

        self.persistent_sprites.push(world
            .create_entity()
            .with(background_sprite.clone()) // Sprite Render
            .with(TiledScroller {
                speed: -75.0,
                position: 1.0,
                width: 143.0 * 3.0,
                height: 256.0 * 3.0,
            })
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(3.0*143.0/2.0*3.0, 3.0*256.0/2.0, 0.0);

        self.persistent_sprites.push(world
            .create_entity()
            .with(background_sprite.clone()) // Sprite Render
            .with(TiledScroller {
                speed: -75.0,
                position: 2.0,
                width: 143.0 * 3.0,
                height: 256.0 * 3.0,
            })
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(3.0*168.0/2.0, 3.0*56.0/2.0, 0.1);

        self.persistent_sprites.push(world
            .create_entity()
            .with(ground_sprite.clone()) // Sprite Render
            .with(TiledScroller {
                speed: -100.0,
                position: 2.0,
                width: 167.0 * 3.0,
                height: 56.0 * 3.0,
            })
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(3.0*168.0/2.0*3.0, 3.0*56.0/2.0, 0.1);

        self.persistent_sprites.push(world
            .create_entity()
            .with(ground_sprite.clone()) // Sprite Render
            .with(TiledScroller {
                speed: -100.0,
                position: 2.0,
                width: 167.0 * 3.0,
                height: 56.0 * 3.0,
            })
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(dimensions.width()*0.5, dimensions.height()*0.8, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(flappy_bird_text_sprite.clone())
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(dimensions.width()*0.25, dimensions.height()*0.4, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(play_button_sprite.clone())
            .with(transform.clone())
            .build());

        transform.set_translation_xyz(dimensions.width()*0.75, dimensions.height()*0.4, 0.2);

        self.sprites.push(world
            .create_entity()
            .with(leaderboard_button_sprite.clone())
            .with(transform.clone())
            .build());
    }

    fn init_camera(world: &mut World) {

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Center the camera in the middle of the screen, and let it cover
        // the entire screen
        let mut transform = Transform::default();
        transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

        world
            .create_entity()
            .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
            .with(transform)
            .build();
    }

    fn load_sprites(world: &mut World) -> HashMap<String, SpriteRender> {
        // Load the texture for our sprites. We'll later need to
        // add a handle to this texture to our `SpriteRender`s, so
        // we need to keep a reference to it.
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/flappy.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        // Load the spritesheet definition file, which contains metadata on our
        // spritesheet texture.
        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/flappy.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };

        let sprite_map = vec![
            ("day-background".to_string(),     0),
            ("night-background".to_string(),   1),
            ("down-pipe".to_string(),          2),
            ("up-pipe".to_string(),            3),
            ("ground".to_string(),             4),
            ("floppy".to_string(),             5),
            ("tap-tap-dialogue".to_string(),   6),
            ("play-button".to_string(),        7),
            ("leaderboard-button".to_string(), 8),
            ("get-ready-text".to_string(),     9),
            ("flappy-bird-text".to_string(),   10),
        ];

        sprite_map.iter()
            .map(|i| (i.0.clone(), SpriteRender {
                sprite_sheet: sheet_handle.clone(),
                sprite_number: i.1,
            }))
            .collect()
    }
}

impl SimpleState for SplashState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the sprites. Insert them into the world as this is the first function to be called
        let sprites = SplashState::load_sprites(world);
        world.insert(sprites.clone());

        SplashState::load_sprites(world);
        SplashState::init_camera(world);
        SplashState::init_sprites(self, world);

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

            // Check if the window should be closed
            if is_key_down(&event, VirtualKeyCode::Space) {
                let world = data.world;
                for i in &self.sprites {
                    world.delete_entity(*i);
                }
                self.sprites.clear();

                return Trans::Push(Box::new(ReadyState::default()));
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

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