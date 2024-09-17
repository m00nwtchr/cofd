use std::{
	cmp::min,
	collections::HashMap,
	ops::{Add, Sub},
};

use carboxyl::{Signal, Sink};
use cofd_schema::{
	dice_pool::DicePool,
	prelude::{Attribute, Skill},
};
use serde::{Deserialize, Serialize};

use crate::{
	prelude::VariantName,
	splat::{ability::Ability, Merit, Splat},
};

pub mod modifier;
// pub mod traits;

use modifier::*;

use crate::{
	dice_pool::DicePoolExt,
	observer::{RxAttribute, RxAttributes},
	traits::*,
};

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
		let power = if matches!(self.splat, Splat::Mortal(..)) {
			0
		} else if self.power > 0 {
			self.power
		} else {
			1
		};

		let mut character = Character {
			splat: self.splat,
			info: self.info,
			power,
			attributes: RxAttributes::from(self.attributes),
			skills: self.skills,
			abilities: self.abilities,
			merits: self.merits,
			specialties: self.specialties,
			..Default::default()
		};

		if self.flag {
			character.calc_mod_map();
		}

		character.fuel = self.fuel.unwrap_or_else(|| character.max_fuel());
		character.willpower = character.max_willpower();

		character
	}
}

#[derive(Default, Debug, Clone)]
pub enum Wound {
	#[default]
	None,
	Bashing,
	Lethal,
	Aggravated,
}

impl Wound {
	#[must_use]
	pub fn inc(&self) -> Wound {
		match self {
			Wound::None => Wound::Bashing,
			Wound::Bashing => Wound::Lethal,
			Wound::Lethal => Wound::Aggravated,
			Wound::Aggravated => Wound::Aggravated,
		}
	}

	#[must_use]
	pub fn poke(&self) -> Wound {
		if let Wound::Aggravated = self {
			Wound::None
		} else {
			self.inc()
		}
	}

	pub fn poke_mut(&mut self) {
		*self = self.poke();
	}
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Damage {
	#[serde(skip_serializing_if = "is_zero")]
	aggravated: u8,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u8,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u8,
}

impl Damage {
	pub fn new(bashing: u8, lethal: u8, aggravated: u8) -> Self {
		Self {
			aggravated,
			lethal,
			bashing,
		}
	}

	pub fn get(&self, wound: &Wound) -> u8 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	pub fn get_i(&self, i: usize) -> Wound {
		// println!("{i}");
		if i < self.aggravated as usize {
			Wound::Aggravated
		} else if i >= self.aggravated as usize && i < (self.aggravated + self.lethal) as usize {
			Wound::Lethal
		} else if i >= (self.aggravated + self.lethal) as usize
			&& i < (self.aggravated + self.lethal + self.bashing) as usize
		{
			Wound::Bashing
		} else {
			Wound::None
		}
	}

	pub fn sum(&self) -> u8 {
		self.bashing + self.lethal + self.aggravated
	}

	pub fn dec(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}

	pub fn inc(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => self.bashing += 1,
			Wound::Lethal => self.lethal += 1,
			Wound::Aggravated => self.aggravated += 1,
		}
	}

