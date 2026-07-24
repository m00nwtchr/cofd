use strum::VariantArray;

use crate::prelude::Attribute;

#[derive(Clone, Copy, facet::Facet, Debug, PartialEq, Eq, VariantArray)]
#[repr(u8)]
pub enum Regalia {
	Crown,
	Jewels,
	Mirror,
	Shield,
	Steed,
	Sword,
}

#[derive(Clone, Copy, facet::Facet, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Seeming {
	#[default]
	Beast,
	Darkling,
	Elemental,
	Fairest,
	Ogre,
	Wizened,
}

impl Seeming {
	#[must_use]
	pub fn favored_regalia(&self) -> Regalia {
		match self {
			Seeming::Beast => Regalia::Steed,
			Seeming::Darkling => Regalia::Mirror,
			Seeming::Elemental => Regalia::Sword,
			Seeming::Fairest => Regalia::Crown,
			Seeming::Ogre => Regalia::Shield,
			Seeming::Wizened => Regalia::Jewels,
		}
	}

	#[must_use]
	pub fn favored_attributes(&self) -> [Attribute; 3] {
		// Attribute::get(AttributeCategory::Type(match self {
		// 	Seeming::Beast => AttributeType::Resistance,
		// 	Seeming::Darkling => AttributeType::Finesse,
		// 	Seeming::Elemental => AttributeType::Resistance,
		// 	Seeming::Fairest => AttributeType::Power,
		// 	Seeming::Ogre => AttributeType::Power,
		// 	Seeming::Wizened => AttributeType::Finesse,
		// 	Seeming::_Custom(.., _type) => _type.clone(),
		// }))
		// TODO
		[
			Attribute::Intelligence,
			Attribute::Intelligence,
			Attribute::Intelligence,
		]
	}
}
