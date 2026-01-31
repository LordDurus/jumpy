mod ai;
mod assets;
mod common;
mod ecs;
mod engine_math;
pub mod game;
pub mod physics;
pub mod platform;
mod tile;

// crate-root re-exports so older `crate::X` imports keep working
pub use crate::game::{game_session::GameSession, game_state::GameState, level::Level, music::MusicId};

pub use crate::platform::render::backend::RenderBackend;

// if you have these and you want the older paths to keep working too:
pub use crate::game::book::{BookId, BookSlug};
