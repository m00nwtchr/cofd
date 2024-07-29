use std::cmp::{max, min};

use crate::character::Character;
use crate::traits::Trait;
pub use cofd_schema::dice_pool::DicePool;

pub trait DicePoolExt {
	fn value(&self, character: &Character) -> i8;
}

impl DicePoolExt for DicePool {
	fn value(&self, character: &Character) -> i8 {
		match self {
			Self::Mod(val) => *val,

			Self::Trait(trait_) => match trait_ {
				cofd_schema::traits::Trait::DerivedTrait(dt) => {
					character.get_trait(&Trait::DerivedTrait(dt.clone())) as i8
				}
				cofd_schema::traits::Trait::Attribute(attr) => {
					character.get_trait(&Trait::Attribute(attr.clone())) as i8
				}

				cofd_schema::traits::Trait::Skill(skill) => {
					character.get_trait(&Trait::Skill(skill.clone())) as i8
				}
				cofd_schema::traits::Trait::SupernaturalTolerance(_) => character.power as i8,
			},
			Self::Max(p1, p2) => max(p1.value(character), p2.value(character)),
			Self::Min(p1, p2) => min(p1.value(character), p2.value(character)),

			Self::Add(vec) => vec.iter().fold(0, |acc, e| acc + e.value(character)),
			Self::Sub(p1, p2) => p1.value(character) - p2.value(character),

			_ => 0,
		}
	}
}
