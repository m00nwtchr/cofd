use cofd_schema::prelude::{Attribute, Skill};
use serde::{Deserialize, Serialize};

pub use cofd_schema::traits::{
	DerivedTrait, TraitCategory, {attribute, skill},
};

#[derive(
	Debug,
	Clone,
	Copy,
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
	Attribute(Attribute),
	Skill(Skill),

	DerivedTrait(DerivedTrait),
	AlternateBeats,

	// Armor(Option<Armor>),
	Willpower,
	Power,
	Fuel,
	Integrity,
}
