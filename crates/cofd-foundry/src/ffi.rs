use std::{
	fmt::Display,
	str::FromStr,
	sync::{Arc, Mutex, MutexGuard},
};

use cofd::{
	ability::Ability,
	character::{
		Character,
		damage::{Damage, Wound},
		info::{CharacterInfo, InfoTrait, Weapon},
	},
	prelude::AllVariants,
	schema::{
		prelude::{Attribute, Skill},
		template::{Template, werewolf::Form},
		traits::{DerivedTrait, Trait as SchemaTrait},
	},
	splat::{
		SplatCharacter, SplatTrait,
		mortal::Mortal,
		werewolf::{MoonGift, RENOWNS, Werewolf, WerewolfExt},
	},
	systema::prelude::Actor,
	traits::Trait,
};
use strum::VariantArray;

#[derive(Debug, uniffi::Error)]
pub enum FfiError {
	Message { message: String },
}

impl Display for FfiError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Message { message } => formatter.write_str(message),
		}
	}
}

fn error(message: impl Display) -> FfiError {
	FfiError::Message {
		message: message.to_string(),
	}
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
	mutex
		.lock()
		.expect("cofd Foundry object mutex was poisoned")
}

fn whole_number(value: f64, min: f64, max: f64, name: &str) -> Result<f64, FfiError> {
	if value.is_finite() && value.fract() == 0.0 && (min..=max).contains(&value) {
		Ok(value)
	} else {
		Err(error(format!(
			"{name} must be an integer between {min} and {max}"
		)))
	}
}

fn as_u8(value: f64, name: &str) -> Result<u8, FfiError> {
	Ok(whole_number(value, 0.0, f64::from(u8::MAX), name)? as u8)
}

fn as_i8(value: f64, name: &str) -> Result<i8, FfiError> {
	Ok(whole_number(value, f64::from(i8::MIN), f64::from(i8::MAX), name)? as i8)
}

fn normalized(value: &str) -> String {
	value
		.chars()
		.filter(|character| character.is_ascii_alphanumeric())
		.flat_map(char::to_lowercase)
		.collect()
}

fn parse_template(value: &str) -> Result<Template, FfiError> {
	Template::from_str(value).map_err(|_| error(format!("unknown CofD template: {value}")))
}

fn parse_skill(value: &str) -> Result<Skill, FfiError> {
	Skill::from_str(value).map_err(|_| error(format!("unknown CofD skill: {value}")))
}

fn parse_trait(value: &str) -> Result<Trait, FfiError> {
	if normalized(value) == "size" {
		return Ok(Trait::Size);
	}

	let parsed =
		SchemaTrait::from_str(value).map_err(|_| error(format!("unknown CofD trait: {value}")))?;
	Ok(match parsed {
		SchemaTrait::Attribute(attribute) => Trait::Attribute(attribute),
		SchemaTrait::Skill(skill) => Trait::Skill(skill),
		SchemaTrait::DerivedTrait(derived) => Trait::DerivedTrait(derived),
		SchemaTrait::SupernaturalTolerance(tolerance) => Trait::SupernaturalTolerance(tolerance),
		SchemaTrait::Size => Trait::Size,
	})
}

fn validate_trait_value(trait_key: &Trait, value: u8) -> Result<(), FfiError> {
	let valid = match trait_key {
		Trait::Attribute(_) | Trait::Skill(_) => value <= 5,
		Trait::Size => (1..=20).contains(&value),
		Trait::SupernaturalTolerance(_) => value <= 10,
		Trait::Ability(_) | Trait::DerivedTrait(_) => value <= 5,
	};
	if valid {
		Ok(())
	} else {
		Err(error(format!("CofD trait value is out of range: {value}")))
	}
}

fn parse_wound(value: &str) -> Result<Wound, FfiError> {
	match normalized(value).as_str() {
		"none" => Ok(Wound::None),
		"bashing" => Ok(Wound::Bashing),
		"lethal" => Ok(Wound::Lethal),
		"aggravated" => Ok(Wound::Aggravated),
		_ => Err(error(format!("unknown wound type: {value}"))),
	}
}

