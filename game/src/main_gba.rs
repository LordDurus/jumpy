use jumpy::{
	debugln,
	platform::{
		level_loader::load_level_from_name,
		render::{BackgroundDrawParams, BackgroundId, backend::RenderBackend, gba::GbaRenderer},
	},
};

pub fn run(mut gba: agb::Gba) -> ! {
	let mut renderer = GbaRenderer::new_with_gba(&mut gba);
	renderer.init();

	let bootstrap_level = load_level_from_name("../worlds/01/01.lvlb");
	let background_id = bootstrap_level.unwrap().background_id;

	let background_draw_params: BackgroundDrawParams = BackgroundDrawParams {
		background_id: background_id,
		camera_left: 0,
		camera_top: 0,
		scale: 4.0,
	};

	debugln!("Before Drawing background... {}", renderer.loaded_background.to_u8());
	renderer.draw_background(&background_draw_params);
	debugln!("After Drawing background...{}", renderer.loaded_background.to_u8());

	loop {
		// agb::println!("background_id: {}", background_draw_params.background_id.to_u8());
		renderer.draw_background(&background_draw_params);
		renderer.commit();
	}
}
