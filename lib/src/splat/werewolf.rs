use std::collections::HashMap;

pub use cofd_schema::template::werewolf as schema;
use cofd_schema::{template::SupernaturalTolerance, traits::DerivedTrait};
use cofd_util::VariantName;
use derive_more::{From, TryInto};
use schema::{Form, HuntersAspect::Monstrous, Lodge, Renown};
use serde::{Deserialize, Serialize};
use systema::prelude::{Actor, AttributeModifier, Value};

use super::{Merit, SplatTrait, XSplat, YSplat, ZSplat};
use crate::{
	COp, CofDSystem, Modifier,
	ability::{Ability, AbilityTrait, CModifier},
	prelude::*,
	traits::{NameKey, Trait},
};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Werewolf {
	pub auspice: Option<Auspice>,
	pub tribe: Option<Tribe>,
	pub lodge: Option<Lodge>,

	pub hunters_aspect: Option<HuntersAspect>,
	skill_bonus: Option<Skill>,
	pub form: Form,
	// pub moon_gifts: BTreeMap<MoonGift, AbilityVal>,
	pub triggers: KuruthTriggers,
	pub moon_gifts: HashMap<MoonGift, u16>,
	pub shadow_gifts: Vec<ShadowGift>,
	pub wolf_gifts: Vec<WolfGift>,
	pub rites: Vec<Rite>,
}

pub trait WerewolfExt: Actor<System = CofDSystem> {
	fn form(&self) -> &Form;

	fn set_form(&mut self, form: Form);
}

impl WerewolfExt for Character<Werewolf> {
	fn form(&self) -> &Form {
		&self.splat.form
	}

	fn set_form(&mut self, form: Form) {
		let old_form = self.splat.form;

		self.attributes_mut()
			.remove_modifiers(&Modifier::Form(old_form));

		self.splat.form = form;
		let mods: &[(Trait, AttributeModifier<Trait, u8, COp>)] = get_modifiers(form);
		for (t, m) in mods {
			self.attributes_mut()
				.add_modifier(t, Modifier::Form(form), m.clone());
		}
	}
}

impl Werewolf {
	#[must_use]
	pub fn new() -> Self {
		Werewolf::default()
	}

	#[must_use]
	pub fn with_auspice<I: Into<Auspice>>(mut self, auspice: I) -> Self {
		self.auspice = Some(auspice.into());
		self
	}

	#[must_use]
	pub fn with_tribe<I: Into<Tribe>>(mut self, tribe: I) -> Self {
		self.tribe = Some(tribe.into());
		self
	}

	#[must_use]
	pub fn with_lodge(mut self, lodge: Lodge) -> Self {
		self.lodge = Some(lodge);
		self
	}

	#[must_use]
	pub fn with_skill_bonus(mut self, skill: Skill) -> Self {
		self.set_skill_bonus(skill);
		self
	}

	#[must_use]
	pub fn skill_bonus(&self) -> Option<&Skill> {
		if self.auspice.is_some() {
			self.skill_bonus.as_ref()
		} else {
			None
		}
	}

	pub fn set_skill_bonus(&mut self, skill: Skill) {
		if let Some(auspice) = &self.auspice {
			if auspice.skills().contains(&skill) {
				self.skill_bonus = Some(skill);
			}
		}
	}
}

impl SplatTrait for Werewolf {
	fn template(&self) -> Template {
		Template::Werewolf
	}

	fn set_xsplat(&mut self, splat: Option<XSplat>) {
		match splat {
			Some(XSplat::Auspice(auspice)) => self.auspice = Some(auspice),
			_ => self.auspice = None,
		}
	}

	fn set_ysplat(&mut self, splat: Option<YSplat>) {
		match splat {
			Some(YSplat::Tribe(tribe)) => self.tribe = Some(tribe),
			_ => self.tribe = None,
		}
	}

	fn set_zsplat(&mut self, splat: Option<ZSplat>) {
		match splat {
			Some(ZSplat::Lodge(lodge)) => self.lodge = Some(lodge),
			_ => self.lodge = None,
		}
	}

