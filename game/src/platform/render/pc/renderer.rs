use super::window::{WindowSettings, load_window_settings, save_window_settings};
use crate::{
	GameSession, GameState, RenderBackend,
	assets::{get_font_path, get_gfx_root},
	engine_math::Vec2,
	game::game_state::EntityKind,
	platform::{input::InputState, render::common::RenderCommon},
};
use sdl2::{
	EventPump,
	image::LoadTexture,
	pixels::Color,
	render::{BlendMode, Canvas, Texture},
	ttf::{Font, Sdl2TtfContext},
	video::Window,
};

use std::path::PathBuf;

#[derive(Clone, Copy)]
pub(crate) enum SlimeTextureKey {
	BlueWalk,
	BlueRun,
	BlueDeath,
	UndeadWalk,
	UndeadRun,
	UndeadDeath,
	LavaWalk,
	LavaRun,
	LavaDeath,
}

pub struct PcRenderer {
	pub video: sdl2::VideoSubsystem,
	pub canvas: Canvas<Window>,
	pub event_pump: EventPump,
	pub common: RenderCommon,
	pub slime_blue_walk_texture: Texture<'static>,
	pub slime_blue_run_texture: Texture<'static>,
	pub slime_blue_death_texture: Texture<'static>,
	pub slime_undead_walk_texture: Texture<'static>,
	pub slime_undead_run_texture: Texture<'static>,
	pub slime_undead_death_texture: Texture<'static>,
	pub slime_lava_walk_texture: Texture<'static>,
	pub slime_lava_run_texture: Texture<'static>,
	pub slime_lava_death_texture: Texture<'static>,
	pub frame_index: u32,
	pub atlas_tile_width_pixels: u32,
	pub atlas_tile_height_pixels: u32,
	pub tile_texture: Option<Texture<'static>>,

	bg_id: u8,
	pub bg_texture: Option<Texture<'static>>,
	pub bg_parallax_x: f32,
	pub bg_parallax_y: f32,
	render_scale: u32,
	pub texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext>,
	pub font: Font<'static, 'static>,
	pub trigger_texture: Texture<'static>,
}

impl Drop for PcRenderer {
	fn drop(&mut self) {
		save_window_settings(self.canvas.window());
		return;
	}
}

impl PcRenderer {
	pub fn screen_size_pixels(&self) -> (u32, u32) {
		let (width_pixels, height_pixels) = self.canvas.window().size();
		return (width_pixels, height_pixels);
	}

	pub fn set_level_background(&mut self, background_id: u8) {
		if self.bg_texture.is_some() && self.bg_id == background_id {
			return;
		}

		self.bg_id = background_id;
		let file_name = parse_background_id(background_id);
		let bg_path = gfx_pc_path(&["background", file_name]);

		let bg_texture = load_texture(&self.texture_creator, bg_path);
		self.bg_texture = Some(bg_texture);

		if let Some(background) = self.bg_texture.as_mut() {
			background.set_blend_mode(BlendMode::Blend);
		}

		return;
	}
}

impl RenderBackend for PcRenderer {
	fn init(&mut self) {
		// Nothing yet
		return;
	}

	fn get_render_scale(&self) -> f32 {
		return self.render_scale as f32;
	}

