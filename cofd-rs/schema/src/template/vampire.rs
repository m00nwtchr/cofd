use crate::prelude::Attribute;

#[derive(Clone, facet::Facet, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Clan {
	#[default]
	Daeva,
	Gangrel,
	Mekhet,
	Nosferatu,
	Ventrue,
}

impl Clan {
	#[must_use]
	pub fn favored_attributes(&self) -> [Attribute; 2] {
		match self {
			Clan::Daeva => [Attribute::Dexterity, Attribute::Manipulation],
			Clan::Gangrel => [Attribute::Composure, Attribute::Stamina],
			Clan::Mekhet => [Attribute::Intelligence, Attribute::Wits],
			Clan::Nosferatu => [Attribute::Composure, Attribute::Strength],
			Clan::Ventrue => [Attribute::Presence, Attribute::Resolve],
		}
	}
}