	fn xsplat(&self) -> Option<XSplat> {
		self.auspice.clone().map(Into::into)
	}

	fn ysplat(&self) -> Option<YSplat> {
		self.tribe.clone().map(Into::into)
	}

	fn zsplat(&self) -> Option<ZSplat> {
		self.lodge.clone().map(Into::into)
	}

	fn xsplats(&self) -> Vec<XSplat> {
		// Auspice::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		// Tribe::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		// Lodge::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(
			Auspice::Custom(CustomAuspice {
				name,
				skills: [Skill::Academics, Skill::AnimalKen, Skill::Athletics],
				renown: Renown::Cunning,
				moon_gift: MoonGift::Custom(String::from("Custom")),
				affinity_gifts: Box::new([ShadowGift::Death, ShadowGift::Dominance]),
				hunters_aspect: HuntersAspect::Base(Monstrous),
			})
			.into(),
		)
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(
			Tribe::Forsaken(ForsakenTribe::Custom {
				name,
				renown: Renown::Cunning,
				affinity_gifts: Box::new([
					ShadowGift::Death,
					ShadowGift::Dominance,
					ShadowGift::Elementals,
				]),
			})
			.into(),
		)
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(Lodge { name }.into())
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		Some(Renown::all().into_iter().map(Into::into).collect())
	}

	fn merits(&self) -> Vec<Merit> {
		WerewolfMerit::all().map(Into::into).to_vec()
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KuruthTriggerSet {
	pub passive: String,
	pub common: String,
	pub specific: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, AllVariants)]
pub enum KuruthTriggers {
	Blood,
	Moon,
	TheOther,
	Pack,
	Territory,
	Wound,
	Custom(KuruthTriggerSet),
}

impl Default for KuruthTriggers {
	fn default() -> Self {
		Self::Custom(KuruthTriggerSet::default())
	}
}

impl KuruthTriggers {
	#[must_use]
	pub fn name(&self) -> Option<&str> {
		match self {
			KuruthTriggers::Blood => Some("blood-trigger"),
			KuruthTriggers::Moon => Some("moon-trigger"),
			KuruthTriggers::TheOther => Some("the-other-trigger"),
			KuruthTriggers::Pack => Some("pack-trigger"),
			KuruthTriggers::Territory => Some("territory-trigger"),
			KuruthTriggers::Wound => Some("wound-trigger"),
			KuruthTriggers::Custom(_) => None,
		}
	}

	#[must_use]
	pub fn get_triggers(&self) -> KuruthTriggerSet {
		match self {
			KuruthTriggers::Blood => KuruthTriggerSet {
				passive: "Smelling human blood.".to_owned(),
				common: "Tasting human blood.".to_owned(),
				specific: "Swallowing human blood.".to_owned(),
			},
			KuruthTriggers::Moon => KuruthTriggerSet {
				passive: "Your auspice moon is in the sky.".to_owned(),
				common: "You witness your auspice moon in the sky.".to_owned(),
				specific: "Hear a wolf or werewolf howl when your auspice moon is in the sky."
					.to_owned(),
			},
			KuruthTriggers::TheOther => KuruthTriggerSet {
				passive: "You come within 10 yards of a supernatural creature.".to_owned(),
				common: "You witness a supernatural creature doing something obviously inhuman."
					.to_owned(),
				specific: "You are the target of a supernatural power.".to_owned(),
			},
			KuruthTriggers::Pack => KuruthTriggerSet {
				passive: "A pack member takes lethal damage.".to_owned(),
				common: "Seeing someone attack a pack member.".to_owned(),
				specific: "You cause lethal damage to a pack member.".to_owned(),
			},
			KuruthTriggers::Territory => KuruthTriggerSet {
				passive: "A werewolf you don't know enters your territory without permission."
					.to_owned(),
				common: "You see a werewolf you don't know in your territory.".to_owned(),
				specific:
					"A werewolf you don't know challenges your pack's ability to do its duty."
						.to_owned(),
			},
			KuruthTriggers::Wound => KuruthTriggerSet {
				passive: "Being in the area of a Wound.".to_owned(),
				common: "Interacting with a Wound-born spirit.".to_owned(),
				specific: "Being attacked by a Wound-born spirit.".to_owned(),
			},
			KuruthTriggers::Custom(set) => set.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, From, TryInto)]
pub enum HuntersAspect {
	Base(schema::HuntersAspect),
	Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CustomAuspice {
	pub name: String,
	pub skills: [Skill; 3],
	pub renown: Renown,
	pub moon_gift: MoonGift,
	pub affinity_gifts: Box<[ShadowGift; 2]>,
	pub hunters_aspect: HuntersAspect,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum Auspice {
	Base(schema::Auspice),
	Custom(CustomAuspice),
}

impl Auspice {
	#[must_use]
	pub fn skills(&self) -> [Skill; 3] {
		match self {
			Auspice::Base(b) => b.skills(),
			Auspice::Custom(CustomAuspice { skills, .. }) => *skills,
		}
	}

	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			Auspice::Base(b) => b.renown(),
			Auspice::Custom(CustomAuspice { renown, .. }) => *renown,
		}
	}

