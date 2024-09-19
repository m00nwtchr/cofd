use std::{
	collections::HashMap,
	fmt,
	fmt::{Debug, Formatter},
	ops::{Add, Sub},
};

use carboxyl::{lift, Signal, Sink};
use cofd_schema::prelude::Skill;
use serde::{Deserialize, Serialize};

use crate::{
	dice_pool::DicePoolExt,
	is_zero,
	prelude::VariantName,
	reactive::{RxAttributes, RxSkills, RxTrait},
	splat::{ability::Ability, Merit, Splat},
	traits::*,
};

pub mod attributes;
pub mod health;
pub mod info;
pub mod modifier;
pub mod skills;
// pub mod traits;

use attributes::Attributes;
use health::Health;
use info::CharacterInfo;
use modifier::*;
use skills::Skills;

#[derive(Default)]
pub struct CharacterBuilder {
	splat: Splat,
	info: CharacterInfo,
	attributes: Attributes,
	skills: Skills,
	specialties: HashMap<Skill, Vec<String>>,
	merits: Vec<(Merit, u8)>,

	abilities: HashMap<Ability, u8>,
	power: u8,
	fuel: Option<u8>,

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
	pub fn with_attributes(mut self, attributes: Attributes) -> Self {
		self.attributes = attributes;
		self
	}

	#[must_use]
	pub fn with_skills(mut self, skills: Skills) -> Self {
		self.skills = skills;
		self
	}

	#[must_use]
	pub fn with_specialties(mut self, skill: Skill, specialties: Vec<String>) -> Self {
		self.specialties.insert(skill, specialties);
		self
	}

