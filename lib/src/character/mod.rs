use std::{collections::HashMap, ops::Deref};

use cofd_schema::{
	dice_pool::DicePool,
	prelude::{Attribute, Skill},
	template::Template,
	traits::Trait,
};
use damage::Damage;
use info::{CharacterInfo, Weapon};
use serde::{Deserialize, Serialize};
use systema::prelude::{Actor, AttributeMap};

use crate::{
	CofDSystem,
	splat::{Merit, Splat, ability::Ability},
	splat_attributes,
	util::is_empty_vec,
};

pub(crate) mod damage;
mod info;
pub mod modifier;
// pub mod traits;

#[derive(Default)]
pub struct CharacterBuilder {
	splat: Splat,
	info: CharacterInfo,
	attributes: Vec<(Attribute, u8)>,
	skills: Vec<(Skill, u8)>,
	specialties: HashMap<Skill, Vec<String>>,
	merits: Vec<(Merit, u16)>,

	abilities: HashMap<Ability, u16>,
	power: u16,
	fuel: Option<u16>,

	flag: bool,
}

impl CharacterBuilder {
	#[must_use]
	pub fn with_splat(mut self, splat: impl Into<Splat>) -> Self {
		self.splat = splat.into();
		self
	}

	#[must_use]
	pub fn with_info(mut self, info: CharacterInfo) -> Self {
		self.info = info;
		self
	}

	#[must_use]
	pub fn with_attributes(mut self, attributes: Vec<(Attribute, u8)>) -> Self {
		self.attributes = attributes;
		self
	}

	#[must_use]
	pub fn with_skills(mut self, skills: Vec<(Skill, u8)>) -> Self {
		self.skills = skills;
		self
	}

	#[must_use]
	pub fn with_specialties(mut self, skill: Skill, specialties: Vec<String>) -> Self {
		self.specialties.insert(skill, specialties);
		self
	}

