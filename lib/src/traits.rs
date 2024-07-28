use cofd_schema::prelude::{Attribute, Skill};
use serde::{Deserialize, Serialize};
// use cofd_util::VariantName;

// use super::Armor;
// use crate::splat::NameKey;

pub use cofd_schema::traits::DerivedTrait;
pub use cofd_schema::traits::{attribute, skill};
// #[derive(VariantName)]
// pub enum TraitCategory {
// 	Mental,
// 	Physical,
// 	Social,
// }

// impl TraitCategory {
// 	pub fn unskilled(&self) -> u16 {
// 		match self {
// 			TraitCategory::Mental => 3,
// 			TraitCategory::Physical => 1,
// 			TraitCategory::Social => 1,
// 		}
// 	}
// }

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
// pub enum AttributeType {
// 	Power,
// 	Finesse,
// 	Resistance,
// }
//
// pub enum AttributeCategory {
// 	Type(AttributeType),
// 	Trait(TraitCategory),
// }

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
