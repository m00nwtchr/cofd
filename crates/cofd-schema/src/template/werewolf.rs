use crate::prelude::Skill;

#[derive(Debug, Clone, Copy, Hash, facet::Facet, PartialEq, Eq, strum::EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum Renown {
	Purity,
	Glory,
	Honor,
	Wisdom,
	Cunning,
}

#[derive(Clone, Copy, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum HuntersAspect {
	Monstrous,
	Isolating,
	Blissful,
	Mystic,
	Dominant,

	Fanatical,
	Frenzied,
	Agonized,
	Insidious,
	Implacable,
	Primal,
}

#[derive(Debug, Clone, Copy, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[repr(u8)]
pub enum Auspice {
	Cahalith,
	Elodoth,
	Irraka,
	Ithaeur,
	Rahu,
}

impl Auspice {
	#[must_use]
	pub fn skills(&self) -> [Skill; 3] {
		match self {
			Auspice::Cahalith => [Skill::Crafts, Skill::Expression, Skill::Persuasion],
			Auspice::Elodoth => [Skill::Empathy, Skill::Investigation, Skill::Politics],
			Auspice::Irraka => [Skill::Larceny, Skill::Stealth, Skill::Subterfuge],
			Auspice::Ithaeur => [Skill::AnimalKen, Skill::Medicine, Skill::Occult],
			Auspice::Rahu => [Skill::Brawl, Skill::Intimidation, Skill::Survival],
		}
	}

	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			Auspice::Cahalith => Renown::Glory,
			Auspice::Elodoth => Renown::Honor,
			Auspice::Irraka => Renown::Cunning,
			Auspice::Ithaeur => Renown::Wisdom,
			Auspice::Rahu => Renown::Purity,
		}
	}

	#[must_use]
	pub fn hunters_aspect(&self) -> HuntersAspect {
		match self {
			Auspice::Cahalith => HuntersAspect::Monstrous,
			Auspice::Elodoth => HuntersAspect::Isolating,
			Auspice::Irraka => HuntersAspect::Blissful,
			Auspice::Ithaeur => HuntersAspect::Mystic,
			Auspice::Rahu => HuntersAspect::Dominant,
		}
	}
}

#[derive(Clone, Copy, Debug, facet::Facet, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
pub enum Form {
	#[default]
	Hishu,
	Dalu,
	Gauru,
	Urshul,
	Urhan,
}

#[derive(Clone, Copy)]
pub enum KuruthTriggerKind {
	Passive,
	Common,
	Specific,
}

#[derive(Clone, Copy, facet::Facet, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ForsakenTribe {
	BloodTalons,
	BoneShadows,
	HuntersInDarkness,
	IronMasters,
	StormLords,
}

impl ForsakenTribe {
	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			Self::BloodTalons => Renown::Glory,
			Self::BoneShadows => Renown::Wisdom,
			Self::HuntersInDarkness => Renown::Purity,
			Self::IronMasters => Renown::Cunning,
			Self::StormLords => Renown::Honor,
		}
	}
}

#[derive(Clone, Copy, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PureTribe {
	FireTouched,
	IvoryClaws,
	PredatorKings,
}

impl PureTribe {
	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			PureTribe::FireTouched => Renown::Wisdom,
			PureTribe::IvoryClaws => Renown::Purity,
			PureTribe::PredatorKings => Renown::Glory,
		}
	}

	#[must_use]
	pub fn secondary_renown(&self) -> [Renown; 2] {
		match self {
			Self::FireTouched => [Renown::Cunning, Renown::Glory],
			Self::IvoryClaws => [Renown::Glory, Renown::Honor],
			Self::PredatorKings => [Renown::Purity, Renown::Wisdom],
		}
	}

	#[must_use]
	pub fn skills(&self) -> [Skill; 3] {
		match self {
			Self::FireTouched => [Skill::Expression, Skill::Occult, Skill::Subterfuge],
			Self::IvoryClaws => [Skill::Intimidation, Skill::Persuasion, Skill::Politics],
			Self::PredatorKings => [Skill::AnimalKen, Skill::Brawl, Skill::Crafts],
		}
	}

	#[must_use]
	pub fn hunters_aspects(&self) -> [HuntersAspect; 2] {
		match self {
			Self::FireTouched => [HuntersAspect::Fanatical, HuntersAspect::Frenzied],
			Self::IvoryClaws => [HuntersAspect::Agonized, HuntersAspect::Insidious],
			Self::PredatorKings => [HuntersAspect::Implacable, HuntersAspect::Primal],
		}
	}
}

#[derive(Clone, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lodge {
	pub name: String,
}
