#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod book;
pub mod dice_pool;
pub mod dot_range;
pub mod error;
pub mod item;
pub mod modifiers;
pub mod prerequisites;
pub mod template;
pub mod traits;

pub mod prelude {
	pub use super::{
		book::BookInfo,
		dot_range::DotRange,
		traits::{attribute::Attribute, skill::Skill},
	};
}

pub static DOT_CHAR: char = '•';