fn parse_info(value: &str) -> Result<Option<InfoTrait>, FfiError> {
	Ok(match normalized(value).as_str() {
		"name" => Some(InfoTrait::Name),
		"age" => Some(InfoTrait::Age),
		"player" => Some(InfoTrait::Player),
		"virtue" | "virtueanchor" => Some(InfoTrait::VirtueAnchor),
		"vice" | "viceanchor" => Some(InfoTrait::ViceAnchor),
		"concept" => Some(InfoTrait::Concept),
		"chronicle" => Some(InfoTrait::Chronicle),
		"faction" => Some(InfoTrait::Faction),
		"groupname" => Some(InfoTrait::GroupName),
		"dateofbirth" => Some(InfoTrait::DateOfBirth),
		"hair" => Some(InfoTrait::Hair),
		"eyes" => Some(InfoTrait::Eyes),
		"race" => Some(InfoTrait::Race),
		"nationality" => Some(InfoTrait::Nationality),
		"height" => Some(InfoTrait::Height),
		"weight" => Some(InfoTrait::Weight),
		"sex" => Some(InfoTrait::Sex),
		"other" => None,
		_ => return Err(error(format!("unknown character info field: {value}"))),
	})
}

fn parse_form(value: &str) -> Result<Form, FfiError> {
	match normalized(value).as_str() {
		"hishu" => Ok(Form::Hishu),
		"dalu" => Ok(Form::Dalu),
		"gauru" => Ok(Form::Gauru),
		"urshul" => Ok(Form::Urshul),
		"urhan" => Ok(Form::Urhan),
		_ => Err(error(format!("unknown werewolf form: {value}"))),
	}
}

fn form_name(form: Form) -> &'static str {
	match form {
		Form::Hishu => "Hishu",
		Form::Dalu => "Dalu",
		Form::Gauru => "Gauru",
		Form::Urshul => "Urshul",
		Form::Urhan => "Urhan",
	}
}

trait CharacterData {
	fn template(&self) -> Template;
	fn info(&self) -> &CharacterInfo;
	fn info_mut(&mut self) -> &mut CharacterInfo;
	fn health(&self) -> &Damage;
	fn health_mut(&mut self) -> &mut Damage;
	fn specialties(&self) -> &std::collections::HashMap<Skill, Vec<String>>;
	fn specialties_mut(&mut self) -> &mut std::collections::HashMap<Skill, Vec<String>>;
	fn touchstones(&self) -> &Vec<String>;
	fn touchstones_mut(&mut self) -> &mut Vec<String>;
	fn conditions(&self) -> &Vec<String>;
	fn conditions_mut(&mut self) -> &mut Vec<String>;
	fn aspirations(&self) -> &Vec<String>;
	fn aspirations_mut(&mut self) -> &mut Vec<String>;
	fn weapons(&self) -> &Vec<Weapon>;
	fn weapons_mut(&mut self) -> &mut Vec<Weapon>;
	fn wound_penalty(&self) -> u8;
	fn max_fuel(&self) -> Option<u8>;
}

impl<S: SplatTrait> CharacterData for Character<S> {
	fn template(&self) -> Template {
		self.splat.template()
	}

	fn info(&self) -> &CharacterInfo {
		&self.info
	}

	fn info_mut(&mut self) -> &mut CharacterInfo {
		&mut self.info
	}

	fn health(&self) -> &Damage {
		Character::health(self)
	}

	fn health_mut(&mut self) -> &mut Damage {
		Character::health_mut(self)
	}

	fn specialties(&self) -> &std::collections::HashMap<Skill, Vec<String>> {
		&self.specialties
	}

	fn specialties_mut(&mut self) -> &mut std::collections::HashMap<Skill, Vec<String>> {
		&mut self.specialties
	}

	fn touchstones(&self) -> &Vec<String> {
		&self.touchstones
	}

	fn touchstones_mut(&mut self) -> &mut Vec<String> {
		&mut self.touchstones
	}

	fn conditions(&self) -> &Vec<String> {
		&self.conditions
	}

	fn conditions_mut(&mut self) -> &mut Vec<String> {
		&mut self.conditions
	}