	fn new() -> Self {
		let sdl = sdl2::init().unwrap();
		let _ = sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0"); // nearest

		let _image = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
		let video = sdl.video().unwrap();
		let dm = video.desktop_display_mode(0).expect("desktop_display_mode failed");

		// let desktop_width_pixels: u32 = dm.w as u32;

		let desktop_height_pixels: u32 = dm.h as u32;
		let target_aspect: f32 = 16.0 / 9.0;

		let saved: Option<WindowSettings> = load_window_settings();

		let (window_width_pixels, window_height_pixels) = if let Some(s) = saved {
			(s.width_pixels, s.height_pixels)
		} else {
			let mut window_height_pixels: u32 = ((desktop_height_pixels as f32) * 0.80) as u32;
			if window_height_pixels < 360 {
				window_height_pixels = 360;
			}

			let window_width_pixels: u32 = ((window_height_pixels as f32) * target_aspect).round() as u32;
			(window_width_pixels, window_height_pixels)
		};

		let mut window = video
			.window("jumpy", window_width_pixels, window_height_pixels)
			.position_centered()
			.resizable()
			.build()
			.expect("window build failed");

		if let Some(s) = saved {
			window.set_position(sdl2::video::WindowPos::Positioned(s.left), sdl2::video::WindowPos::Positioned(s.top));
			if s.is_maximized {
				window.maximize();
			}
		}

		let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		canvas.set_blend_mode(BlendMode::Blend);

		let event_pump = sdl.event_pump().unwrap();
		let texture_creator = leak_texture_creator(&canvas);

		let ttf_box = Box::new(sdl2::ttf::init().unwrap());
		let ttf: &'static Sdl2TtfContext = Box::leak(ttf_box);

		let font_path: PathBuf = get_font_path().join("DejaVuSansMono.ttf");
		let mut font = ttf.load_font(font_path, 12).unwrap();
		font.set_style(sdl2::ttf::FontStyle::NORMAL);

		let tile_path = gfx_pc_path(&["tiles", "tiles64.png"]);
		let tile_texture = load_texture(&texture_creator, tile_path);

		let slime_blue_walk_path = gfx_pc_path(&["slime", "blue", "walk_body.png"]);
		let slime_blue_walk_texture = load_texture(&texture_creator, slime_blue_walk_path);

		let slime_blue_run_path = gfx_pc_path(&["slime", "blue", "run_body.png"]);
		let slime_blue_run_texture = load_texture(&texture_creator, slime_blue_run_path);

		let slime_undead_walk_path = gfx_pc_path(&["slime", "undead", "walk_body.png"]);
		let slime_undead_walk_texture = load_texture(&texture_creator, slime_undead_walk_path);

		let slime_blue_death_path: PathBuf = gfx_pc_path(&["slime", "blue", "death_body.png"]);
		let slime_blue_death_texture = load_texture(&texture_creator, slime_blue_death_path);

		let slime_undead_run_path = gfx_pc_path(&["slime", "undead", "run_body.png"]);
		let slime_undead_run_texture = load_texture(&texture_creator, slime_undead_run_path);

		let slime_lava_walk_path = gfx_pc_path(&["slime", "lava", "walk_body.png"]);
		let slime_lava_walk_texture = load_texture(&texture_creator, slime_lava_walk_path);

		let slime_lava_run_path = gfx_pc_path(&["slime", "lava", "run_body.png"]);
		let slime_lava_run_texture = load_texture(&texture_creator, slime_lava_run_path);

		let slime_lava_death_path: PathBuf = gfx_pc_path(&["slime", "lava", "death_body.png"]);
		let slime_lava_death_texture = texture_creator.load_texture(slime_lava_death_path).expect("failed to load slime_lava_death.png");

		let slime_undead_death_path: PathBuf = gfx_pc_path(&["slime", "undead", "death_body.png"]);
		let slime_undead_death_texture = texture_creator.load_texture(slime_undead_death_path).expect("failed to load slime_undead_death.png");

		let trigger_atlas_path: PathBuf = gfx_pc_path(&["icons.png"]);
		let trigger_texture = texture_creator.load_texture(trigger_atlas_path).expect("failed to load icons.png");

		let renderer = PcRenderer {
			video,
			canvas,
			event_pump,
			common: RenderCommon::new(),
			slime_blue_walk_texture,
			slime_blue_run_texture,
			slime_blue_death_texture,
			slime_undead_walk_texture,
			slime_undead_run_texture,
			slime_undead_death_texture,
			slime_lava_walk_texture,
			slime_lava_run_texture,
			slime_lava_death_texture,
			frame_index: 0,
			atlas_tile_width_pixels: 64,
			atlas_tile_height_pixels: 64,
			tile_texture: Some(tile_texture),
			bg_id: 0,
			bg_texture: None,
			bg_parallax_x: 0.35,
			bg_parallax_y: 0.15,
			render_scale: 4,
			texture_creator,
			font,
			trigger_texture,
		};

		return renderer;
	}

	fn screen_size(&self) -> (i32, i32) {
		let (width_pixels, height_pixels) = self.screen_size_pixels();
		return (width_pixels as i32, height_pixels as i32);
	}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::pc::poll(&mut self.event_pump);
	}

	fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
		self.canvas.clear();

		return;
	}

	fn draw_level(&mut self, game_state: &GameState, game_session: &GameSession) {
		self.draw_level_internal(game_state, game_session);
	}

	/*
	fn draw_death_entity(&mut self, state: &GameState) {
		self.draw_death_entity_internal(state);
		return;
	}
	*/

	fn draw_death_entity(
		&mut self,
		game_state: &GameState,
		game_session: &GameSession,
		entity_kind: EntityKind,
		pos: &Vec2,
		half_height: f32,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		death_timer: u16,
	) {
		self.draw_death_entity_internal(game_state, game_session, entity_kind, pos, half_height, camera_left, camera_top, scale, death_timer);
	}

	fn commit(&mut self) {
		self.canvas.present();
		self.frame_index += 1;
		return;
	}
}

fn leak_texture_creator(canvas: &sdl2::render::Canvas<sdl2::video::Window>) -> &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> {
	let creator_box = Box::new(canvas.texture_creator());
	let texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> = Box::leak(creator_box);
	return texture_creator;
}

fn gfx_pc_path(segments: &[&str]) -> PathBuf {
	let mut path = get_gfx_root().join("pc");
	for s in segments {
		path = path.join(s);
	}

	return path;
}

fn parse_background_id(background_id: u8) -> &'static str {
	match background_id {
		super::BACKGROUND_ID_LIBRARY_STONE => "bg_library_stone.png",
		super::BACKGROUND_PARALLAX_FOREST => "bg_parallax_forest.png",
		_ => panic!("Unknown id: {}", background_id),
	}
}

fn load_texture(texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext>, file_path: PathBuf) -> Texture<'static> {
	let texture = texture_creator.load_texture(&file_path).unwrap_or_else(|err| {
		panic!("missing texture file\npath: {}\nsdl error: {}", file_path.display(), err);
	});

	let t: Texture<'static> = unsafe { std::mem::transmute::<Texture<'_>, Texture<'static>>(texture) };
	return t;
}
