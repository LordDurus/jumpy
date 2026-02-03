use jumpy::platform::{
	level_loader::load_level_from_name,
	render::{backend::RenderBackend, gba::GbaRenderer},
};

pub fn run(mut gba: agb::Gba) -> ! {
	let mut renderer = GbaRenderer::new_with_gba(&mut gba);
	renderer.init();

	let bootstrap_level = load_level_from_name("../worlds/01/01.lvlb");
	agb::println!("loaded bootstrap level {:?}", bootstrap_level);

	loop {
		// agb::println!("tick");
		renderer.commit();
	}
}