	#[must_use]
	pub fn with_abilities<const N: usize>(mut self, abilities: [(Ability, u8); N]) -> Self {
		self.abilities = HashMap::from(abilities);

		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_merits<const N: usize>(mut self, merits: [(Merit, u8); N]) -> Self {
		self.merits = Vec::from(merits);
		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_st(mut self, st: u8) -> Self {
		self.power = st;
		self
	}

	#[must_use]
	pub fn with_fuel(mut self, fuel: u8) -> Self {
		self.fuel = Some(fuel);
		self
	}

	#[must_use]
	pub fn build(self) -> Character {
		let power = RxTrait::new(if matches!(self.splat, Splat::Mortal(..)) {
			0
		} else if self.power > 0 {
			self.power
		} else {
			1
		});

		let size = RxTrait::new(5);

		let attributes = RxAttributes::from(self.attributes);
		let skills = RxSkills::from(self.skills);

		let perception = perception(&attributes);
		let initiative = initiative(&attributes);
		let speed = speed(&attributes);

		let willpower = Willpower::new(&attributes);
		let health = Health::new(&attributes, &size);

		let defense = Defense::new(&attributes, &skills);

		let mut character = Character {
			splat: self.splat,
			info: self.info,
			power,
			fuel: 0,
			integrity: 0,
			attributes,
			skills,
			abilities: self.abilities,
			merits: self.merits,
			weapons: vec![],
			perception,
			initiative,
			speed,
			size,
			base_armor: Default::default(),
			beats: 0,
			alternate_beats: 0,
			conditions: vec![],
			aspirations: vec![],
			specialties: self.specialties,

			health,
			willpower,

			touchstones: vec![],
			modifiers: Default::default(),
			defense,
			//..Default::default()
		};

		if self.flag {
			character.calc_mod_map();
		}

		if let Splat::Werewolf(werewolf) = &character.splat {
			werewolf.modifiers(&character);
		}

		// character.fuel = self.fuel.unwrap_or_else(|| character.max_fuel());
		// character.willpower = character.max_willpower();
		character.willpower.set_current(character.willpower.max());

		character
	}
}

pub fn is_empty_vec(vec: &Vec<String>) -> bool {
	vec.is_empty()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_five(n: &u8) -> bool {
	*n == 5
}

#[derive(Clone, Debug, Serialize)]
#[serde(default)]
pub struct Character {
	pub splat: Splat,

	pub info: CharacterInfo,

	attributes: RxAttributes,
	skills: RxSkills,
	pub specialties: HashMap<Skill, Vec<String>>,

	health: Health,

	pub willpower: Willpower,
	pub power: RxTrait,
	pub fuel: u8,
	pub integrity: u8,

	#[serde(skip_serializing_if = "is_empty_vec")]
	pub touchstones: Vec<String>,

	// #[serde(skip)]
	pub abilities: HashMap<Ability, u8>,
	// #[serde(skip)]
	pub merits: Vec<(Merit, u8)>,

	pub weapons: Vec<Weapon>,

	pub size: RxTrait,
	pub defense: Defense,
	pub initiative: RxTrait,
	pub speed: RxTrait,
	base_armor: ArmorStruct,
	pub perception: RxTrait,

	pub beats: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub alternate_beats: u8,

	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,

	#[serde(skip)]
	modifiers: Modifiers,
}

impl Character {
	pub fn builder() -> CharacterBuilder {
		CharacterBuilder::default()
	}

	pub fn add_ability(&mut self, ability: Ability, val: u8) {
		self.abilities.insert(ability, val);
	}

	pub fn has_ability(&self, key: &Ability) -> bool {
		self.abilities.contains_key(key)
	}

	pub fn remove_ability(&mut self, key: &Ability) -> Option<u8> {
		self.abilities.remove(key)
	}

	pub fn get_ability_value(&self, key: &Ability) -> Option<&u8> {
		self.abilities.get(key)
	}

	pub fn get_ability_value_mut(&mut self, key: &Ability) -> Option<&mut u8> {
		self.abilities.get_mut(key)
	}

	pub fn add_merit(&mut self, key: Merit) {
		self.merits.push((key, 0));
	}

	pub fn remove_merit(&mut self, i: usize) -> (Merit, u8) {
		self.merits.remove(i)
	}

	pub fn get_merit(&self, i: usize) -> Option<&(Merit, u8)> {
		self.merits.get(i)
	}

	pub fn get_merit_mut(&mut self, i: usize) -> Option<&mut (Merit, u8)> {
		self.merits.get_mut(i)
	}

	pub fn get_trait(&self, trait_: &Trait) -> u8 {
		match trait_ {
			// Trait::DerivedTrait(dt) => match dt {
			// 	DerivedTrait::Speed => self.speed(),
			// 	DerivedTrait::Defense => self.defense(),
			// 	DerivedTrait::Initiative => self.initiative(),
			// 	DerivedTrait::Perception => self.perception(),
			// 	DerivedTrait::Health => self.max_health(),
			// 	DerivedTrait::Willpower => self.max_willpower(),
			// 	DerivedTrait::Size => self.size(),
			// },
			Trait::Beats => self.beats,

			// Trait::Armor(Some(armor)) => match armor {
			// 	Armor::General => self.armor().general,
			// 	Armor::Ballistic => self.armor().ballistic,
			// },
			// Trait::Power => self.power,
			Trait::Fuel => self.fuel,
			Trait::Integrity => self.integrity,

			Trait::Attribute(attr) => self.attributes().get(*attr).value(),
			Trait::Skill(skill) => self.skills().get(*skill).value(),
			_ => 0,
		}
	}

	pub fn calc_mod_map(&self) {
		// self.modifiers.update(self);
	}

	pub fn attributes(&self) -> &RxAttributes {
		&self.attributes
	}
	pub fn skills(&self) -> &RxSkills {
		&self.skills
	}
	pub fn health(&self) -> &Health {
		&self.health
	}
	pub fn size(&self) -> &RxTrait {
		&self.size
	}

	// pub fn _modified(&self, target: impl Into<ModifierTarget>) -> u8 {
	// 	let target = &target.into();
	// 	let base = match &target {
	// 		ModifierTarget::BaseAttribute(attr) | ModifierTarget::Attribute(attr) => {
	// 			*self.attributes.get(attr)
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
	// 	let res = base.saturating_add_signed(base_modifier);
	// 	if res > 5 {
	// 		res
	// 	} else {
	// 		base.saturating_add_signed(base_modifier + modifier)
	// 	}
	// }

	pub fn armor(&self) -> ArmorStruct {
		ArmorStruct {
			general: self.base_armor.general,
			ballistic: self.base_armor.ballistic,
		}
	}
	pub fn base_armor_mut(&mut self) -> &mut ArmorStruct {
		&mut self.base_armor
	}

	pub fn experience(&self) -> u8 {
		self.beats / 5
	}
	pub fn alternate_experience(&self) -> u8 {
		self.alternate_beats / 5
	}
}

fn perception(attributes: &RxAttributes) -> RxTrait {
	&attributes.wits + &attributes.composure
}

pub fn initiative(attributes: &RxAttributes) -> RxTrait {
	&attributes.dexterity + &attributes.composure
}

pub fn speed(attributes: &RxAttributes) -> RxTrait {
	(&attributes.dexterity + &attributes.strength).map(|a| a + 5)
}

fn max_fuel(power: &RxTrait, attributes: &RxAttributes) -> RxTrait {
	RxTrait::from(lift!(
		|power, stamina| {
			match power {
				0 => stamina,
				1..=4 => 10 + power - 1,
				5..=8 => 10 + (power - 4) * 5,
				9 => 50,
				10 => 75,
				_ => 0,
			}
		},
		power.signal(),
		attributes.stamina.signal()
	))
}

#[derive(Clone)]
struct DefenseCalc {
	attributes: (Signal<u8>, Signal<u8>),
	skill: Signal<u8>,
	flag: Signal<bool>,
}

#[derive(Clone, Serialize)]
pub struct Defense {
	value: RxTrait,
	#[serde(skip)]
	sink: Sink<DefenseCalc>,
}

impl Defense {
	pub fn new(attributes: &RxAttributes, skills: &RxSkills) -> Self {
		let sink = Sink::new();
		let defense = sink
			.stream()
			.hold(DefenseCalc {
				attributes: (
					attributes.wits.signal().clone(),
					attributes.dexterity.signal().clone(),
				),
				skill: skills.athletics.signal().clone(),
				flag: Signal::new(false),
			})
			.map(|a| {
				lift!(
					|a1, a2, skill, flag| if flag {
						a1.max(a2) + skill
					} else {
						a1.min(a2) + skill
					},
					&a.attributes.0,
					&a.attributes.1,
					&a.skill,
					&a.flag
				)
			})
			.switch();

		Self {
			sink,
			value: RxTrait::from(defense),
		}
	}

	pub fn value(&self) -> u8 {
		self.value.value()
	}

	pub fn signal(&self) -> &Signal<u8> {
		self.value.signal()
	}
}

impl Debug for Defense {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Defense")
			.field("value", &self.value)
			.finish()
	}
}

#[derive(Clone, Serialize)]
pub struct Willpower {
	max: RxTrait,
	#[serde(skip)]
	current_sink: Sink<u8>,
	#[serde(skip)]
	current: Signal<u8>,
}

impl Debug for Willpower {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Willpower")
			.field("max", &self.max)
			.field("current", &self.current)
			.finish()
	}
}

impl Willpower {
	pub fn new(attributes: &RxAttributes) -> Self {
		let max = &attributes.resolve + &attributes.composure;

		let current_sink = Sink::new();
		let current = lift!(
			|w, max| w.min(max),
			&current_sink.stream().hold(0),
			&max.signal()
		);

		Self {
			max,
			current_sink,
			current,
		}
	}

