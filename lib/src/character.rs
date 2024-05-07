use serde::{Deserialize, Serialize};
use crate::wound::WoundTracker;

fn is_empty(str: &String) -> bool {
	str.is_empty()
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Character {
	// pub splat: Splat,

	pub info: CharacterInfo,

	// #[serde(rename = "attributes")]
	// _attributes: Attributes,
	// skills: Skills,
	// pub specialties: HashMap<Skill, Vec<String>>,

	health: WoundTracker,

	pub willpower: u16,
	pub power: u16,
	pub fuel: u16,
	pub integrity: u16,

	// #[serde(skip_serializing_if = "is_empty_vec")]
	// pub touchstones: Vec<String>,

	// #[serde(skip)]
	// pub abilities: HashMap<Ability, u16>,
	// // #[serde(skip)]
	// pub merits: Vec<(Merit, u16)>,

	// pub weapons: Vec<Weapon>,

	// #[serde(skip_serializing_if = "is_five")]
	// pub base_size: u16,
	// base_armor: ArmorStruct,
	pub beats: u16,
	// #[serde(skip_serializing_if = "is_zero")]
	// pub alternate_beats: u16,

	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,

	// #[serde(skip)]
	// modifiers: Modifiers,
}