use jumpy::platform::render::{backend::RenderBackend, gba::GbaRenderer};

pub fn run(mut gba: agb::Gba) -> ! {
	let mut renderer = GbaRenderer::new_with_gba(&mut gba);

	agb::println!("jumpy gba: alive");

	renderer.init();

	let mut gfx = gba.graphics.get();

	loop {
		agb::println!("tick GbaRenderer");
		renderer.commit();
	}
}
