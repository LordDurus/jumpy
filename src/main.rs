mod game;
mod physics;
mod render;
mod vector2;
mod world;

use render::Renderer;

#[cfg(feature = "gba")]
use render::gba::GbaRenderer;

#[cfg(feature = "pc")]
use render::pc::PcRenderer;

fn main() {
    // Create the world
    let mut world = world::World::new();

    // Define the renderer (compile-time conditional)
    #[cfg(feature = "gba")]
    let mut renderer = GbaRenderer::new();

    #[cfg(feature = "pc")]
    let mut renderer = PcRenderer::new();

    // Initialize the renderer
    renderer.init();

    // Run the game loop
    run_game_loop(&mut world, &mut renderer);
}

fn run_game_loop<R: Renderer>(world: &mut world::World, renderer: &mut R) {
    loop {
        game::movement::movement(world);
        physics::gravity::gravity_system(world);
        renderer.render_frame(world);
    }
}
