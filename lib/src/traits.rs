use cofd_schema::prelude::{Attribute, Skill};
pub use cofd_schema::traits::{attribute, skill, DerivedTrait, TraitCategory};
use cofd_util::VariantName;
use serde::{Deserialize, Serialize};

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
	VariantName,
)]
#[serde(untagged)]
pub enum Trait {
	#[expand]
	Attribute(Attribute),
	#[expand]
	Skill(Skill),

	#[expand]
	DerivedTrait(DerivedTrait),
	Beats,
	AlternateBeats,

	// Armor(Option<Armor>),
	Willpower,
	Power,
	Fuel,
	Integrity,
}
