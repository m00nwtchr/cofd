use std::collections::HashMap;

use cofd_schema::{
	dice_pool::DicePool,
	prelude::{Attribute, Skill},
	traits::{DerivedTrait, Trait},
};
use damage::Damage;
use enum_dispatch::enum_dispatch;
use info::{CharacterInfo, Weapon};
use serde::{Deserialize, Serialize};
use systema::prelude::{Actor, AttributeMap};

use crate::{
	CofDSystem,
	prelude::SplatTrait,
	splat::{Merit, ability::Ability},
	splat_attributes,
	util::is_empty_vec,
};

pub(crate) mod damage;
mod info;
pub mod modifier;
// pub mod traits;

fn defense_pool() -> DicePool {
	DicePool::min(Attribute::Wits, Attribute::Dexterity) + Skill::Athletics
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Character<S: SplatTrait> {
	pub splat: S,

	pub info: CharacterInfo,

	attributes: AttributeMap<CofDSystem>,
	pub specialties: HashMap<Skill, Vec<String>>,

	health: Damage,

	// pub willpower: u16,
	// pub fuel: u16,
	#[serde(skip_serializing_if = "is_empty_vec")]
	pub touchstones: Vec<String>,

	// #[serde(skip)]
	// pub abilities: HashMap<Ability, u16>,
	// #[serde(skip)]
	// pub merits: Vec<(Merit, u16)>,
	pub weapons: Vec<Weapon>,

	// base_armor: ArmorStruct,
	// pub beats: u16,
	// #[serde(skip_serializing_if = "is_zero")]
	// pub alternate_beats: u16,
	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,
}

impl<S: SplatTrait> Character<S> {
	pub fn builder() -> CharacterBuilder<S> {
		CharacterBuilder::default()
	}

	// pub fn add_ability(&mut self, ability: Ability, val: u16) {
	// 	self.abilities.insert(ability, val);
	// }
	//
	// pub fn has_ability(&self, key: &Ability) -> bool {
	// 	self.abilities.contains_key(key)
	// }
	//
	// pub fn remove_ability(&mut self, key: &Ability) -> Option<u16> {
	// 	self.abilities.remove(key)
	// }
	//
	// pub fn get_ability_value(&self, key: &Ability) -> Option<&u16> {
	// 	self.abilities.get(key)
	// }
	//
	// pub fn get_ability_value_mut(&mut self, key: &Ability) -> Option<&mut u16> {
	// 	self.abilities.get_mut(key)
	// }
	//
	// pub fn add_merit(&mut self, key: Merit) {
	// 	self.merits.push((key, 0));
	// }
	//
	// pub fn remove_merit(&mut self, i: usize) -> (Merit, u16) {
	// 	self.merits.remove(i)
	// }
	//
	// pub fn get_merit(&self, i: usize) -> Option<&(Merit, u16)> {
	// 	self.merits.get(i)
	// }
	//
	// pub fn get_merit_mut(&mut self, i: usize) -> Option<&mut (Merit, u16)> {
	// 	self.merits.get_mut(i)
	// }

	//
	// pub fn get_modifier(&self, target: impl Into<ModifierTarget>) -> i16 {
	// 	self.modifiers.get_modifier(self, target)
	// }
	//
	// pub fn get_conditional_modifier(
	// 	&self,
	// 	target: impl Into<ModifierTarget>,
	// 	condition: impl Into<Condition>,
	// ) -> Option<i16> {
	// 	self.modifiers
	// 		.get_conditional_modifier(self, target, condition)
	// }
	//
	// pub fn get_pool(&self, target: impl Into<ModifierTarget>) -> Option<DicePool> {
	// 	self.modifiers.get_pool(self, target)
	// }
	//
	// pub fn get_conditional_pool(
	// 	&self,
	// 	target: impl Into<ModifierTarget>,
	// 	condition: impl Into<Condition>,
	// ) -> Option<DicePool> {
	// 	self.modifiers.get_conditional_pool(target, condition)
	// }

	pub fn health(&self) -> &Damage {
		&self.health
	}

	pub fn health_mut(&mut self) -> &mut Damage {
		&mut self.health
	}

	pub fn wound_penalty(&self) -> u8 {
		let mh = self
			.attributes
			.value(&Trait::DerivedTrait(DerivedTrait::Health))
			.unwrap_or_default();
		match mh - self.health.total().min(mh) {
			2 => 1,
			1 => 2,
			0 => 3,
			_ => 0,
		}
	}

	// #[allow(clippy::cast_sign_loss)]
	// pub fn defense(&self) -> u16 {
	// 	let res = self
	// 		.get_pool(Trait::DerivedTrait(DerivedTrait::Defense))
	// 		.unwrap()
	// 		.value(self);
	//
	// 	if res > 0 { res as u16 } else { 0 }
	// }

	pub fn max_fuel(&self) -> Option<u8> {
		self.splat
			.template()
			.supernatural_tolerance()
			.and_then(|st| self.attributes.value(&Trait::SupernaturalTolerance(st)))
			.map(|st| match st {
				0 => self
					.attributes
					.value(&Trait::Attribute(Attribute::Stamina))
					.unwrap_or_default(),
				1..=4 => 10 + st - 1,
				5..=8 => 10 + (st - 4) * 5,
				9 => 50,
				10 => 75,
				_ => 0,
			})
	}

	// pub fn armor(&self) -> ArmorStruct {
	// 	ArmorStruct {
	// 		general: self.base_armor.general,
	// 		ballistic: self.base_armor.ballistic,
	// 	}
	// }
	// pub fn base_armor_mut(&mut self) -> &mut ArmorStruct {
	// 	&mut self.base_armor
	// }

	// pub fn experience(&self) -> u16 {
	// 	self.beats / 5
	// }
	// pub fn alternate_experience(&self) -> u16 {
	// 	self.alternate_beats / 5
	// }
}

impl<S: SplatTrait> Actor for Character<S> {
	type System = CofDSystem;
	type Kind = S;

	fn new(splat: Self::Kind) -> Self {
		let attributes = AttributeMap::<CofDSystem>::new(splat_attributes(splat.template()));

		Self {
			splat,
			attributes,
			..Character::default()
		}
	}

	fn attributes(&self) -> &AttributeMap<Self::System> {
		&self.attributes
	}

	fn attributes_mut(&mut self) -> &mut AttributeMap<Self::System> {
		&mut self.attributes
	}
}

#[enum_dispatch]
pub trait CharacterTrait: Actor<System = CofDSystem> {
	fn attributes(&self) -> &AttributeMap<Self::System> {
		Actor::attributes(self)
	}
	fn attributes_mut(&mut self) -> &mut AttributeMap<Self::System> {
		Actor::attributes_mut(self)
	}
}

impl<S: SplatTrait> CharacterTrait for Character<S> {}

#[derive(Default)]
#[must_use]
pub struct CharacterBuilder<S: SplatTrait> {
	splat: S,
	info: CharacterInfo,
	attributes: Vec<(Attribute, u8)>,
	skills: Vec<(Skill, u8)>,
	specialties: HashMap<Skill, Vec<String>>,
	merits: Vec<(Merit, u8)>,

	abilities: HashMap<Ability, u8>,
	power: u8,
	fuel: Option<u8>,

	flag: bool,
}

impl<S: SplatTrait> CharacterBuilder<S> {
	pub fn with_splat(mut self, splat: S) -> Self {
		self.splat = splat;
		self
	}

	pub fn with_info(mut self, info: CharacterInfo) -> Self {
		self.info = info;
		self
	}

	pub fn with_attributes(mut self, attributes: Vec<(Attribute, u8)>) -> Self {
		self.attributes = attributes;
		self
	}

	pub fn with_skills(mut self, skills: Vec<(Skill, u8)>) -> Self {
		self.skills = skills;
		self
	}

	pub fn with_specialties(mut self, skill: Skill, specialties: Vec<String>) -> Self {
		self.specialties.insert(skill, specialties);
		self
	}

	// TODO
	pub fn with_abilities<const N: usize>(mut self, abilities: [(Ability, u8); N]) -> Self {
		self.abilities = HashMap::from(abilities);

		self.flag = true;
		self
	}

	// TODO
	pub fn with_merits<const N: usize>(mut self, merits: [(Merit, u8); N]) -> Self {
		self.merits = Vec::from(merits);
		self.flag = true;
		self
	}

	pub fn with_st(mut self, st: u8) -> Self {
		self.power = st;
		self
	}

	pub fn with_fuel(mut self, fuel: u8) -> Self {
		self.fuel = Some(fuel);
		self
	}

	pub fn build(self) -> Character<S> {
		let mut character = Character {
			info: self.info,
			specialties: self.specialties,
			..Character::new(self.splat)
		};

		let template = character.splat.template();

		for (attribute, value) in self.attributes {
			character
				.attributes
				.set_raw_value(&Trait::Attribute(attribute), value);
		}

		for (skill, value) in self.skills {
			character
				.attributes
				.set_raw_value(&Trait::Skill(skill), value);
		}

		if let Some(st) = template.supernatural_tolerance() {
			if self.power > 1 {
				character
					.attributes
					.set_raw_value(&Trait::SupernaturalTolerance(st), self.power);
			}
		}

		if template.fuel().is_some() {
			// TODO
		}

		character
	}
}

impl<S: SplatTrait> From<CharacterBuilder<S>> for Character<S> {
	fn from(builder: CharacterBuilder<S>) -> Self {
		builder.build()
	}
}
