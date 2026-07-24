pub use cofd_schema::traits::{DerivedTrait, TraitCategory, attribute, skill};
use cofd_schema::{
	prelude::{Attribute, Skill},
	template::SupernaturalTolerance,
};
use serde::{Deserialize, Serialize};

use crate::ability::Ability;
// #[derive(
// 	Debug,
// 	Clone,
// 	Copy,
// 	Hash,
// 	Serialize,
// 	Deserialize,
// 	PartialEq,
// 	Eq,
// 	derive_more::Display,
// 	derive_more::From,
// 	VariantName,
// )]
// #[serde(untagged)]
// pub enum Trait {
// 	#[expand]
// 	Attribute(Attribute),
// 	#[expand]
// 	Skill(Skill),
//
// 	#[expand]
// 	DerivedTrait(DerivedTrait),
// 	Beats,
// 	AlternateBeats,
//
// 	// Armor(Option<Armor>),
// 	Willpower,
// 	Power,
// 	Fuel,
// 	Integrity,
// }

#[derive(
	Debug,
	Clone,
	Hash,
	Serialize,
	Deserialize,
	PartialEq,
	Eq,
	derive_more::Display,
	derive_more::From,
)]
#[serde(untagged)]
pub enum Trait {
	Ability(Ability),
	Attribute(Attribute),
	Skill(Skill),

	DerivedTrait(DerivedTrait),

	Size,
	SupernaturalTolerance(SupernaturalTolerance),
}

pub trait NameKey {
	fn name_key(&self) -> String;
}