	#[must_use]
	pub fn gifts(&self) -> [ShadowGift; 2] {
		match self {
			Self::Base(b) => match b {
				schema::Auspice::Cahalith => [ShadowGift::Inspiration, ShadowGift::Knowledge],
				schema::Auspice::Elodoth => [ShadowGift::Insight, ShadowGift::Warding],
				schema::Auspice::Irraka => [ShadowGift::Evasion, ShadowGift::Stealth],
				schema::Auspice::Ithaeur => [ShadowGift::Elementals, ShadowGift::Shaping],
				schema::Auspice::Rahu => [ShadowGift::Dominance, ShadowGift::Strength],
			},
			Self::Custom(CustomAuspice { affinity_gifts, .. }) => affinity_gifts.as_ref().clone(),
		}
	}

	#[must_use]
	pub fn moon_gift(&self) -> MoonGift {
		match self {
			Self::Base(b) => match b {
				schema::Auspice::Cahalith => MoonGift::Gibbous,
				schema::Auspice::Elodoth => MoonGift::Half,
				schema::Auspice::Irraka => MoonGift::New,
				schema::Auspice::Ithaeur => MoonGift::Crescent,
				schema::Auspice::Rahu => MoonGift::Full,
			},
			Self::Custom(CustomAuspice { moon_gift, .. }) => moon_gift.clone(),
		}
	}

