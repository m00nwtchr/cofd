use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::prelude::{MentalSkill, PhysicalSkill, SocialSkill};
use crate::traits::skill::Skill;
use {
	MentalSkill::{Academics, Crafts, Investigation, Medicine, Occult, Politics, Science},
	PhysicalSkill::{Athletics, Larceny, Stealth, Survival},
	SocialSkill::{Empathy, Expression, Intimidation, Persuasion, Subterfuge},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq)]
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
		// (String, [Arcanum; 2], Arcanum),
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
	pub fn get_rote_skills(&self) -> &[Skill; 3] {
		match self {
			Order::AdamantineArrow => &[
				Skill::Physical(Athletics),
				Skill::Social(Intimidation),
				Skill::Mental(Medicine),
			],
			Order::GuardiansOfTheVeil => &[
				Skill::Mental(Investigation),
				Skill::Physical(Stealth),
				Skill::Social(Subterfuge),
			],
			Order::Mysterium => &[
				Skill::Mental(Investigation),
				Skill::Mental(Occult),
				Skill::Physical(Survival),
			],
			Order::SilverLadder => &[
				Skill::Social(Expression),
				Skill::Social(Persuasion),
				Skill::Social(Subterfuge),
			],
			Order::FreeCouncil => &[
				Skill::Mental(Crafts),
				Skill::Social(Persuasion),
				Skill::Mental(Science),
			],
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => match ministry {
					Ministry::Hegemony => &[
						Skill::Mental(Politics),
						Skill::Social(Persuasion),
						Skill::Social(Empathy),
					],
					Ministry::Panopticon => &[
						Skill::Mental(Investigation),
						Skill::Physical(Stealth),
						Skill::Social(Subterfuge),
					],
					Ministry::Paternoster => &[
						Skill::Mental(Academics),
						Skill::Mental(Occult),
						Skill::Social(Expression),
					],
					Ministry::Praetorian => &[
						Skill::Physical(Athletics),
						Skill::Physical(Larceny),
						Skill::Social(Intimidation),
					],
					Ministry::_Custom { rote_skills, .. } => rote_skills,
				},
				None => &[
					Skill::Mental(Investigation),
					Skill::Mental(Occult),
					Skill::Social(Persuasion),
				],
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