	pub fn max(&self) -> u8 {
		self.max.value()
	}

	pub fn set_current(&self, current: u8) {
		self.current_sink.send(current);
	}

	pub fn current(&self) -> u8 {
		self.current.sample()
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ArmorStruct {
	pub general: u8,
	pub ballistic: u8,
}

#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Armor {
	General,
	Ballistic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Weapon {
	pub name: String,
	pub dice_pool: String,
	pub damage: String,
	pub range: String,
	pub initative: i8,
	pub size: u8,
}

impl Default for Character {
	fn default() -> Self {
		let attributes = RxAttributes::default();
		let skills = RxSkills::default();

		let size = RxTrait::new(5);

		let willpower = Willpower::new(&attributes);
		let health = Health::new(&attributes, &size);

		let perception = perception(&attributes);
		let initiative = initiative(&attributes);
		let speed = speed(&attributes);
		let defense = Defense::new(&attributes, &skills);

		Self {
			splat: Default::default(),
			info: Default::default(),
			attributes,
			skills,
			size,
			abilities: Default::default(),
			merits: Default::default(),
			health,

			modifiers: Default::default(),

			power: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			willpower,
			beats: Default::default(),
			alternate_beats: Default::default(),
			base_armor: Default::default(),
			specialties: Default::default(),
			touchstones: Default::default(),
			conditions: Default::default(),
			aspirations: Default::default(),
			weapons: Default::default(),
			perception,
			initiative,
			speed,
			defense,
		}
	}
}