	#[must_use]
	pub fn with_abilities<const N: usize>(mut self, abilities: [(Ability, u16); N]) -> Self {
		self.abilities = HashMap::from(abilities);

		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_merits<const N: usize>(mut self, merits: [(Merit, u16); N]) -> Self {
		self.merits = Vec::from(merits);
		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_st(mut self, st: u16) -> Self {
		self.power = st;
		self
	}

	#[must_use]
	pub fn with_fuel(mut self, fuel: u16) -> Self {
		self.fuel = Some(fuel);
		self
	}

	#[must_use]
	pub fn build(self) -> Character {
		let mut character = Character {
			info: self.info,
			specialties: self.specialties,
			..Character::new(self.splat)
		};

		for (attribute, value) in self.attributes {
			character
				.attributes
				.set_raw_value(Trait::Attribute(attribute), value);
		}

		for (skill, value) in self.skills {
			character
				.attributes
				.set_raw_value(Trait::Skill(skill), value);
		}

		// for (skill, value) in self.skills {
		// 	character
		// 		.attributes
		// 		.set_raw_value(Trait::Skill(skill), value);
		// }

		character
	}
}

pub fn defense_pool() -> DicePool {
	DicePool::min(Attribute::Wits, Attribute::Dexterity) + Skill::Athletics
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Character {
	pub splat: Splat,

	pub info: CharacterInfo,

	attributes: AttributeMap<CofDSystem>,

	// #[serde(rename = "attributes")]
	// _attributes: Attributes,
	// skills: Skills,
	pub specialties: HashMap<Skill, Vec<String>>,

	health: Damage,

	// pub willpower: u16,
	// pub power: u16,
	// pub fuel: u16,
	// pub integrity: u16,
	#[serde(skip_serializing_if = "is_empty_vec")]
	pub touchstones: Vec<String>,

	// #[serde(skip)]
	// pub abilities: HashMap<Ability, u16>,
	// #[serde(skip)]
	// pub merits: Vec<(Merit, u16)>,
	pub weapons: Vec<Weapon>,

	// #[serde(skip_serializing_if = "is_five")]
	// pub base_size: u16,
	// base_armor: ArmorStruct,
	// pub beats: u16,
	// #[serde(skip_serializing_if = "is_zero")]
	// pub alternate_beats: u16,
	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,
	// #[serde(skip)]
	// modifiers: Modifiers,
}

impl Character {
	pub fn builder() -> CharacterBuilder {
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

	// pub fn get_trait(&self, trait_: &Trait) -> u16 {
	// 	match trait_ {
	// 		Trait::DerivedTrait(dt) => match dt {
	// 			DerivedTrait::Speed => self.speed(),
	// 			DerivedTrait::Defense => self.defense(),
	// 			DerivedTrait::Initiative => self.initiative(),
	// 			DerivedTrait::Perception => self.perception(),
	// 			DerivedTrait::Health => self.max_health(),
	// 			DerivedTrait::Willpower => self.max_willpower(),
	// 			DerivedTrait::Size => self.size(),
	// 		},
	//
	// 		Trait::Beats => self.beats,
	//
	// 		// Trait::Armor(Some(armor)) => match armor {
	// 		// 	Armor::General => self.armor().general,
	// 		// 	Armor::Ballistic => self.armor().ballistic,
	// 		// },
	// 		Trait::Power => self.power,
	// 		Trait::Fuel => self.fuel,
	// 		Trait::Integrity => self.integrity,
	//
	// 		Trait::Attribute(attr) => *self.attributes().get(attr),
	// 		Trait::Skill(skill) => self.skills().get(*skill),
	// 		_ => 0,
	// 	}
	// }

	// #[allow(clippy::too_many_lines)]
	// pub fn calc_mod_map(&self) {
	// 	self.modifiers.update(self);
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

	// pub fn attributes(&self) -> Attributes {
	// 	Attributes {
	// 		intelligence: self._modified(Attribute::Intelligence),
	// 		wits: self._modified(Attribute::Wits),
	// 		resolve: self._modified(Attribute::Resolve),
	// 		strength: self._modified(Attribute::Strength),
	// 		dexterity: self._modified(Attribute::Dexterity),
	// 		stamina: self._modified(Attribute::Stamina),
	// 		presence: self._modified(Attribute::Presence),
	// 		manipulation: self._modified(Attribute::Manipulation),
	// 		composure: self._modified(Attribute::Composure),
	// 	}
	// }
	// pub fn base_attributes_mut(&mut self) -> &mut Attributes {
	// 	&mut self._attributes
	// }
	// pub fn base_attributes(&self) -> &Attributes {
	// 	&self._attributes
	// }

	// pub fn _modified(&self, target: impl Into<ModifierTarget>) -> u16 {
	// 	let target = &target.into();
	// 	let base = match &target {
	// 		ModifierTarget::BaseAttribute(attr) | ModifierTarget::Attribute(attr) => {
	// 			*self._attributes.get(attr)
	// 		}
	// 		ModifierTarget::BaseSkill(skill) | ModifierTarget::Skill(skill) => {
	// 			self.skills.get(*skill)
	// 		}
	// 		_ => 0,
	// 	};
	//
	// 	let modifier = match &target {
	// 		ModifierTarget::BaseAttribute(_) => 0,
	// 		ModifierTarget::BaseSkill(_) => 0,
	// 		_ => self.modifiers.get_modifier(self, target.clone()),
	// 	};
	// 	let base_modifier = self.modifiers.get_modifier(
	// 		self,
	// 		match *target {
	// 			ModifierTarget::Attribute(attr) => ModifierTarget::BaseAttribute(attr),
	// 			ModifierTarget::Skill(skill) => ModifierTarget::BaseSkill(skill),
	// 			_ => target.clone(),
	// 		},
	// 	);
	//
	// 	if add(base, base_modifier) > 5 {
	// 		add(base, modifier)
	// 	} else {
	// 		add(base, base_modifier + modifier)
	// 	}
	// }

	// pub fn skills(&self) -> Skills {
	// 	Skills {
	// 		academics: self._modified(Skill::Academics),
	// 		computer: self._modified(Skill::Computer),
	// 		crafts: self._modified(Skill::Crafts),
	// 		investigation: self._modified(Skill::Investigation),
	// 		medicine: self._modified(Skill::Medicine),
	// 		occult: self._modified(Skill::Occult),
	// 		politics: self._modified(Skill::Politics),
	// 		science: self._modified(Skill::Science),
	//
	// 		athletics: self._modified(Skill::Athletics),
	// 		brawl: self._modified(Skill::Brawl),
	// 		drive: self._modified(Skill::Drive),
	// 		firearms: self._modified(Skill::Firearms),
	// 		larceny: self._modified(Skill::Larceny),
	// 		stealth: self._modified(Skill::Stealth),
	// 		survival: self._modified(Skill::Survival),
	// 		weaponry: self._modified(Skill::Weaponry),
	//
	// 		animal_ken: self._modified(Skill::AnimalKen),
	// 		empathy: self._modified(Skill::Empathy),
	// 		expression: self._modified(Skill::Expression),
	// 		intimidation: self._modified(Skill::Intimidation),
	// 		persuasion: self._modified(Skill::Persuasion),
	// 		socialize: self._modified(Skill::Socialize),
	// 		streetwise: self._modified(Skill::Streetwise),
	// 		subterfuge: self._modified(Skill::Subterfuge),
	// 	}
	// }
	// pub fn base_skills(&self) -> &Skills {
	// 	&self.skills
	// }
	// pub fn base_skills_mut(&mut self) -> &mut Skills {
	// 	&mut self.skills
	// }

	// pub fn max_health(&self) -> u16 {
	// 	let attributes = self.attributes();
	//
	// 	add(
	// 		self.size() + attributes.stamina,
	// 		self.modifiers
	// 			.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Health)),
	// 	)
	// }

	pub fn health(&self) -> &Damage {
		&self.health
	}

	pub fn health_mut(&mut self) -> &mut Damage {
		&mut self.health
	}

	// pub fn wound_penalty(&self) -> u16 {
	// 	let mh = self.max_health();
	// 	match mh - min(self.health.sum(), mh) {
	// 		2 => 1,
	// 		1 => 2,
	// 		0 => 3,
	// 		_ => 0,
	// 	}
	// }

	// pub fn max_willpower(&self) -> u16 {
	// 	let attributes = self.attributes();
	//
	// 	attributes.resolve + attributes.composure
	// }

	// pub fn size(&self) -> u16 {
	// 	add(
	// 		self.base_size,
	// 		self.modifiers
	// 			.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Size)),
	// 	)
	// }
	// pub fn speed(&self) -> u16 {
	// 	let attributes = self.attributes();
	//
	// 	add(
	// 		5 + attributes.dexterity + attributes.strength,
	// 		self.modifiers
	// 			.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Speed)),
	// 	)
	// }

	// #[allow(clippy::cast_sign_loss)]
	// pub fn defense(&self) -> u16 {
	// 	let res = self
	// 		.get_pool(Trait::DerivedTrait(DerivedTrait::Defense))
	// 		.unwrap()
	// 		.value(self);
	//
	// 	if res > 0 { res as u16 } else { 0 }
	// }
	// pub fn armor(&self) -> ArmorStruct {
	// 	ArmorStruct {
	// 		general: self.base_armor.general,
	// 		ballistic: self.base_armor.ballistic,
	// 	}
	// }
	// pub fn base_armor_mut(&mut self) -> &mut ArmorStruct {
	// 	&mut self.base_armor
	// }
	// pub fn initiative(&self) -> u16 {
	// 	let attributes = self.attributes();
	//
	// 	add(
	// 		attributes.dexterity + attributes.composure,
	// 		self.modifiers
	// 			.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Initiative)),
	// 	)
	// }
	// pub fn perception(&self) -> u16 {
	// 	let attributes = self.attributes();
	//
	// 	add(
	// 		attributes.wits + attributes.composure,
	// 		self.modifiers
	// 			.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Perception)),
	// 	)
	// }
	// pub fn experience(&self) -> u16 {
	// 	self.beats / 5
	// }
	// pub fn alternate_experience(&self) -> u16 {
	// 	self.alternate_beats / 5
	// }

	// pub fn max_fuel(&self) -> u16 {
	// 	match self.power {
	// 		0 => self.attributes().stamina,
	// 		1..=4 => 10 + self.power - 1,
	// 		5..=8 => 10 + (self.power - 4) * 5,
	// 		9 => 50,
	// 		10 => 75,
	// 		_ => 0,
	// 	}
	// }
}

impl Actor for Character {
	type System = CofDSystem;
	type Kind = Splat;

	fn new(splat: Self::Kind) -> Self {
		let attributes = AttributeMap::<CofDSystem>::new(splat_attributes(*splat));

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

impl From<CharacterBuilder> for Character {
	fn from(builder: CharacterBuilder) -> Self {
		builder.build()
	}
}
