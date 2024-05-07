use cofd_schema::prelude::Skill;
use cofd_schema::splat::mage::Arcanum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rote {
	pub arcanum: Arcanum,
	pub level: u16,
	pub spell: String,
	pub creator: String,
	pub skill: Skill,
}
