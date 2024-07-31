use cofd_util::VariantName;
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantArray};

use crate::traits::skill::Skill;

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	PartialEq,
	Eq,
	VariantArray,
	Hash,
	VariantName,
)]
pub enum Arcanum {
	Death,
	Fate,
	Forces,
	Life,
	Matter,
	Mind,
	Prime,
	Space,
	Spirit,
	Time,
}

impl Arcanum {
	#[must_use] pub fn all() -> &'static [Self] {
		Self::VARIANTS
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq)]
pub enum Practice {
	Compelling,
	Knowing,
	Unveiling,

	Ruling,
	Shielding,
	Veiling,

	Fraying,
	Perfecting,
	Weaving,

	Patterning,
	Unraveling,

	Making,
	Unmaking,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Path {
	#[default]
	Acanthus,
	Mastigos,
	Moros,
	Obrimos,
	Thyrsus,
	_Custom {
		name: String,
		ruling_arcana: [Arcanum; 2],
		inferior_arcanum: Arcanum,
	},
}

impl Path {
	fn get_ruling_arcana(&self) -> &[Arcanum; 2] {
		match self {
			Path::Acanthus => &[Arcanum::Time, Arcanum::Fate],
			Path::Mastigos => &[Arcanum::Space, Arcanum::Mind],
			Path::Moros => &[Arcanum::Matter, Arcanum::Death],
			Path::Obrimos => &[Arcanum::Forces, Arcanum::Prime],
			Path::Thyrsus => &[Arcanum::Life, Arcanum::Spirit],
			Path::_Custom { ruling_arcana, .. } => ruling_arcana,
		}
	}
	fn get_inferior_arcanum(&self) -> &Arcanum {
		match self {
			Path::Acanthus => &Arcanum::Forces,
			Path::Mastigos => &Arcanum::Matter,
			Path::Moros => &Arcanum::Spirit,
			Path::Obrimos => &Arcanum::Death,
			Path::Thyrsus => &Arcanum::Mind,
			Path::_Custom {
				inferior_arcanum, ..
			} => inferior_arcanum,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Order {
	AdamantineArrow,
	GuardiansOfTheVeil,
	Mysterium,
	SilverLadder,
	FreeCouncil,
	// #[expand]
	SeersOfTheThrone(Option<Ministry>),
	_Custom {
		name: String,
		rote_skills: [Skill; 3],
	},
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Ministry {
	Hegemony,
	Panopticon,
	Paternoster,
	Praetorian,
	_Custom {
		name: String,
		rote_skills: [Skill; 3],
	},
}

impl Order {
	#[must_use]
	pub fn rote_skills(&self) -> &[Skill; 3] {
		match self {
			Order::AdamantineArrow => &[Skill::Athletics, Skill::Intimidation, Skill::Medicine],
			Order::GuardiansOfTheVeil => &[Skill::Investigation, Skill::Stealth, Skill::Subterfuge],
			Order::Mysterium => &[Skill::Investigation, Skill::Occult, Skill::Survival],
			Order::SilverLadder => &[Skill::Expression, Skill::Persuasion, Skill::Subterfuge],
			Order::FreeCouncil => &[Skill::Crafts, Skill::Persuasion, Skill::Science],
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => match ministry {
					Ministry::Hegemony => &[Skill::Politics, Skill::Persuasion, Skill::Empathy],
					Ministry::Panopticon => {
						&[Skill::Investigation, Skill::Stealth, Skill::Subterfuge]
					}
					Ministry::Paternoster => &[Skill::Academics, Skill::Occult, Skill::Expression],
					Ministry::Praetorian => {
						&[Skill::Athletics, Skill::Larceny, Skill::Intimidation]
					}
					Ministry::_Custom { rote_skills, .. } => rote_skills,
				},
				None => &[Skill::Investigation, Skill::Occult, Skill::Persuasion],
			},
			Order::_Custom { rote_skills, .. } => rote_skills,
		}
	}
}

impl From<Ministry> for Order {
	fn from(ministry: Ministry) -> Self {
		Order::SeersOfTheThrone(Some(ministry))
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Legacy {
	name: String,
	ruling_arcanum: Arcanum,
}
