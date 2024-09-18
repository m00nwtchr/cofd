use std::cmp::{max, min};

pub use cofd_schema::dice_pool::DicePool;

use crate::{character::Character, traits::Trait};

pub trait DicePoolExt {
	fn value(&self, character: &Character) -> i8;
}

impl DicePoolExt for DicePool {
	fn value(&self, character: &Character) -> i8 {
		match self {
			Self::Mod(val) => *val,

			Self::Trait(trait_) => match trait_ {
				cofd_schema::traits::Trait::DerivedTrait(dt) => {
					character.get_trait(&Trait::DerivedTrait(*dt)) as i8
				}
				cofd_schema::traits::Trait::Attribute(attr) => {
					character.attributes().get(*attr).value() as i8
				}

				cofd_schema::traits::Trait::Skill(skill) => character.skills().get(*skill).value() as i8,
				cofd_schema::traits::Trait::SupernaturalTolerance(_) => {
					character.power.value() as i8
				}
			},
			Self::Max(p1, p2) => p1.value(character).max(p2.value(character)),
			Self::Min(p1, p2) => p1.value(character).min(p2.value(character)),

			Self::Add(vec) => vec.iter().fold(0, |acc, e| acc + e.value(character)),
			Self::Sub(p1, p2) => p1.value(character) - p2.value(character),

			_ => 0,
		}
	}
}
