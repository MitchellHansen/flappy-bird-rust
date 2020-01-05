use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::math::Vector3,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

use log::info;
use crate::components::*;
use std::collections::HashMap;

pub struct MyState;

impl SimpleState for MyState {

    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();


        // Place the camera
        /// function sets size of camera window
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {

        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
               // info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
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
        (0, "day-background".to_string()),
        (1, "night-background".to_string()),
        (2, "down-pipe".to_string()),
        (3, "up-pipe".to_string()),
        (4, "ground".to_string()),
        (5, "floppy".to_string()),
    ];

    sprite_map.iter()
        .map(|i| (i.1.clone(), SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i.0,
        }))
        .collect()
}

fn init_sprites(world: &mut World, sprites: &HashMap<String, SpriteRender>, dimensions: &ScreenDimensions) {

    let background_sprite = sprites.get("day-background").unwrap();


    let background_object = TiledScroller {
        speed: -75.0,
        position: 1.0,
        width: 144.0 * 3.0,
        height: 256.0 * 3.0,
    };

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(3.0, 3.0, 3.0));
    transform.set_translation_xyz(background_object.width/2.0, background_object.height/2.0, 0.0);


    world
        .create_entity()
        .with(background_sprite.clone()) // Sprite Render
        .with(background_object.clone())
        .with(transform.clone())
        .build();

    transform.set_translation_xyz(3.0*144.0/2.0*3.0, 3.0*256.0/2.0, 0.0);

    world
        .create_entity()
        .with(background_sprite.clone()) // Sprite Render
        .with(TiledScroller {
            speed: -75.0,
            position: 2.0,
            width: 0.0,
            height: 0.0
        })
        .with(transform.clone())
        .build();

    let ground_sprite = sprites.get("ground").unwrap();
    transform.set_translation_xyz(3.0*168.0/2.0*3.0, 3.0*56.0/2.0, 0.1);

    world
        .create_entity()
        .with(ground_sprite.clone()) // Sprite Render
        .with(TiledScroller {
            speed: -100.0,
            position: 2.0,
            width: 0.0,
            height: 0.0,
        })
        .with(transform.clone())
        .build();

    let birb_sprite = sprites.get("floppy").unwrap();
    transform.set_translation_xyz(dimensions.width()/2.0, dimensions.height()/2.0, 0.2);


    world
        .create_entity()
        .with(birb_sprite.clone()) // Sprite Render
        .with(Birb {
            vertical_speed: 0.0,
            position: 0.0,
            starting_height: 0.0
        })
        .with(transform)
        .build();

}
