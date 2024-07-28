use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

#[derive(
	Clone,
	Copy,
	Debug,
	Hash,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	strum::Display,
)]
#[strum(ascii_case_insensitive)]
pub enum Skill {
	Academics,
	Computer,
	Crafts,
	Investigation,
	Medicine,
	Occult,
	Politics,
	Science,

	Athletics,
	Brawl,
	Drive,
	Firearms,
	Larceny,
	Stealth,
	Survival,
	Weaponry,

	AnimalKen,
	Empathy,
	Expression,
	Intimidation,
	Persuasion,
	Socialize,
	Streetwise,
	Subterfuge,
}

impl Skill {
	// fn mental() -> [Skill; 8] {
	// 	[
	// 		Self::Academics,
	// 		Self::Computer,
	// 		Self::Crafts,
	// 		Self::Investigation,
	// 		Self::Medicine,
	// 		Self::Occult,
	// 		Self::Politics,
	// 		Self::Science,
	// 	]
	// }
	//
	// fn physical() -> [Skill; 8] {
	// 	[
	// 		Self::Athletics,
	// 		Self::Brawl,
	// 		Self::Drive,
	// 		Self::Firearms,
	// 		Self::Larceny,
	// 		Self::Stealth,
	// 		Self::Survival,
	// 		Self::Weaponry,
	// 	]
	// }
	//
	// fn social() -> [Skill; 8] {
	// 	[
	// 		Self::AnimalKen,
	// 		Self::Empathy,
	// 		Self::Expression,
	// 		Self::Intimidation,
	// 		Self::Persuasion,
	// 		Self::Socialize,
	// 		Self::Streetwise,
	// 		Self::Subterfuge,
	// 	]
	// }

	// pub fn get(cat: &TraitCategory) -> [Skill; 8] {
	// 	match cat {
	// 		TraitCategory::Mental => Self::mental(),
	// 		TraitCategory::Physical => Self::physical(),
	// 		TraitCategory::Social => Self::social(),
	// 	}
	// }
}