	fn aspirations(&self) -> &Vec<String> {
		&self.aspirations
	}

	fn aspirations_mut(&mut self) -> &mut Vec<String> {
		&mut self.aspirations
	}

	fn weapons(&self) -> &Vec<Weapon> {
		&self.weapons
	}

	fn weapons_mut(&mut self) -> &mut Vec<Weapon> {
		&mut self.weapons
	}

	fn wound_penalty(&self) -> u8 {
		Character::wound_penalty(self)
	}

	fn max_fuel(&self) -> Option<u8> {
		Character::max_fuel(self)
	}
}

fn data(character: &SplatCharacter) -> &dyn CharacterData {
	match character {
		SplatCharacter::Mortal(character) => character,
		SplatCharacter::Vampire(character) => character,
		SplatCharacter::Werewolf(character) => character,
		SplatCharacter::Mage(character) => character,
		SplatCharacter::Changeling(character) => character,
		SplatCharacter::Bound(character) => character,
	}
}

fn data_mut(character: &mut SplatCharacter) -> &mut dyn CharacterData {
	match character {
		SplatCharacter::Mortal(character) => character,
		SplatCharacter::Vampire(character) => character,
		SplatCharacter::Werewolf(character) => character,
		SplatCharacter::Mage(character) => character,
		SplatCharacter::Changeling(character) => character,
		SplatCharacter::Bound(character) => character,
	}
}

fn supported_character(template: Template) -> Result<SplatCharacter, FfiError> {
	match template {
		Template::Mortal => Ok(SplatCharacter::Mortal(Character::<Mortal>::new(Mortal))),
		Template::Werewolf => {
			let mut character = Character::<Werewolf>::new(Werewolf::default());
			character.set_form(Form::default());
			Ok(SplatCharacter::Werewolf(character))
		}
		_ => Err(error(format!(
			"CofD template {template} is not supported by the current engine"
		))),
	}
}

fn ability_values(character: &SplatCharacter) -> Vec<Ability> {
	match character {
		SplatCharacter::Mortal(character) => character.splat.all_abilities().unwrap_or_default(),
		SplatCharacter::Vampire(character) => character.splat.all_abilities().unwrap_or_default(),
		SplatCharacter::Werewolf(_) => RENOWNS
			.iter()
			.copied()
			.map(Ability::Renown)
			.chain(MoonGift::all().into_iter().map(Ability::MoonGift))
			.collect(),
		SplatCharacter::Mage(character) => character.splat.all_abilities().unwrap_or_default(),
		SplatCharacter::Changeling(character) => {
			character.splat.all_abilities().unwrap_or_default()
		}
		SplatCharacter::Bound(character) => character.splat.all_abilities().unwrap_or_default(),
	}
}

#[derive(uniffi::Object)]
pub struct WeaponHandle {
	inner: Mutex<Weapon>,
}

#[uniffi::export]
impl WeaponHandle {
	#[uniffi::constructor]
	pub fn new(
		name: String,
		dice_pool: String,
		damage: String,
		range: String,
		initiative: f64,
		size: f64,
	) -> Result<Arc<Self>, FfiError> {
		Ok(Arc::new(Self {
			inner: Mutex::new(Weapon {
				name,
				dice_pool,
				damage,
				range,
				initiative: as_i8(initiative, "initiative")?,
				size: as_u8(size, "size")?,
			}),
		}))
	}

	pub fn name(&self) -> String {
		lock(&self.inner).name.clone()
	}

	pub fn set_name(&self, value: String) {
		lock(&self.inner).name = value;
	}

	pub fn dice_pool(&self) -> String {
		lock(&self.inner).dice_pool.clone()
	}

	pub fn set_dice_pool(&self, value: String) {
		lock(&self.inner).dice_pool = value;
	}

	pub fn damage(&self) -> String {
		lock(&self.inner).damage.clone()
	}

	pub fn set_damage(&self, value: String) {
		lock(&self.inner).damage = value;
	}

	pub fn range(&self) -> String {
		lock(&self.inner).range.clone()
	}

