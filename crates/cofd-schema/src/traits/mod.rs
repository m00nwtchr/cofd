use std::{convert::Into, str::FromStr};

use strum::{AsRefStr, Display, EnumString, ParseError};

use self::skill::Skill;
use crate::{prelude::Attribute, template::SupernaturalTolerance};

pub mod attribute;
pub mod skill;

#[derive(Debug, Clone, Copy, facet::Facet, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TraitCategory {
	Mental,
	Physical,
	Social,
}

impl TraitCategory {
	#[must_use]
	pub fn unskilled(&self) -> u8 {
		match self {
			TraitCategory::Mental => 3,
			TraitCategory::Physical | TraitCategory::Social => 1,
		}
	}
}

#[derive(
	Debug,
	Clone,
	Hash,
	Copy,
	facet::Facet,
	EnumString,
	Display,
	AsRefStr,
	strum::VariantArray,
	PartialEq,
	Eq,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum DerivedTrait {
	Speed,
	Defense,
	Initiative,
	Perception,
	Health,
	Willpower,
}

#[derive(
	Debug, Clone, Copy, Hash, facet::Facet, PartialEq, Eq, derive_more::Display, derive_more::From,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[serde(untagged)]
#[repr(u8)]
pub enum Trait {
	Attribute(Attribute),
	Skill(Skill),

	DerivedTrait(DerivedTrait),

	Size,
	SupernaturalTolerance(SupernaturalTolerance),
}

impl FromStr for Trait {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Attribute::from_str(s)
			.map(Into::into)
			.or_else(|_| Skill::from_str(s).map(Into::into))
			.or_else(|_| SupernaturalTolerance::from_str(s).map(Into::into))
			.or_else(|_| DerivedTrait::from_str(s).map(Into::into))
	}
}

impl AsRef<str> for Trait {
	fn as_ref(&self) -> &str {
		match self {
			Trait::Attribute(attr) => attr.as_ref(),
			Trait::Skill(skill) => skill.as_ref(),
			Trait::DerivedTrait(dt) => dt.as_ref(),
			Trait::SupernaturalTolerance(st) => st.as_ref(),
			Trait::Size => "Size",
		}
	}
}