	pub fn poke(&mut self, wound: &Wound) {
		match wound {
			Wound::None => self.bashing += 1,
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
				self.lethal += 1;
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
				self.aggravated += 1;
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}
}

pub fn defense_pool() -> DicePool {
	DicePool::min(Attribute::Wits, Attribute::Dexterity) + Skill::Athletics
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
	skills: Skills,
	pub specialties: HashMap<Skill, Vec<String>>,

	health: Damage,

	pub willpower: Willpower,
	pub max_willpower: RxAttribute<u8>,
	pub power: u8,
	pub fuel: u8,
	pub integrity: u8,

	#[serde(skip_serializing_if = "is_empty_vec")]
	pub touchstones: Vec<String>,

	// #[serde(skip)]
	pub abilities: HashMap<Ability, u8>,
	// #[serde(skip)]
	pub merits: Vec<(Merit, u8)>,

	pub weapons: Vec<Weapon>,

	#[serde(skip_serializing_if = "is_five")]
	pub size: RxAttribute<u8>,
	base_armor: ArmorStruct,
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
			Trait::DerivedTrait(dt) => match dt {
				DerivedTrait::Speed => self.speed(),
				DerivedTrait::Defense => self.defense(),
				DerivedTrait::Initiative => self.initiative(),
				DerivedTrait::Perception => self.perception(),
				DerivedTrait::Health => self.max_health(),
				DerivedTrait::Willpower => self.max_willpower(),
				DerivedTrait::Size => self.size(),
			},

			Trait::Beats => self.beats,

			// Trait::Armor(Some(armor)) => match armor {
			// 	Armor::General => self.armor().general,
			// 	Armor::Ballistic => self.armor().ballistic,
			// },
			Trait::Power => self.power,
			Trait::Fuel => self.fuel,
			Trait::Integrity => self.integrity,

			Trait::Attribute(attr) => *self.attributes().get(attr),
			Trait::Skill(skill) => self.skills().get(*skill),
			_ => 0,
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn calc_mod_map(&self) {
		self.modifiers.update(self);
	}

	pub fn get_modifier(&self, target: impl Into<ModifierTarget>) -> i8 {
		self.modifiers.get_modifier(self, target)
	}

	pub fn get_conditional_modifier(
		&self,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<i8> {
		self.modifiers
			.get_conditional_modifier(self, target, condition)
	}

	pub fn get_pool(&self, target: impl Into<ModifierTarget>) -> Option<DicePool> {
		self.modifiers.get_pool(self, target)
	}

	pub fn get_conditional_pool(
		&self,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<DicePool> {
		self.modifiers.get_conditional_pool(target, condition)
	}

	pub fn attributes(&self) -> &RxAttributes {
		&self.attributes
	}

	pub fn _modified(&self, target: impl Into<ModifierTarget>) -> u8 {
		// let target = &target.into();
		// let base = match &target {
		// 	ModifierTarget::BaseAttribute(attr) | ModifierTarget::Attribute(attr) => {
		// 		*self.attributes.get(attr)
		// 	}
		// 	ModifierTarget::BaseSkill(skill) | ModifierTarget::Skill(skill) => {
		// 		self.skills.get(*skill)
		// 	}
		// 	_ => 0,
		// };
		//
		// let modifier = match &target {
		// 	ModifierTarget::BaseAttribute(_) => 0,
		// 	ModifierTarget::BaseSkill(_) => 0,
		// 	_ => self.modifiers.get_modifier(self, target.clone()),
		// };
		// let base_modifier = self.modifiers.get_modifier(
		// 	self,
		// 	match *target {
		// 		ModifierTarget::Attribute(attr) => ModifierTarget::BaseAttribute(attr),
		// 		ModifierTarget::Skill(skill) => ModifierTarget::BaseSkill(skill),
		// 		_ => target.clone(),
		// 	},
		// );
		//
		// let res = base.saturating_add_signed(base_modifier);
		// if res > 5 {
		// 	res
		// } else {
		// 	base.saturating_add_signed(base_modifier + modifier)
		// }
		0
	}

	pub fn skills(&self) -> Skills {
		Skills {
			academics: self._modified(Skill::Academics),
			computer: self._modified(Skill::Computer),
			crafts: self._modified(Skill::Crafts),
			investigation: self._modified(Skill::Investigation),
			medicine: self._modified(Skill::Medicine),
			occult: self._modified(Skill::Occult),
			politics: self._modified(Skill::Politics),
			science: self._modified(Skill::Science),

			athletics: self._modified(Skill::Athletics),
			brawl: self._modified(Skill::Brawl),
			drive: self._modified(Skill::Drive),
			firearms: self._modified(Skill::Firearms),
			larceny: self._modified(Skill::Larceny),
			stealth: self._modified(Skill::Stealth),
			survival: self._modified(Skill::Survival),
			weaponry: self._modified(Skill::Weaponry),

			animal_ken: self._modified(Skill::AnimalKen),
			empathy: self._modified(Skill::Empathy),
			expression: self._modified(Skill::Expression),
			intimidation: self._modified(Skill::Intimidation),
			persuasion: self._modified(Skill::Persuasion),
			socialize: self._modified(Skill::Socialize),
			streetwise: self._modified(Skill::Streetwise),
			subterfuge: self._modified(Skill::Subterfuge),
		}
	}
	pub fn base_skills(&self) -> &Skills {
		&self.skills
	}
	pub fn base_skills_mut(&mut self) -> &mut Skills {
		&mut self.skills
	}

	pub fn max_health(&self) -> u8 {
		let attributes = self.attributes();

		(self.size() + attributes.stamina.value()).saturating_add_signed(
			self.modifiers
				.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Health)),
		)
	}

	pub fn health(&self) -> &Damage {
		&self.health
	}

	pub fn health_mut(&mut self) -> &mut Damage {
		&mut self.health
	}

	pub fn wound_penalty(&self) -> u8 {
		let mh = self.max_health();
		match mh - min(self.health.sum(), mh) {
			2 => 1,
			1 => 2,
			0 => 3,
			_ => 0,
		}
	}

	pub fn max_willpower(&self) -> RxAttribute<u8> {
		let attributes = self.attributes();

		&attributes.resolve + &attributes.composure
	}

	pub fn size(&self) -> &RxAttribute<u8> {
		&self.size
	}

	pub fn speed(&self) -> RxAttribute<u8> {
		let attributes = self.attributes();

		// self.modifiers
		// 	.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Speed));
		(&attributes.dexterity + &attributes.strength).map(|a| a + 5)
	}

	#[allow(clippy::cast_sign_loss)]
	pub fn defense(&self) -> u8 {
		let res = self
			.get_pool(Trait::DerivedTrait(DerivedTrait::Defense))
			.unwrap()
			.value(self);

		if res > 0 {
			res as u8
		} else {
			0
		}
	}
	pub fn armor(&self) -> ArmorStruct {
		ArmorStruct {
			general: self.base_armor.general,
			ballistic: self.base_armor.ballistic,
		}
	}
	pub fn base_armor_mut(&mut self) -> &mut ArmorStruct {
		&mut self.base_armor
	}
	pub fn initiative(&self) -> u8 {
		let attributes = self.attributes();

		(attributes.dexterity + attributes.composure).saturating_add_signed(
			self.modifiers
				.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Initiative)),
		)
	}
	pub fn perception(&self) -> u8 {
		let attributes = self.attributes();

		(attributes.wits + attributes.composure).saturating_add_signed(
			self.modifiers
				.get_modifier(self, Trait::DerivedTrait(DerivedTrait::Perception)),
		)
	}
	pub fn experience(&self) -> u8 {
		self.beats / 5
	}
	pub fn alternate_experience(&self) -> u8 {
		self.alternate_beats / 5
	}

	pub fn max_fuel(&self) -> u8 {
		match self.power {
			0 => self.attributes().stamina,
			1..=4 => 10 + self.power - 1,
			5..=8 => 10 + (self.power - 4) * 5,
			9 => 50,
			10 => 75,
			_ => 0,
		}
	}
}

