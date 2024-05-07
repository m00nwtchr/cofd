mod mage;

use crate::splat::mage::Rote;
use cofd_schema::prelude::Attribute;
use cofd_schema::splat::mage::{Order, Path};

pub enum Splat {
	// #[strum(to_string = "Mortal", serialize = "Human")]
	Mortal,
	// #[strum(to_string = "Mage", serialize = "Awakened")]
	Mage {
		path: Path,
		order: Order,
		attribute_bonus: Attribute,
		obsessions: Vec<String>,
		rotes: Vec<Rote>,
	},
	// #[strum(to_string = "Vampire", serialize = "Kindred")]
	Vampire,
	Werewolf,
	Promethean,
	Changeling,
	Hunter,
	Bound,
	Mummy,
	Demon,
	Beast,
	Deviant,
}
