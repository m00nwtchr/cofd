//! Raw WebAssembly bridge between the `cofd` rules engine and Foundry VTT.
//!
//! The crate is compiled as a `cdylib` for `wasm32-unknown-unknown`. Foundry
//! loads the resulting `cofd_foundry.wasm` directly — no wasm-bindgen glue, no
//! external toolchain — and talks to it through the small C ABI exported at the
//! bottom of this file.

use std::mem::size_of;

use cofd::{
	character::Character,
	schema::template::Template,
	splat::{SplatCharacter, mortal::Mortal},
	systema::prelude::Actor,
};
use facet::Facet;

/// Version of the packaged module; kept in sync with `module/module.json`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Number of character templates described by the `cofd` schema.
///
/// Counted through facet's reflection metadata, so the value always tracks
/// `cofd_schema::template::Template`.
#[must_use]
pub fn template_count() -> u32 {
	match &Template::SHAPE.ty {
		facet::Type::User(facet::UserType::Enum(enumeration)) => enumeration.variants.len() as u32,
		_ => 0,
	}
}

/// Builds a mortal character with the `cofd` engine.
#[must_use]
pub fn mortal_character() -> SplatCharacter {
	SplatCharacter::new(Template::Mortal)
}

/// Size, in bytes, of the in-memory layout of a mortal character.
#[must_use]
pub fn mortal_character_size() -> u32 {
	let character = mortal_character();
	assert!(matches!(character, SplatCharacter::Mortal(_)));
	size_of::<Character<Mortal>>() as u32
}

/// Pointer to the UTF-8 encoded version string.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn cofd_foundry_version_ptr() -> *const u8 {
	VERSION.as_ptr()
}

/// Length, in bytes, of the UTF-8 encoded version string.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn cofd_foundry_version_len() -> u32 {
	VERSION.len() as u32
}

/// Number of character templates known to the `cofd` schema.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn cofd_foundry_template_count() -> u32 {
	template_count()
}

/// Size, in bytes, of the in-memory layout of a mortal character.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn cofd_foundry_mortal_character_size() -> u32 {
	mortal_character_size()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn version_is_the_package_version() {
		assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
		assert!(!VERSION.is_empty());
	}

	#[test]
	fn templates_are_discovered_from_the_schema() {
		assert!(template_count() > 0);
	}

	#[test]
	fn mortal_character_can_be_built() {
		let character = mortal_character();
		assert!(matches!(character, SplatCharacter::Mortal(_)));
		assert!(mortal_character_size() > 0);
	}
}
