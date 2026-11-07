//! WebAssembly bridge between the `cofd` rules engine and Foundry VTT.
//!
//! The crate is compiled as a `cdylib` for `wasm32-unknown-unknown`. The
//! `wasm-bindgen` CLI generates the JavaScript loader and bindings consumed by
//! the Foundry system.

use cofd::{
	character::Character,
	schema::{prelude::Attribute, template::Template, traits::DerivedTrait},
	splat::{SplatCharacter, mortal::Mortal},
	systema::prelude::Actor,
	traits::Trait,
};
use facet::Facet;
use wasm_bindgen::prelude::*;

/// Version of the packaged system; kept in sync with `system/system.json`.
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
	SplatCharacter::Mortal(Character::<Mortal>::new(Mortal))
}

/// Version of the embedded rules engine.
#[wasm_bindgen]
pub fn version() -> String {
	VERSION.to_owned()
}

/// Number of character templates known to the `cofd` schema.
#[wasm_bindgen(js_name = templateCount)]
pub fn template_count_export() -> u32 {
	template_count()
}

fn mortal_character_with_attributes(
	stamina: u8,
	size: u8,
	resolve: u8,
	composure: u8,
) -> Character<Mortal> {
	let SplatCharacter::Mortal(mut character) = mortal_character() else {
		unreachable!("mortal_character must return a mortal character");
	};

	let attributes = character.attributes_mut();
	attributes.set_raw_value(&Trait::Attribute(Attribute::Stamina), stamina);
	attributes.set_raw_value(&Trait::Size, size);
	attributes.set_raw_value(&Trait::Attribute(Attribute::Resolve), resolve);
	attributes.set_raw_value(&Trait::Attribute(Attribute::Composure), composure);
	character
}

/// Calculate a mortal character's derived Health with the CofD engine.
#[wasm_bindgen(js_name = mortalHealth)]
pub fn mortal_health(stamina: u8, size: u8) -> u8 {
	mortal_character_with_attributes(stamina, size, 1, 1)
		.attributes()
		.value(&Trait::DerivedTrait(DerivedTrait::Health))
		.unwrap_or_default()
}

/// Calculate a mortal character's derived Willpower with the CofD engine.
#[wasm_bindgen(js_name = mortalWillpower)]
pub fn mortal_willpower(resolve: u8, composure: u8) -> u8 {
	mortal_character_with_attributes(1, 5, resolve, composure)
		.attributes()
		.value(&Trait::DerivedTrait(DerivedTrait::Willpower))
		.unwrap_or_default()
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
	}

	#[test]
	fn derived_values_use_the_cofd_engine() {
		assert_eq!(mortal_health(1, 5), 6);
		assert_eq!(mortal_health(3, 5), 8);
		assert_eq!(mortal_willpower(1, 1), 2);
		assert_eq!(mortal_willpower(3, 4), 7);
	}
}
