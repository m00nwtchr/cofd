use serde::{Deserialize, Serialize};

use crate::util::is_empty;

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct CharacterInfo {
	#[serde(skip_serializing_if = "is_empty")]
	pub name: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub player: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub virtue_anchor: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub vice_anchor: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub faction: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub group_name: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub concept: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub chronicle: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub age: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub date_of_birth: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub hair: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub eyes: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub race: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub nationality: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub height: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub weight: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub sex: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub other: String,
}

impl CharacterInfo {
	pub fn get(&self, info: InfoTrait) -> &String {
		match info {
			InfoTrait::Name => &self.name,
			InfoTrait::Age => &self.age,
			InfoTrait::Player => &self.player,
			InfoTrait::Concept => &self.concept,
			InfoTrait::Chronicle => &self.chronicle,
			InfoTrait::DateOfBirth => &self.date_of_birth,
			InfoTrait::Hair => &self.hair,
			InfoTrait::Eyes => &self.eyes,
			InfoTrait::Race => &self.race,
			InfoTrait::Nationality => &self.nationality,
			InfoTrait::Height => &self.height,
			InfoTrait::Weight => &self.weight,
			InfoTrait::Sex => &self.sex,
			InfoTrait::VirtueAnchor => &self.virtue_anchor,
			InfoTrait::ViceAnchor => &self.vice_anchor,
			InfoTrait::Faction => &self.faction,
			InfoTrait::GroupName => &self.group_name,
		}
	}

	pub fn get_mut(&mut self, info: InfoTrait) -> &mut String {
		match info {
			InfoTrait::Name => &mut self.name,
			InfoTrait::Age => &mut self.age,
			InfoTrait::Player => &mut self.player,
			InfoTrait::Concept => &mut self.concept,
			InfoTrait::Chronicle => &mut self.chronicle,
			InfoTrait::DateOfBirth => &mut self.date_of_birth,
			InfoTrait::Hair => &mut self.hair,
			InfoTrait::Eyes => &mut self.eyes,
			InfoTrait::Race => &mut self.race,
			InfoTrait::Nationality => &mut self.nationality,
			InfoTrait::Height => &mut self.height,
			InfoTrait::Weight => &mut self.weight,
			InfoTrait::Sex => &mut self.sex,
			InfoTrait::VirtueAnchor => &mut self.virtue_anchor,
			InfoTrait::ViceAnchor => &mut self.vice_anchor,
			InfoTrait::Faction => &mut self.faction,
			InfoTrait::GroupName => &mut self.group_name,
		}
	}
}

#[derive(Debug, Clone, Copy, VariantName)]
pub enum InfoTrait {
	Name,
	Age,
	Player,
	VirtueAnchor,
	ViceAnchor,
	Concept,
	Chronicle,

	Faction,
	GroupName,

	DateOfBirth,
	Hair,
	Eyes,
	Race,
	Nationality,
	Height,
	Weight,
	Sex,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ArmorStruct {
	pub general: u16,
	pub ballistic: u16,
}

#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Armor {
	General,
	Ballistic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Weapon {
	pub name: String,
	pub dice_pool: String,
	pub damage: String,
	pub range: String,
	pub initiative: i16,
	pub size: u16,
}
