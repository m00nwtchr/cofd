use cofd_schema::prelude::Skill;
use serde::{Deserialize, Serialize};

use crate::is_zero;

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Skills {
	#[serde(skip_serializing_if = "is_zero")]
	pub academics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub computer: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub crafts: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub investigation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub medicine: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub occult: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub politics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub science: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub athletics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub brawl: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub drive: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub firearms: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub larceny: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub stealth: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub survival: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub weaponry: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub animal_ken: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub empathy: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub expression: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub intimidation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub persuasion: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub socialize: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub streetwise: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub subterfuge: u8,
}

impl Skills {
	pub fn get(&self, skill: Skill) -> u8 {
		match skill {
			Skill::Academics => self.academics,
			Skill::Computer => self.computer,
			Skill::Crafts => self.crafts,
			Skill::Investigation => self.investigation,
			Skill::Medicine => self.medicine,
			Skill::Occult => self.occult,
			Skill::Politics => self.politics,
			Skill::Science => self.science,
			//
			Skill::Athletics => self.athletics,
			Skill::Brawl => self.brawl,
			Skill::Drive => self.drive,
			Skill::Firearms => self.firearms,
			Skill::Larceny => self.larceny,
			Skill::Stealth => self.stealth,
			Skill::Survival => self.survival,
			Skill::Weaponry => self.weaponry,
			//
			Skill::AnimalKen => self.animal_ken,
			Skill::Empathy => self.empathy,
			Skill::Expression => self.expression,
			Skill::Intimidation => self.intimidation,
			Skill::Persuasion => self.persuasion,
			Skill::Socialize => self.socialize,
			Skill::Streetwise => self.streetwise,
			Skill::Subterfuge => self.subterfuge,
		}
	}

	pub fn get_mut(&mut self, skill: Skill) -> &mut u8 {
		match skill {
			Skill::Academics => &mut self.academics,
			Skill::Computer => &mut self.computer,
			Skill::Crafts => &mut self.crafts,
			Skill::Investigation => &mut self.investigation,
			Skill::Medicine => &mut self.medicine,
			Skill::Occult => &mut self.occult,
			Skill::Politics => &mut self.politics,
			Skill::Science => &mut self.science,
			//
			Skill::Athletics => &mut self.athletics,
			Skill::Brawl => &mut self.brawl,
			Skill::Drive => &mut self.drive,
			Skill::Firearms => &mut self.firearms,
			Skill::Larceny => &mut self.larceny,
			Skill::Stealth => &mut self.stealth,
			Skill::Survival => &mut self.survival,
			Skill::Weaponry => &mut self.weaponry,
			//
			Skill::AnimalKen => &mut self.animal_ken,
			Skill::Empathy => &mut self.empathy,
			Skill::Expression => &mut self.expression,
			Skill::Intimidation => &mut self.intimidation,
			Skill::Persuasion => &mut self.persuasion,
			Skill::Socialize => &mut self.socialize,
			Skill::Streetwise => &mut self.streetwise,
			Skill::Subterfuge => &mut self.subterfuge,
		}
	}
}
