use std::{
	collections::HashMap,
	fmt,
	fmt::{Debug, Formatter},
	ops::{Add, Neg, Sub},
	sync::{Arc, RwLock},
};

use carboxyl::{lift, Signal, Sink};
use cofd_schema::prelude::{Attribute, Skill};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::character::{attributes::Attributes, modifier, modifier::ModifierKey, skills::Skills};

#[derive(Clone)]
pub struct RxValue<T>
where
	T: Clone + Sync + Send + 'static,
{
	sink: Sink<T>,
	value: Signal<T>,
}

impl<T> RxValue<T>
where
	T: Clone + Sync + Send + 'static,
{
	pub fn new(init: T) -> Self {
		let sink = Sink::new();
		let value = sink.stream().hold(init);

		Self { sink, value }
	}

	pub fn set(&self, value: T) {
		self.sink.send(value)
	}

	pub fn value(&self) -> T {
		self.value.sample()
	}

	pub fn signal(&self) -> &Signal<T> {
		&self.value
	}
}

impl<T> Debug for RxValue<T>
where
	T: Clone + Sync + Send + 'static + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("RxValue")
			.field("value", &self.value)
			.finish()
	}
}

impl<T> Default for RxValue<T>
where
	T: Clone + Sync + Send + 'static + Default,
{
	fn default() -> Self {
		RxValue::new(T::default())
	}
}

impl<T> PartialEq for RxValue<T>
where
	T: Clone + Sync + Send + 'static + PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.value.sample().eq(&other.value.sample())
	}
}
impl<T> Eq for RxValue<T> where T: Clone + Sync + Send + 'static + Eq {}

impl<T> Serialize for RxValue<T>
where
	T: Send + Sync + Default + Clone + Serialize + 'static,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.value.sample().serialize(serializer)
	}
}

impl<'de, T> Deserialize<'de> for RxValue<T>
where
	T: 'static + Clone + Default + Send + Deserialize<'de> + Sync,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		T::deserialize(deserializer).map(|a| RxValue::new(a))
	}
}

type ModifierMap = HashMap<ModifierKey, Signal<u8>>;

#[cfg(test)]
mod tests {
	use crate::reactive::RxTrait;

	#[test]
	pub fn size() {
		println!("{}", std::mem::size_of::<RxTrait>());
	}
}

#[derive(Clone)]
pub struct RxTrait {
	sink: Option<Sink<u8>>,
	modifier_sink: Sink<Signal<u8>>,
	base_modifier_sink: Sink<Signal<u8>>,

	modifier_map: Arc<RwLock<ModifierMap>>,
	base_modifier: Arc<RwLock<ModifierMap>>,

	value: Signal<u8>,
	base_value: Signal<u8>,
	raw_value: Signal<u8>,
}

impl Debug for RxTrait {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("RxTrait")
			.field("value", &self.value)
			.field("base_value", &self.base_value)
			.finish()
	}
}

fn init_value(base_value: Signal<u8>, modifier_sink: &Sink<Signal<u8>>) -> Signal<u8> {
	modifier_sink
		.stream()
		.hold(Signal::new(Default::default()))
		.map(move |modifier| lift!(|a, b| a + b, &base_value, &modifier))
		.switch()
}

impl RxTrait {
	pub fn new(default: u8) -> Self {
		let base_modifier_sink = Sink::new();
		let modifier_sink = Sink::new();
		let sink = Sink::new();

		let raw_value = sink.stream().hold(default);
		let base_value = init_value(raw_value.clone(), &base_modifier_sink);
		let value = init_value(base_value.clone(), &modifier_sink);

		Self {
			sink: Some(sink),
			modifier_sink,
			base_modifier_sink,
			modifier_map: Default::default(),
			base_modifier: Default::default(),
			value,
			base_value,
			raw_value,
		}
	}

	pub fn map<F>(&self, function: F) -> RxTrait
	where
		F: Fn(u8) -> u8 + Sync + Send + 'static,
	{
		RxTrait::from(self.signal().map(function))
	}

	pub fn add_modifier(&self, key: ModifierKey, value: Signal<u8>) {
		self.modifier_map.write().expect("").insert(key, value);
		self.apply()
		// self.modifier_sink.send(signal);
	}

	pub fn remove_modifier(&self, key: &ModifierKey) {
		self.modifier_map.write().expect("").remove(key);
		self.apply()
		// self.modifier_sink.send(signal);
	}
	
