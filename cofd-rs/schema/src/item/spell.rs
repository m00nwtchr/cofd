use std::str::FromStr;

use strum::EnumString;

use crate::{
	template::mage::{Arcanum, Practice},
	traits::skill::Skill,
};

#[derive(Debug, Clone, Copy, facet::Facet, EnumString, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[repr(u8)]
pub enum PrimaryFactor {
	Duration,
	Potency,
}

#[derive(Debug, Clone, Copy, facet::Facet)]
#[repr(u8)]
pub enum SpellFactor {
	Scale,
	// #[serde(untagged)]
	PrimaryFactor(PrimaryFactor),
}

impl From<PrimaryFactor> for SpellFactor {
	fn from(value: PrimaryFactor) -> Self {
		Self::PrimaryFactor(value)
	}
}

impl FromStr for SpellFactor {
	type Err = strum::ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		PrimaryFactor::from_str(s).map(Into::into).or_else(|err| {
			if s.eq_ignore_ascii_case("Scale") {
				Ok(SpellFactor::Scale)
			} else {
				Err(err)
			}
		})
	}
}

#[derive(Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ReachEffect {
	cost: u8,
	description: Vec<String>,
}

#[derive(Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[repr(u8)]
pub enum ArcanumEffectKind {
	Add,
	Substitute,

	Or(Vec<ArcanumEffectKind>),
}

#[derive(Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ArcanumEffect {
	arcanum: Arcanum,
	rating: u8,
	description: Vec<String>,
	kind: ArcanumEffectKind,
}

#[derive(Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
// #[serde(rename_all = "camelCase")]
pub struct Spell {
	arcana: Vec<(Arcanum, u8)>,
	practice: Practice,
	primary_factor: PrimaryFactor,
	cost: String,
	suggested_rote_skills: [Skill; 3],
	reaches: Vec<ReachEffect>,
	arcana_effects: Vec<ArcanumEffect>,
}