	pub fn set_range(&self, value: String) {
		lock(&self.inner).range = value;
	}

	pub fn initiative(&self) -> i8 {
		lock(&self.inner).initiative
	}

	pub fn set_initiative(&self, value: f64) -> Result<(), FfiError> {
		lock(&self.inner).initiative = as_i8(value, "initiative")?;
		Ok(())
	}

	pub fn size(&self) -> u8 {
		lock(&self.inner).size
	}

	pub fn set_size(&self, value: f64) -> Result<(), FfiError> {
		lock(&self.inner).size = as_u8(value, "size")?;
		Ok(())
	}
}

#[derive(uniffi::Object)]
pub struct CharacterHandle {
	inner: Mutex<SplatCharacter>,
}

#[uniffi::export]
impl CharacterHandle {
	#[uniffi::constructor]
	pub fn new(template: &str) -> Result<Arc<Self>, FfiError> {
		Ok(Arc::new(Self {
			inner: Mutex::new(supported_character(parse_template(template)?)?),
		}))
	}

	pub fn template_name(&self) -> String {
		data(&lock(&self.inner)).template().to_string()
	}

	pub fn get_trait(&self, key: &str) -> Result<Option<u8>, FfiError> {
		let trait_key = parse_trait(key)?;
		Ok(lock(&self.inner).attributes().value(&trait_key))
	}

	pub fn get_base_trait(&self, key: &str) -> Result<Option<u8>, FfiError> {
		let trait_key = parse_trait(key)?;
		Ok(lock(&self.inner).attributes().base_value(&trait_key))
	}