	fn apply(&self) {
		let mut sig = self.base_value.clone();

		for (_, v) in self.modifier_map.read().expect("").iter() {
			sig = lift!(|a, b| a + b, &sig, v)
		}

		self.modifier_sink.send(sig);
	}

	pub fn set(&self, value: u8) {
		if let Some(sink) = &self.sink {
			sink.send(value);
		}
	}

	pub fn signal(&self) -> &Signal<u8> {
		&self.value
	}

	pub fn value(&self) -> u8 {
		self.value.sample()
	}
	pub fn base_value(&self) -> u8 {
		self.base_value.sample()
	}
}

impl Default for RxTrait {
	fn default() -> Self {
		RxTrait::new(u8::default())
	}
}

impl From<Signal<u8>> for RxTrait {
	fn from(raw_value: Signal<u8>) -> Self {
		let base_modifier_sink = Sink::new();
		let modifier_sink = Sink::new();

		let base_value = init_value(raw_value.clone(), &base_modifier_sink);
		let value = init_value(base_value.clone(), &modifier_sink);

		Self {
			sink: None,
			modifier_sink,
			value,
			base_value,
			raw_value,
			base_modifier_sink,
			modifier_map: Default::default(),
			base_modifier: Default::default(),
		}
	}
}

impl AsRef<Signal<u8>> for RxTrait {
	fn as_ref(&self) -> &Signal<u8> {
		&self.value
	}
}

impl<'a, 'b> Add<&'b RxTrait> for &'a RxTrait {
	type Output = RxTrait;

	fn add(self, rhs: &'b RxTrait) -> Self::Output {
		RxTrait::from(lift!(|a, b| a + b, self.signal(), rhs.signal()))
	}
}

impl<'a, 'b> Sub<&'b RxTrait> for &'a RxTrait {
	type Output = RxTrait;

	fn sub(self, rhs: &'b RxTrait) -> Self::Output {
		RxTrait::from(lift!(|a, b| a - b, self.signal(), rhs.signal()))
	}
}

// impl<'a> Neg for &'a RxAttribute {
// 	type Output = RxAttribute;
//
// 	fn neg(self) -> Self::Output {
// 		self.map(|a| -a)
// 	}
// }

impl Serialize for RxTrait {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.raw_value.sample().serialize(serializer)
	}
}

#[derive(Clone, Serialize, Debug)]
pub struct RxAttributes {
	pub intelligence: RxTrait,
	pub wits: RxTrait,
	pub resolve: RxTrait,

	pub strength: RxTrait,
	pub dexterity: RxTrait,
	pub stamina: RxTrait,

	pub presence: RxTrait,
	pub manipulation: RxTrait,
	pub composure: RxTrait,
}

impl RxAttributes {
	pub fn get(&self, attribute: Attribute) -> &RxTrait {
		match attribute {
			Attribute::Intelligence => &self.intelligence,
			Attribute::Wits => &self.wits,
			Attribute::Resolve => &self.resolve,
			Attribute::Strength => &self.strength,
			Attribute::Dexterity => &self.dexterity,
			Attribute::Stamina => &self.stamina,
			Attribute::Presence => &self.presence,
			Attribute::Manipulation => &self.manipulation,
			Attribute::Composure => &self.composure,
		}
	}
}

impl Default for RxAttributes {
	fn default() -> Self {
		Self {
			intelligence: RxTrait::new(1),
			wits: RxTrait::new(1),
			resolve: RxTrait::new(1),
			//
			strength: RxTrait::new(1),
			dexterity: RxTrait::new(1),
			stamina: RxTrait::new(1),
			//
			presence: RxTrait::new(1),
			manipulation: RxTrait::new(1),
			composure: RxTrait::new(1),
		}
	}
}

impl From<Attributes> for RxAttributes {
	fn from(other: Attributes) -> Self {
		Self {
			intelligence: RxTrait::new(other.intelligence),
			wits: RxTrait::new(other.wits),
			resolve: RxTrait::new(other.resolve),
			//
			strength: RxTrait::new(other.strength),
			dexterity: RxTrait::new(other.dexterity),
			stamina: RxTrait::new(other.stamina),
			//
			presence: RxTrait::new(other.presence),
			manipulation: RxTrait::new(other.manipulation),
			composure: RxTrait::new(other.composure),
		}
	}
}

#[derive(Clone, Default, Serialize, Debug)]
#[serde(default)]
pub struct RxSkills {
	pub academics: RxTrait,
	pub computer: RxTrait,
	pub crafts: RxTrait,
	pub investigation: RxTrait,
	pub medicine: RxTrait,
	pub occult: RxTrait,
	pub politics: RxTrait,
	pub science: RxTrait,

