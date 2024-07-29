use std::{convert::Into, str::FromStr};

use self::{attribute::Attribute, skill::Skill};
use crate::template::SupernaturalTolerance;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, ParseError};

pub mod attribute;
pub mod skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraitCategory {
	Mental,
	Physical,
	Social,
}

#[derive(
	Debug, Clone, Hash, Copy, Serialize, Deserialize, EnumString, Display, AsRefStr, PartialEq, Eq,
)]
#[strum(ascii_case_insensitive)]
pub enum DerivedTrait {
	Speed,
	Defense,
	Initiative,
	Perception,
	Health,
	Willpower,

	Beats,

	Size,
}

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
		}
	}
}
