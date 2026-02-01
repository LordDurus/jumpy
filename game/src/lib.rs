#![cfg_attr(feature = "gba", no_std)]

#[cfg(feature = "gba")]
extern crate alloc;

mod ai;
mod common;
mod ecs;
mod engine_math;
pub mod physics;
pub mod platform;
pub mod runtime;
mod tile;

pub use crate::{
	platform::render::backend::RenderBackend,
	runtime::{
		book::{BookId, BookSlug},
		level::Level,
		music::MusicId,
		session::Session,
		state::State,
	},
};