	#[must_use]
	pub fn hunters_aspect(&self) -> HuntersAspect {
		match self {
			Auspice::Base(b) => HuntersAspect::from(b.hunters_aspect()),
			Auspice::Custom(CustomAuspice { hunters_aspect, .. }) => hunters_aspect.clone(),
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum ForsakenTribe {
	Base(schema::ForsakenTribe),
	Custom {
		name: String,
		renown: Renown,
		affinity_gifts: Box<[ShadowGift; 3]>,
	},
}

impl ForsakenTribe {
	#[must_use]
	pub fn gifts(&self) -> [ShadowGift; 3] {
		match self {
			Self::Base(t) => match t {
				schema::ForsakenTribe::BloodTalons => [
					ShadowGift::Inspiration,
					ShadowGift::Rage,
					ShadowGift::Strength,
				],
				schema::ForsakenTribe::BoneShadows => [
					ShadowGift::Death,
					ShadowGift::Elementals,
					ShadowGift::Insight,
				],
				schema::ForsakenTribe::HuntersInDarkness => {
					[ShadowGift::Nature, ShadowGift::Stealth, ShadowGift::Warding]
				}
				schema::ForsakenTribe::IronMasters => [
					ShadowGift::Knowledge,
					ShadowGift::Shaping,
					ShadowGift::Technology,
				],
				schema::ForsakenTribe::StormLords => [
					ShadowGift::Evasion,
					ShadowGift::Dominance,
					ShadowGift::Weather,
				],
			},
			Self::Custom { affinity_gifts, .. } => affinity_gifts.as_ref().clone(),
		}
	}

	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			Self::Base(b) => b.renown(),
			Self::Custom { renown, .. } => *renown,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum PureTribe {
	Base(schema::PureTribe),
	Custom {
		name: String,
		renown: Renown,
		secondary_renown: [Renown; 2],
		skills: [Skill; 3],
		hunters_aspects: [HuntersAspect; 2],
		affinity_gifts: Box<[ShadowGift; 4]>,
	},
}

impl PureTribe {
	#[must_use]
	pub fn renown(&self) -> Renown {
		match self {
			Self::Base(b) => b.renown(),
			Self::Custom { renown, .. } => *renown,
		}
	}

	#[must_use]
	pub fn secondary_renown(&self) -> [Renown; 2] {
		match self {
			Self::Base(b) => b.secondary_renown(),
			Self::Custom {
				secondary_renown, ..
			} => *secondary_renown,
		}
	}

	#[must_use]
	pub fn skills(&self) -> [Skill; 3] {
		match self {
			Self::Base(b) => b.skills(),
			Self::Custom { skills, .. } => *skills,
		}
	}

	pub fn hunters_aspects(&self) -> [HuntersAspect; 2] {
		match self {
			Self::Base(b) => b.hunters_aspects().map(HuntersAspect::Base),
			Self::Custom {
				hunters_aspects, ..
			} => hunters_aspects.clone(),
		}
	}

	#[must_use]
	pub fn gifts(&self) -> [ShadowGift; 4] {
		match self {
			Self::Base(t) => match t {
				schema::PureTribe::FireTouched => [
					ShadowGift::Disease,
					ShadowGift::Fervor,
					ShadowGift::Insight,
					ShadowGift::Inspiration,
				],
				schema::PureTribe::IvoryClaws => [
					ShadowGift::Agony,
					ShadowGift::Blood,
					ShadowGift::Dominance,
					ShadowGift::Warding,
				],
				schema::PureTribe::PredatorKings => [
					ShadowGift::Hunger,
					ShadowGift::Nature,
					ShadowGift::Rage,
					ShadowGift::Strength,
				],
			},
			Self::Custom { affinity_gifts, .. } => affinity_gifts.as_ref().clone(),
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum Tribe {
	Forsaken(ForsakenTribe),
	Pure(PureTribe),
}

impl Tribe {
	#[must_use]
	pub fn gifts(&self) -> Box<[ShadowGift]> {
		match self {
			Self::Forsaken(f) => Box::from(f.gifts()),
			Self::Pure(p) => Box::from(p.gifts()),
		}
	}
}

impl From<schema::ForsakenTribe> for Tribe {
	fn from(value: schema::ForsakenTribe) -> Self {
		Self::from(ForsakenTribe::from(value))
	}
}

impl From<schema::PureTribe> for Tribe {
	fn from(value: schema::PureTribe) -> Self {
		Self::from(PureTribe::from(value))
	}
}

cofd_macros::gifts!();

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Gift {
	Moon(MoonGift),
	Shadow(ShadowGift),
	Wolf(WolfGift),
}

// impl MoonGift {
// pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
// 	match self {
// 		// MoonGift::Crescent => vec![],
// 		MoonGift::Full => {
// 			if value > 2 {
// 				vec![Modifier::new(
// 					Trait::DerivedTrait(DerivedTrait::Health),
// 					Ability::Renown(Renown::Purity),
// 					ModifierOp::Add,
// 				)]
// 			} else {
// 				vec![]
// 			}
// 		}
// 		// MoonGift::Gibbous => vec![],
// 		// MoonGift::Half => vec![],
// 		// MoonGift::New => vec![],
// 		// MoonGift::_Custom(_) => todo!(),
// 		_ => vec![],
// 	}
// }
// }

impl AbilityTrait for MoonGift {
	fn get_modifiers(&self) -> Box<[CModifier]> {
		match self {
			// MoonGift::Crescent => vec![],
			MoonGift::Full => Box::new([(
				Trait::DerivedTrait(DerivedTrait::Health),
				AttributeModifier::new(
					Value::Attribute(Trait::Size), //
					COp::GreaterThan(2, Box::new(COp::Add)),
				),
			)]),
			// MoonGift::Gibbous => vec![],
			// MoonGift::Half => vec![],
			// MoonGift::New => vec![],
			// MoonGift::_Custom(_) => todo!(),
			_ => Box::new([]),
		}
	}
}

impl AbilityTrait for Renown {}

impl NameKey for MoonGift {
	fn name_key(&self) -> String {
		format!("moon-gifts.{}", self.name())
	}
}

impl NameKey for ShadowGift {
	fn name_key(&self) -> String {
		format!("shadow-gifts.{}", self.name())
	}
}

impl NameKey for WolfGift {
	fn name_key(&self) -> String {
		format!("wolf-gifts.{}", self.name())
	}
}

type Modifiers<const N: usize> = [(Trait, AttributeModifier<Trait, u8, COp>); N];

#[must_use]
pub fn get_modifiers(form: Form) -> &'static [(Trait, AttributeModifier<Trait, u8, COp>)] {
	static HISHU: Modifiers<1> = [(
		Trait::DerivedTrait(DerivedTrait::Perception),
		AttributeModifier::new_const(Value::Value(1), COp::Add),
	)];
	static DALU: Modifiers<3> = [
		(
			Trait::Attribute(Attribute::Strength),
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Stamina),
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
		(
			Trait::Size,
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
	];
	static GAURU: Modifiers<4> = [
		(
			Trait::Attribute(Attribute::Strength),
			AttributeModifier::new_const(Value::Value(3), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Dexterity),
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Stamina),
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
		(
			Trait::Size,
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
	];
	static URHAN: Modifiers<3> = [
		(
			Trait::Attribute(Attribute::Dexterity),
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Stamina),
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
		(
			Trait::Size,
			AttributeModifier::new_const(Value::Value(1), COp::Sub),
		),
	];
	static URSHUL: Modifiers<4> = [
		(
			Trait::Attribute(Attribute::Strength),
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Dexterity),
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
		(
			Trait::Attribute(Attribute::Stamina),
			AttributeModifier::new_const(Value::Value(2), COp::Add),
		),
		(
			Trait::Size,
			AttributeModifier::new_const(Value::Value(1), COp::Add),
		),
	];
	match form {
		Form::Hishu => &HISHU,
		Form::Dalu => &DALU,
		Form::Gauru => &GAURU,
		Form::Urhan => &URHAN,
		Form::Urshul => &URSHUL,
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, VariantName)]
pub enum Rite {
	SacredHunt,
	Custom(String),
}

impl NameKey for Rite {
	fn name_key(&self) -> String {
		format!("werewolf.{}", self.name())
	}
}

#[derive(
	Clone,
	Debug,
	PartialEq,
	Eq,
	Serialize,
	Deserialize,
	Hash,
	AllVariants,
	VariantName,
	derive_more::Display,
)]
pub enum WerewolfMerit {
	#[display("Favored Form ({form})")]
	FavoredForm {
		form: Form,
		//
	},
	EfficientKiller,
	Totem,

	InstinctiveDefense,
}

impl WerewolfMerit {
	#[must_use]
	pub fn is_available(&self, character: &Character<Werewolf>) -> bool {
		let attributes = character.attributes();

		match self {
			Self::InstinctiveDefense => {
				attributes
					.value(&Trait::SupernaturalTolerance(
						SupernaturalTolerance::PrimalUrge,
					))
					.is_some_and(|pu| pu >= 2)
					&& attributes
						.value(&Trait::Skill(Skill::Athletics))
						.is_some_and(|s| s >= 2)
			}
			_ => true,
		}
	}

	// pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
	// 	match self {
	// 		Self::InstinctiveDefense => {
	// 			if value == 2 {
	// 				vec![
	// 					Modifier::conditional(
	// 						Trait::DerivedTrait(DerivedTrait::Defense),
	// 						DicePool::max(Attribute::Wits, Attribute::Dexterity),
	// 						ModifierOp::Set,
	// 						Form::Urhan,
	// 					),
	// 					Modifier::conditional(
	// 						Trait::DerivedTrait(DerivedTrait::Defense),
	// 						DicePool::max(Attribute::Wits, Attribute::Dexterity),
	// 						ModifierOp::Set,
	// 						Form::Urshul,
	// 					),
	// 				]
	// 			} else {
	// 				vec![]
	// 			}
	// 		}
	// 		_ => vec![],
	// 	}
	// }
}

// pub fn get_form_trait(character: &Character, form: &Form, target: &ModifierTarget) -> i16 {
// 	let Splat::Werewolf(data) = &character.splat else {
// 		unreachable!()
// 	};
// 	let active_form = &data.form;
//
// 	let value = match target {
// 		ModifierTarget::BaseAttribute(_)
// 		| ModifierTarget::BaseSkill(_)
// 		| ModifierTarget::Skill(_) => unreachable!(),
// 		ModifierTarget::Attribute(attr) => *character.attributes().get(attr) as i16,
// 		ModifierTarget::Trait(trait_) => character.get_trait(trait_) as i16,
// 	};
//
// 	if form.eq(active_form) {
// 		value
// 	} else {
// 		let modifiers = match target {
// 			ModifierTarget::Trait(trait_) => match trait_ {
// 				Trait::DerivedTrait(DerivedTrait::Speed) => {
// 					form_modifier(character, form, &Attribute::Dexterity)
// 						+ form_modifier(character, form, &Attribute::Strength)
// 						- form_modifier(character, active_form, &Attribute::Dexterity)
// 						- form_modifier(character, active_form, &Attribute::Strength)
// 				}
// 				Trait::DerivedTrait(DerivedTrait::Initiative) => {
// 					form_modifier(character, form, &Attribute::Dexterity)
// 						+ form_modifier(character, form, &Attribute::Composure)
// 						- form_modifier(character, active_form, &Attribute::Dexterity)
// 						- form_modifier(character, active_form, &Attribute::Composure)
// 				}
// 				Trait::DerivedTrait(DerivedTrait::Defense) => {
// 					// let active_form_pool = form_pool(character, active_form, target);
// 					// let form_pool = form_pool(character, form, target);
//
// 					// let attributes = character.attributes();
// 					// println!("{active_form_pool} - {form_pool}");
//
// 					// let attributes = character.attributes();
// 					// let dex = attributes.dexterity as i16
// 					// 	+ form_modifier(character, form, &Attribute::Dexterity)
// 					// 	- form_modifier(character, active_form, &Attribute::Dexterity);
// 					// let wits = attributes.wits as i16
// 					// 	+ form_modifier(character, form, &Attribute::Wits)
// 					// 	- form_modifier(character, active_form, &Attribute::Wits);
//
// 					// TODO: uhh make defense display in forms work.
// 					return value; // Active Form Defense
// 				}
// 				_ => 0,
// 			},
// 			_ => 0,
// 		};
//
// 		let active_form_modifier = form_modifier(character, active_form, target);
// 		let form_mod = form_modifier(character, form, target);
//
// 		// println!("{form:?} {target:?} = {value} - {active_form_modifier} + {form_mod}");
// 		value - active_form_modifier + form_mod + modifiers
// 	}
// }
//
// fn form_modifier(
// 	character: &Character,
// 	form: &Form,
// 	target: &(impl Into<ModifierTarget> + Clone),
// ) -> i16 {
// 	character
// 		.get_conditional_modifier((*target).clone(), form.clone())
// 		.unwrap_or(0)
// }
//
// fn form_pool(
// 	character: &Character,
// 	form: &Form,
// 	target: &(impl Into<ModifierTarget> + Clone),
// ) -> DicePool {
// 	character
// 		.get_conditional_pool((*target).clone(), form.clone())
// 		.unwrap_or(DicePool::Mod(0))
// }
