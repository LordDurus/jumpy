use jumpy::platform::{
	level_loader::load_level_from_name,
	render::{backend::RenderBackend, gba::GbaRenderer},
};

pub fn run(mut gba: agb::Gba) -> ! {
	let mut renderer = GbaRenderer::new_with_gba(&mut gba);

	//let bootstrap_level = load_level_from_file(path);
	let bootstrap_level = load_level_from_name("../worlds/01/01.lvlb");
	agb::println!("loaded bootstrap level");
	//agb::println!("bootstrap_level size: {}", bootstrap_level.unwrap().data.len());
	renderer.init();
	let mut gfx = gba.graphics.get();

	loop {
		// agb::println!("tick");
		renderer.commit();
	}
}