	pub athletics: RxTrait,
	pub brawl: RxTrait,
	pub drive: RxTrait,
	pub firearms: RxTrait,
	pub larceny: RxTrait,
	pub stealth: RxTrait,
	pub survival: RxTrait,
	pub weaponry: RxTrait,

	pub animal_ken: RxTrait,
	pub empathy: RxTrait,
	pub expression: RxTrait,
	pub intimidation: RxTrait,
	pub persuasion: RxTrait,
	pub socialize: RxTrait,
	pub streetwise: RxTrait,
	pub subterfuge: RxTrait,
}

impl RxSkills {
	pub fn get(&self, skill: Skill) -> &RxTrait {
		match skill {
			Skill::Academics => &self.academics,
			Skill::Computer => &self.computer,
			Skill::Crafts => &self.crafts,
			Skill::Investigation => &self.investigation,
			Skill::Medicine => &self.medicine,
			Skill::Occult => &self.occult,
			Skill::Politics => &self.politics,
			Skill::Science => &self.science,
			//
			Skill::Athletics => &self.athletics,
			Skill::Brawl => &self.brawl,
			Skill::Drive => &self.drive,
			Skill::Firearms => &self.firearms,
			Skill::Larceny => &self.larceny,
			Skill::Stealth => &self.stealth,
			Skill::Survival => &self.survival,
			Skill::Weaponry => &self.weaponry,
			//
			Skill::AnimalKen => &self.animal_ken,
			Skill::Empathy => &self.empathy,
			Skill::Expression => &self.expression,
			Skill::Intimidation => &self.intimidation,
			Skill::Persuasion => &self.persuasion,
			Skill::Socialize => &self.socialize,
			Skill::Streetwise => &self.streetwise,
			Skill::Subterfuge => &self.subterfuge,
		}
	}
}

impl From<Skills> for RxSkills {
	fn from(other: Skills) -> Self {
		Self {
			academics: RxTrait::new(other.academics),
			computer: RxTrait::new(other.computer),
			crafts: RxTrait::new(other.crafts),
			investigation: RxTrait::new(other.investigation),
			medicine: RxTrait::new(other.medicine),
			occult: RxTrait::new(other.occult),
			politics: RxTrait::new(other.politics),
			science: RxTrait::new(other.science),
			//
			athletics: RxTrait::new(other.athletics),
			brawl: RxTrait::new(other.brawl),
			drive: RxTrait::new(other.drive),
			firearms: RxTrait::new(other.firearms),
			larceny: RxTrait::new(other.larceny),
			stealth: RxTrait::new(other.stealth),
			survival: RxTrait::new(other.survival),
			weaponry: RxTrait::new(other.weaponry),
			//
			animal_ken: RxTrait::new(other.animal_ken),
			empathy: RxTrait::new(other.empathy),
			expression: RxTrait::new(other.expression),
			intimidation: RxTrait::new(other.intimidation),
			persuasion: RxTrait::new(other.persuasion),
			socialize: RxTrait::new(other.socialize),
			streetwise: RxTrait::new(other.streetwise),
			subterfuge: RxTrait::new(other.subterfuge),
		}
	}
}

#[derive(Clone)]
struct DefenseCalc {
	attributes: (Signal<u8>, Signal<u8>),
	skill: Signal<u8>,
	flag: Signal<bool>,
}

#[test]
pub fn test() {
	let attributes = RxAttributes::default();

	let perception = &attributes.wits + &attributes.composure;
	attributes.wits.set(3);
	attributes.composure.set(3);

	assert_eq!(perception.value(), 6);
	perception.apply(Signal::new(1));
	assert_eq!(perception.value(), 7);
	perception.apply(Signal::new(2));
	assert_eq!(perception.value(), 8);

	let defense_calc_sink = Sink::new();
	let defense_calc = defense_calc_sink
		.stream()
		.hold(DefenseCalc {
			attributes: (
				attributes.wits.signal().clone(),
				attributes.dexterity.signal().clone(),
			),
			skill: Signal::new(1),
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

	let defense = RxTrait::from(defense_calc);
	assert_eq!(defense.value(), 2);

	attributes.dexterity.set(4);
	assert_eq!(defense.value(), 4);

	attributes.wits.set(4);
	assert_eq!(defense.value(), 5);
}
