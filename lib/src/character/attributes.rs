use cofd_schema::prelude::Attribute;
use serde::{Deserialize, Serialize};

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_one(num: &u8) -> bool {
	num.eq(&1)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Attributes {
	#[serde(skip_serializing_if = "is_one")]
	pub intelligence: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub wits: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub resolve: u8,

	#[serde(skip_serializing_if = "is_one")]
	pub strength: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub dexterity: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub stamina: u8,

	#[serde(skip_serializing_if = "is_one")]
	pub presence: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub manipulation: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub composure: u8,
}

impl Attributes {
	pub fn get(&self, attr: &Attribute) -> &u8 {
		match attr {
			Attribute::Intelligence => &self.intelligence,
			Attribute::Wits => &self.wits,
			Attribute::Resolve => &self.resolve,
			//
			Attribute::Strength => &self.strength,
			Attribute::Dexterity => &self.dexterity,
			Attribute::Stamina => &self.stamina,
			//
			Attribute::Presence => &self.presence,
			Attribute::Manipulation => &self.manipulation,
			Attribute::Composure => &self.composure,
		}
	}

	pub fn get_mut(&mut self, attr: &Attribute) -> &mut u8 {
		match attr {
			Attribute::Intelligence => &mut self.intelligence,
			Attribute::Wits => &mut self.wits,
			Attribute::Resolve => &mut self.resolve,
			//
			Attribute::Strength => &mut self.strength,
			Attribute::Dexterity => &mut self.dexterity,
			Attribute::Stamina => &mut self.stamina,
			//
			Attribute::Presence => &mut self.presence,
			Attribute::Manipulation => &mut self.manipulation,
			Attribute::Composure => &mut self.composure,
		}
	}
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			intelligence: 1,
			wits: 1,
			resolve: 1,
			strength: 1,
			dexterity: 1,
			stamina: 1,
			presence: 1,
			manipulation: 1,
			composure: 1,
		}
	}
}