	pub fn set_trait(&self, key: &str, value: f64) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let trait_key = parse_trait(key)?;
		let value = as_u8(value, "trait value")?;
		if matches!(trait_key, Trait::DerivedTrait(_)) {
			return Err(error(format!("derived trait {key} is read-only")));
		}
		if inner.attributes().base_value(&trait_key).is_none() {
			return Err(error(format!(
				"trait {key} is not available for {}",
				data(&inner).template()
			)));
		}
		validate_trait_value(&trait_key, value)?;
		inner.attributes_mut().set_raw_value(&trait_key, value);
		Ok(())
	}

	pub fn list_abilities(&self) -> Vec<String> {
		ability_values(&lock(&self.inner))
			.into_iter()
			.map(|ability| ability.to_string())
			.collect()
	}

	pub fn get_ability(&self, name: &str) -> Result<Option<u8>, FfiError> {
		let inner = lock(&self.inner);
		let ability = ability_values(&inner)
			.into_iter()
			.find(|ability| ability.to_string().eq_ignore_ascii_case(name))
			.ok_or_else(|| error(format!("unknown ability: {name}")))?;
		Ok(inner.attributes().value(&Trait::Ability(ability)))
	}

	pub fn set_ability(&self, name: &str, value: f64) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let ability = ability_values(&inner)
			.into_iter()
			.find(|ability| ability.to_string().eq_ignore_ascii_case(name))
			.ok_or_else(|| error(format!("unknown ability: {name}")))?;
		if matches!(ability, Ability::MoonGift(_)) {
			return Err(error(format!("ability {name} is derived and read-only")));
		}
		let value = as_u8(value, "ability value")?;
		validate_trait_value(&Trait::Ability(ability.clone()), value)?;
		inner
			.attributes_mut()
			.set_raw_value(&Trait::Ability(ability), value);
		Ok(())
	}

	pub fn damage_for(&self, wound: &str) -> Result<u8, FfiError> {
		Ok(data(&lock(&self.inner)).health().get(parse_wound(wound)?))
	}

	pub fn damage_total(&self) -> u8 {
		data(&lock(&self.inner)).health().total()
	}

	pub fn apply_damage(&self, wound: &str, points: f64) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let wound = parse_wound(wound)?;
		let points = as_u8(points, "damage points")?;
		if matches!(wound, Wound::None) {
			return Err(error("damage must be bashing, lethal, or aggravated"));
		}
		let health = inner
			.attributes()
			.value(&Trait::DerivedTrait(DerivedTrait::Health))
			.ok_or_else(|| error("character has no derived Health trait"))?;
		if !data_mut(&mut inner)
			.health_mut()
			.apply(wound, points, health)
		{
			return Err(error("damage exceeds the character's Health track"));
		}
		Ok(())
	}

	pub fn poke_damage(&self, index: f64) -> Result<String, FfiError> {
		let mut inner = lock(&self.inner);
		let index = as_u8(index, "damage index")?;
		if index >= data(&inner).health().total() {
			return Err(error(format!("damage index out of range: {index}")));
		}
		let wound = data_mut(&mut inner).health_mut().poke_index(index);
		Ok(match wound {
			Wound::None => "none",
			Wound::Bashing => "bashing",
			Wound::Lethal => "lethal",
			Wound::Aggravated => "aggravated",
		}
		.to_owned())
	}

	pub fn clear_damage(&self, wound: &str, points: f64) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let wound = parse_wound(wound)?;
		let points = as_u8(points, "damage points")?;
		if matches!(wound, Wound::None) {
			return Err(error("damage must be bashing, lethal, or aggravated"));
		}
		for _ in 0..points {
			let Some(value) = data_mut(&mut inner).health_mut().get_mut(wound) else {
				break;
			};
			if *value == 0 {
				break;
			}
			*value -= 1;
		}
		Ok(())
	}

	pub fn heal_damage(&self, wound: &str, points: f64) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let wound = parse_wound(wound)?;
		let points = as_u8(points, "damage points")?;
		if matches!(wound, Wound::None) {
			return Err(error("damage must be bashing, lethal, or aggravated"));
		}
		data_mut(&mut inner).health_mut().heal(wound, points);
		Ok(())
	}

	pub fn wound_penalty(&self) -> u8 {
		data(&lock(&self.inner)).wound_penalty()
	}

	pub fn max_fuel(&self) -> Option<u8> {
		data(&lock(&self.inner)).max_fuel()
	}

	pub fn get_info(&self, key: &str) -> Result<String, FfiError> {
		let inner = lock(&self.inner);
		match parse_info(key)? {
			Some(info) => Ok(data(&inner).info().get(info).clone()),
			None => Ok(data(&inner).info().other.clone()),
		}
	}

	pub fn set_info(&self, key: &str, value: String) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		match parse_info(key)? {
			Some(info) => *data_mut(&mut inner).info_mut().get_mut(info) = value,
			None => data_mut(&mut inner).info_mut().other = value,
		}
		Ok(())
	}

	pub fn specialties_for(&self, skill: &str) -> Result<Vec<String>, FfiError> {
		let skill = parse_skill(skill)?;
		Ok(data(&lock(&self.inner))
			.specialties()
			.get(&skill)
			.cloned()
			.unwrap_or_default())
	}

	pub fn set_specialties(&self, skill: &str, values: Vec<String>) -> Result<(), FfiError> {
		let skill = parse_skill(skill)?;
		data_mut(&mut lock(&self.inner))
			.specialties_mut()
			.insert(skill, values);
		Ok(())
	}

	pub fn touchstones(&self) -> Vec<String> {
		data(&lock(&self.inner)).touchstones().clone()
	}

	pub fn set_touchstones(&self, values: Vec<String>) {
		let mut inner = lock(&self.inner);
		*data_mut(&mut inner).touchstones_mut() = values;
	}

	pub fn conditions(&self) -> Vec<String> {
		data(&lock(&self.inner)).conditions().clone()
	}

	pub fn set_conditions(&self, values: Vec<String>) {
		let mut inner = lock(&self.inner);
		*data_mut(&mut inner).conditions_mut() = values;
	}

	pub fn aspirations(&self) -> Vec<String> {
		data(&lock(&self.inner)).aspirations().clone()
	}

	pub fn set_aspirations(&self, values: Vec<String>) {
		let mut inner = lock(&self.inner);
		*data_mut(&mut inner).aspirations_mut() = values;
	}

	pub fn weapon_count(&self) -> u32 {
		data(&lock(&self.inner)).weapons().len() as u32
	}

	pub fn weapon_at(&self, index: u32) -> Result<Arc<WeaponHandle>, FfiError> {
		data(&lock(&self.inner))
			.weapons()
			.get(index as usize)
			.cloned()
			.map(|inner| {
				Arc::new(WeaponHandle {
					inner: Mutex::new(inner),
				})
			})
			.ok_or_else(|| error(format!("weapon index out of range: {index}")))
	}

	pub fn add_weapon(&self, weapon: Arc<WeaponHandle>) {
		let mut inner = lock(&self.inner);
		data_mut(&mut inner)
			.weapons_mut()
			.push(lock(&weapon.inner).clone());
	}

	pub fn set_weapon(&self, index: u32, weapon: Arc<WeaponHandle>) -> Result<(), FfiError> {
		let mut inner = lock(&self.inner);
		let Some(slot) = data_mut(&mut inner).weapons_mut().get_mut(index as usize) else {
			return Err(error(format!("weapon index out of range: {index}")));
		};
		*slot = lock(&weapon.inner).clone();
		Ok(())
	}

	pub fn remove_weapon(&self, index: u32) -> Result<Arc<WeaponHandle>, FfiError> {
		let mut inner = lock(&self.inner);
		if index as usize >= data(&inner).weapons().len() {
			return Err(error(format!("weapon index out of range: {index}")));
		}
		Ok(Arc::new(WeaponHandle {
			inner: Mutex::new(data_mut(&mut inner).weapons_mut().remove(index as usize)),
		}))
	}

	pub fn form_name(&self) -> Result<String, FfiError> {
		match &*lock(&self.inner) {
			SplatCharacter::Werewolf(character) => Ok(form_name(*character.form()).to_string()),
			_ => Err(error("form is only available for werewolves")),
		}
	}

	pub fn set_form(&self, form: &str) -> Result<(), FfiError> {
		let form = parse_form(form)?;
		match &mut *lock(&self.inner) {
			SplatCharacter::Werewolf(character) => {
				character.set_form(form);
				Ok(())
			}
			_ => Err(error("form is only available for werewolves")),
		}
	}
}