#[derive(Clone, Serialize)]
pub struct Willpower {
	max: RxAttribute<u8>,
	current_sink: Sink<u8>,
	current: Signal<u8>,
}

impl Willpower {
	pub fn new(attributes: &RxAttributes) -> Self {
		let current_sink = Sink::new();
		let current = current_sink.stream().hold(0);

		Self {
			max: &attributes.resolve + &attributes.composure,
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

impl Default for Character {
	fn default() -> Self {
		let attributes = RxAttributes::default();

		let max_willpower = &attributes.resolve + &attributes.composure;

		Self {
			splat: Default::default(),
			info: Default::default(),
			attributes,
			skills: Default::default(),
			size: RxAttribute::new(5),
			abilities: Default::default(),
			merits: Default::default(),
			health: Default::default(),

			modifiers: Default::default(),

			power: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			willpower: Default::default(),
			beats: Default::default(),
			alternate_beats: Default::default(),
			base_armor: Default::default(),
			specialties: Default::default(),
			touchstones: Default::default(),
			conditions: Default::default(),
			aspirations: Default::default(),
			weapons: Default::default(),
		}
	}
}

fn is_empty(str: &String) -> bool {
	str.is_empty()
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct CharacterInfo {
	#[serde(skip_serializing_if = "is_empty")]
	pub name: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub player: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub virtue_anchor: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub vice_anchor: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub faction: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub group_name: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub concept: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub chronicle: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub age: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub date_of_birth: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub hair: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub eyes: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub race: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub nationality: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub height: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub weight: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub sex: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub other: String,
}

#[derive(Debug, Clone, Copy, VariantName)]
pub enum InfoTrait {
	Name,
	Age,
	Player,
	VirtueAnchor,
	ViceAnchor,
	Concept,
	Chronicle,

	Faction,
	GroupName,

	DateOfBirth,
	Hair,
	Eyes,
	Race,
	Nationality,
	Height,
	Weight,
	Sex,
}

impl CharacterInfo {
	pub fn get(&self, info: InfoTrait) -> &String {
		match info {
			InfoTrait::Name => &self.name,
			InfoTrait::Age => &self.age,
			InfoTrait::Player => &self.player,
			InfoTrait::Concept => &self.concept,
			InfoTrait::Chronicle => &self.chronicle,
			InfoTrait::DateOfBirth => &self.date_of_birth,
			InfoTrait::Hair => &self.hair,
			InfoTrait::Eyes => &self.eyes,
			InfoTrait::Race => &self.race,
			InfoTrait::Nationality => &self.nationality,
			InfoTrait::Height => &self.height,
			InfoTrait::Weight => &self.weight,
			InfoTrait::Sex => &self.sex,
			InfoTrait::VirtueAnchor => &self.virtue_anchor,
			InfoTrait::ViceAnchor => &self.vice_anchor,
			InfoTrait::Faction => &self.faction,
			InfoTrait::GroupName => &self.group_name,
		}
	}

	pub fn get_mut(&mut self, info: InfoTrait) -> &mut String {
		match info {
			InfoTrait::Name => &mut self.name,
			InfoTrait::Age => &mut self.age,
			InfoTrait::Player => &mut self.player,
			InfoTrait::Concept => &mut self.concept,
			InfoTrait::Chronicle => &mut self.chronicle,
			InfoTrait::DateOfBirth => &mut self.date_of_birth,
			InfoTrait::Hair => &mut self.hair,
			InfoTrait::Eyes => &mut self.eyes,
			InfoTrait::Race => &mut self.race,
			InfoTrait::Nationality => &mut self.nationality,
			InfoTrait::Height => &mut self.height,
			InfoTrait::Weight => &mut self.weight,
			InfoTrait::Sex => &mut self.sex,
			InfoTrait::VirtueAnchor => &mut self.virtue_anchor,
			InfoTrait::ViceAnchor => &mut self.vice_anchor,
			InfoTrait::Faction => &mut self.faction,
			InfoTrait::GroupName => &mut self.group_name,
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_one(num: &u8) -> bool {
	num.eq(&1)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Attributes {
	#[serde(skip_serializing_if = "is_one")]
	pub intelligence: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub wits: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub resolve: u8,

	#[serde(skip_serializing_if = "is_one")]
	pub strength: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub dexterity: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub stamina: u8,

	#[serde(skip_serializing_if = "is_one")]
	pub presence: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub manipulation: u8,
	#[serde(skip_serializing_if = "is_one")]
	pub composure: u8,
}

impl Attributes {
	pub fn get(&self, attr: &Attribute) -> &u8 {
		match attr {
			Attribute::Intelligence => &self.intelligence,
			Attribute::Wits => &self.wits,
			Attribute::Resolve => &self.resolve,
			//
			Attribute::Strength => &self.strength,
			Attribute::Dexterity => &self.dexterity,
			Attribute::Stamina => &self.stamina,
			//
			Attribute::Presence => &self.presence,
			Attribute::Manipulation => &self.manipulation,
			Attribute::Composure => &self.composure,
		}
	}

	pub fn get_mut(&mut self, attr: &Attribute) -> &mut u8 {
		match attr {
			Attribute::Intelligence => &mut self.intelligence,
			Attribute::Wits => &mut self.wits,
			Attribute::Resolve => &mut self.resolve,
			//
			Attribute::Strength => &mut self.strength,
			Attribute::Dexterity => &mut self.dexterity,
			Attribute::Stamina => &mut self.stamina,
			//
			Attribute::Presence => &mut self.presence,
			Attribute::Manipulation => &mut self.manipulation,
			Attribute::Composure => &mut self.composure,
		}
	}
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			intelligence: 1,
			wits: 1,
			resolve: 1,
			strength: 1,
			dexterity: 1,
			stamina: 1,
			presence: 1,
			manipulation: 1,
			composure: 1,
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(n: &u8) -> bool {
	*n == 0
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Skills {
	#[serde(skip_serializing_if = "is_zero")]
	pub academics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub computer: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub crafts: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub investigation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub medicine: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub occult: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub politics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub science: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub athletics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub brawl: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub drive: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub firearms: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub larceny: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub stealth: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub survival: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub weaponry: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub animal_ken: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub empathy: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub expression: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub intimidation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub persuasion: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub socialize: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub streetwise: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub subterfuge: u8,
}

impl Skills {
	pub fn get(&self, skill: Skill) -> u8 {
		match skill {
			Skill::Academics => self.academics,
			Skill::Computer => self.computer,
			Skill::Crafts => self.crafts,
			Skill::Investigation => self.investigation,
			Skill::Medicine => self.medicine,
			Skill::Occult => self.occult,
			Skill::Politics => self.politics,
			Skill::Science => self.science,
			//
			Skill::Athletics => self.athletics,
			Skill::Brawl => self.brawl,
			Skill::Drive => self.drive,
			Skill::Firearms => self.firearms,
			Skill::Larceny => self.larceny,
			Skill::Stealth => self.stealth,
			Skill::Survival => self.survival,
			Skill::Weaponry => self.weaponry,
			//
			Skill::AnimalKen => self.animal_ken,
			Skill::Empathy => self.empathy,
			Skill::Expression => self.expression,
			Skill::Intimidation => self.intimidation,
			Skill::Persuasion => self.persuasion,
			Skill::Socialize => self.socialize,
			Skill::Streetwise => self.streetwise,
			Skill::Subterfuge => self.subterfuge,
		}
	}

	pub fn get_mut(&mut self, skill: Skill) -> &mut u8 {
		match skill {
			Skill::Academics => &mut self.academics,
			Skill::Computer => &mut self.computer,
			Skill::Crafts => &mut self.crafts,
			Skill::Investigation => &mut self.investigation,
			Skill::Medicine => &mut self.medicine,
			Skill::Occult => &mut self.occult,
			Skill::Politics => &mut self.politics,
			Skill::Science => &mut self.science,
			//
			Skill::Athletics => &mut self.athletics,
			Skill::Brawl => &mut self.brawl,
			Skill::Drive => &mut self.drive,
			Skill::Firearms => &mut self.firearms,
			Skill::Larceny => &mut self.larceny,
			Skill::Stealth => &mut self.stealth,
			Skill::Survival => &mut self.survival,
			Skill::Weaponry => &mut self.weaponry,
			//
			Skill::AnimalKen => &mut self.animal_ken,
			Skill::Empathy => &mut self.empathy,
			Skill::Expression => &mut self.expression,
			Skill::Intimidation => &mut self.intimidation,
			Skill::Persuasion => &mut self.persuasion,
			Skill::Socialize => &mut self.socialize,
			Skill::Streetwise => &mut self.streetwise,
			Skill::Subterfuge => &mut self.subterfuge,
		}
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
