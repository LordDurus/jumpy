use crate::vector2::Vector2;
mod game;
mod physics;
mod vector2;
mod world;

use game::input::InputHandler;
use game::render::Renderer;
use std::time::Instant;

#[cfg(feature = "gba")]
use game::render::gba::GbaRenderer;

#[cfg(feature = "pc")]
use game::render::pc::PcRenderer;

#[cfg(feature = "pc")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "pc")]
type Shared<T> = Arc<Mutex<T>>;

#[cfg(feature = "pc")]
use rayon::prelude::*;

#[cfg(not(feature = "pc"))]
type Shared<T> = T;

fn main() {
    let input_handler: Shared<InputHandler> = Shared::new(Mutex::new(InputHandler::new()));
    let world: Shared<world::World> = Shared::new(Mutex::new(world::World::new()));

    let mut entity_ids = vec![];

    let mut world_lock = world.lock().unwrap(); // Acquire the lock on world
    // Create multiple entities and store their IDs
    for _ in 0..10 {
        let entity_id: u32 = world_lock.add_entity(
            vector2::MyVector2::new(100.0, 100.0),
            vector2::MyVector2::new(0.0, 0.0),
        );
        entity_ids.push(entity_id);
    }

    #[cfg(feature = "pc")]
    let mut renderer: PcRenderer = game::render::pc::PcRenderer::new();

    #[cfg(feature = "gba")]
    let mut renderer = game::render::gba::GbaRenderer::new();

    #[cfg(feature = "psp")]
    let mut renderer = game::render::psp::PspRenderer::new();

    renderer.init();

    run_game_loop(&world, &input_handler, &entity_ids, &mut renderer);
}

fn run_game_loop<R: Renderer>(
    world: &Shared<world::World>,
    input_handler: &Shared<InputHandler>,
    entity_ids: &[u32],
    renderer: &mut R,
) {
    let mut last_time = Instant::now();
    loop {
        let now = Instant::now();
        let _delta_time = (now - last_time).as_secs_f32();
        last_time = now;

        #[cfg(feature = "pc")]
        entity_ids.par_iter().for_each(|&entity_id| {
            let mut world = world.lock().unwrap();
            let mut input_handler = input_handler.lock().unwrap();

            input_handler.handle_input(&mut world, entity_id);
            physics::gravity::gravity_system(&mut world);
            game::movement::movement(&mut world);
        });

        #[cfg(not(feature = "pc"))]
        for &entity_id in entity_ids {
            input_handler
                .lock()
                .unwrap()
                .handle_input(&mut world.lock().unwrap(), entity_id);
            physics::gravity::gravity_system(&mut world.lock().unwrap());
            game::movement::movement(&mut world.lock().unwrap());
        }

        renderer.commit();
    }
}