#[uniffi::export]
pub fn list_templates() -> Vec<String> {
	Template::VARIANTS.iter().map(ToString::to_string).collect()
}

#[uniffi::export]
pub fn list_supported_templates() -> Vec<String> {
	[Template::Mortal, Template::Werewolf]
		.into_iter()
		.map(|template| template.to_string())
		.collect()
}

#[uniffi::export]
pub fn list_attributes() -> Vec<String> {
	Attribute::VARIANTS
		.iter()
		.map(ToString::to_string)
		.collect()
}

#[uniffi::export]
pub fn list_skills() -> Vec<String> {
	Skill::VARIANTS.iter().map(ToString::to_string).collect()
}

#[uniffi::export]
pub fn list_derived_traits() -> Vec<String> {
	DerivedTrait::VARIANTS
		.iter()
		.map(ToString::to_string)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mortal_traits_are_live_engine_values() {
		let character = CharacterHandle::new("Mortal").unwrap();
		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		character.set_trait("Resolve", 3.0).unwrap();
		character.set_trait("Composure", 4.0).unwrap();

		assert_eq!(character.get_trait("Health").unwrap(), Some(8));
		assert_eq!(character.get_trait("Willpower").unwrap(), Some(7));
	}

	#[test]
	fn damage_uses_engine_upgrade_rules() {
		let character = CharacterHandle::new("Mortal").unwrap();
		character.apply_damage("bashing", 2.0).unwrap();
		character.apply_damage("lethal", 1.0).unwrap();

		assert_eq!(character.damage_for("bashing").unwrap(), 2);
		assert_eq!(character.damage_for("lethal").unwrap(), 1);
		assert_eq!(character.damage_total(), 3);
		assert_eq!(character.poke_damage(0.0).unwrap(), "aggravated");
		assert_eq!(character.damage_for("aggravated").unwrap(), 1);
	}

	#[test]
	fn unsupported_templates_are_errors() {
		assert!(CharacterHandle::new("Vampire").is_err());
		assert!(CharacterHandle::new("not-a-template").is_err());
		assert!(list_templates().len() > list_supported_templates().len());
	}

	#[test]
	fn validation_rejects_unsafe_values_and_missing_traits() {
		let character = CharacterHandle::new("Mortal").unwrap();
		assert!(character.set_trait("Stamina", 256.0).is_err());
		assert!(character.set_trait("Stamina", -1.0).is_err());
		assert!(character.set_trait("Size", 0.0).is_err());
		assert!(character.set_trait("Blood Potency", 1.0).is_err());
		assert!(character.set_trait("Health", 1.0).is_err());
	}

	#[test]
	fn info_abilities_and_damage_are_exposed() {
		let character = CharacterHandle::new("Werewolf").unwrap();
		character
			.set_info("Faction", "Forsaken".to_owned())
			.unwrap();
		character
			.set_info("GroupName", "Blood Talons".to_owned())
			.unwrap();
		assert_eq!(character.get_info("faction").unwrap(), "Forsaken");
		assert_eq!(character.get_info("group-name").unwrap(), "Blood Talons");
		assert!(character.list_abilities().iter().any(|name| name == "Full"));
		assert!(character.set_ability("Full", 1.0).is_err());

		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		character.apply_damage("bashing", 2.0).unwrap();
		character.apply_damage("lethal", 1.0).unwrap();
		character.heal_damage("lethal", 1.0).unwrap();
		assert_eq!(character.damage_for("bashing").unwrap(), 3);
		assert_eq!(character.damage_for("lethal").unwrap(), 0);
		assert_eq!(character.damage_total(), 3);
	}

	#[test]
	fn wound_penalty_uses_the_health_track_position() {
		let character = CharacterHandle::new("Mortal").unwrap();
		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		assert_eq!(character.wound_penalty(), 0);
		character.apply_damage("bashing", 6.0).unwrap();
		assert_eq!(character.wound_penalty(), 1);
		character.apply_damage("bashing", 1.0).unwrap();
		assert_eq!(character.wound_penalty(), 2);
		character.apply_damage("bashing", 1.0).unwrap();
		assert_eq!(character.wound_penalty(), 3);
	}

	#[test]
	fn full_bashing_tracks_upgrade_on_further_bashing() {
		let character = CharacterHandle::new("Mortal").unwrap();
		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		character.apply_damage("bashing", 8.0).unwrap();
		character.apply_damage("bashing", 1.0).unwrap();
		assert_eq!(character.damage_for("bashing").unwrap(), 7);
		assert_eq!(character.damage_for("lethal").unwrap(), 1);
		assert_eq!(character.damage_total(), 8);
	}

	#[test]
	fn full_lethal_tracks_upgrade_on_further_damage() {
		let character = CharacterHandle::new("Mortal").unwrap();
		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		character.apply_damage("lethal", 8.0).unwrap();
		character.apply_damage("bashing", 1.0).unwrap();
		assert_eq!(character.damage_for("lethal").unwrap(), 7);
		assert_eq!(character.damage_for("aggravated").unwrap(), 1);
		assert_eq!(character.damage_total(), 8);
	}

	#[test]
	fn weapon_handles_are_borrowed_when_added() {
		let character = CharacterHandle::new("Mortal").unwrap();
		let weapon = WeaponHandle::new(
			"Knife".to_owned(),
			"Dexterity + Weaponry".to_owned(),
			"1L".to_owned(),
			"Short".to_owned(),
			-1.0,
			1.0,
		)
		.unwrap();
		character.add_weapon(Arc::clone(&weapon));
		assert_eq!(weapon.name(), "Knife");
		assert_eq!(character.weapon_count(), 1);
		assert_eq!(character.weapon_at(0).unwrap().damage(), "1L");
	}

	#[test]
	fn werewolf_forms_update_derived_values() {
		let character = CharacterHandle::new("Werewolf").unwrap();
		character.set_trait("Stamina", 3.0).unwrap();
		character.set_trait("Size", 5.0).unwrap();
		assert_eq!(character.get_trait("Health").unwrap(), Some(8));

		character.set_form("Dalu").unwrap();
		assert_eq!(character.get_trait("Health").unwrap(), Some(10));
	}
}
