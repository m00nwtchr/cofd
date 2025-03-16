use cofd_schema::{template::werewolf::Renown, traits::Trait};
use cofd_util::VariantName;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use systema::prelude::AttributeModifier;

use super::{Merit, geist::Haunt, mage::Arcanum, vampire::Discipline, werewolf::MoonGift};
use crate::COp;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, VariantName)]
#[enum_dispatch(AbilityTrait)]
pub enum Ability {
	#[expand]
	Merit(Merit),
	#[expand]
	Discipline(Discipline),
	#[expand]
	Renown(Renown),
	#[expand]
	MoonGift(MoonGift),
	#[expand]
	Arcanum(Arcanum),
	#[expand]
	Haunt(Haunt),
}

impl Ability {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Ability::Merit(Merit::Custom(name))
			| Ability::Discipline(Discipline::Custom(name))
			| Ability::MoonGift(MoonGift::Custom(name)) => Some(name),
			_ => None,
		}
	}

	// pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
	// 	match self {
	// 		Ability::Merit(merit) => merit.get_modifiers(value),
	// 		Ability::Discipline(discipline) => discipline.get_modifiers(value),
	// 		Ability::MoonGift(moon_gift) => moon_gift.get_modifiers(value),
	// 		_ => vec![],
	// 	}
	// }

	#[must_use]
	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Ability::Merit(Merit::Custom(_))
				| Ability::Discipline(Discipline::Custom(_))
				| Ability::MoonGift(MoonGift::Custom(_))
		)
	}
}

pub type CModifier = (Trait, AttributeModifier<Trait, u8, COp>);

#[enum_dispatch]
pub trait AbilityTrait {
	fn get_modifiers(&self) -> Box<[CModifier]> {
		Box::default()
	}
}
