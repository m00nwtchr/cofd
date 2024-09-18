use std::{
	fmt,
	fmt::{Debug, Formatter},
	ops::{Add, Neg, Sub},
};

use carboxyl::{lift, Signal, Sink};
use cofd_schema::prelude::{Attribute, Skill};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{character::Skills, prelude::Attributes};

#[derive(Clone)]
pub struct RxAttribute<T> {
	sink: Option<Sink<T>>,
	mod_sink: Sink<Signal<T>>,

	value: Signal<T>,
	base_value: Signal<T>,
}

impl<T: Debug + Clone + 'static> Debug for RxAttribute<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("RxAttribute")
			.field("value", &self.value)
			.field("base_value", &self.base_value)
			.finish()
	}
}

fn init_value<T>(base_value: Signal<T>, sink: &Sink<Signal<T>>) -> Signal<T>
where
	T: Send + Sync + Clone + Default + Add<T, Output = T> + 'static,
{
	sink.stream()
		.hold(Signal::new(Default::default()))
		.map(move |_mod| lift!(|a, b| a + b, &base_value, &_mod))
		.switch()
}

impl<T> RxAttribute<T>
where
	T: Send + Sync + Clone + Default + 'static,
{
	pub fn new(default: T) -> Self
	where
		T: Add<T, Output = T>,
	{
		let sink = Sink::new();
		let mod_sink = Sink::new();

		let base_value = sink.stream().hold(default);
		let value = init_value(base_value.clone(), &mod_sink);

		Self {
			sink: Some(sink),
			mod_sink,
			value,
			base_value,
		}
	}

	pub fn map<F, B>(&self, function: F) -> RxAttribute<B>
	where
		B: Clone + Send + Sync + Default + Add<Output = B> + 'static,
		F: Fn(T) -> B + Send + Sync + 'static,
	{
		RxAttribute::from(self.signal().map(function))
	}

	pub fn apply(&self, signal: Signal<T>) {
		self.mod_sink.send(signal);
	}

	pub fn set(&self, value: T) {
		if let Some(sink) = &self.sink {
			sink.send(value);
		}
	}

	pub fn signal(&self) -> &Signal<T> {
		&self.value
	}

	pub fn value(&self) -> T {
		self.value.sample()
	}
}

impl<T: Default + Send + Sync + Clone + Add<Output = T> + 'static> Default for RxAttribute<T> {
	fn default() -> Self {
		RxAttribute::new(Default::default())
	}
}

impl<T> From<Signal<T>> for RxAttribute<T>
where
	T: Default + Add<T, Output = T> + Send + Sync + Clone + 'static,
{
	fn from(base_value: Signal<T>) -> Self {
		let mod_sink = Sink::new();
		let value = init_value(base_value.clone(), &mod_sink);

		Self {
			sink: None,
			mod_sink,
			value,
			base_value,
		}
	}
}

impl<T> AsRef<Signal<T>> for RxAttribute<T> {
	fn as_ref(&self) -> &Signal<T> {
		&self.value
	}
}

impl<'a, 'b, T: Add<Rhs> + 'static, Rhs> Add<&'b RxAttribute<Rhs>> for &'a RxAttribute<T>
where
	T: Send + Sync + Default + Clone,
	Rhs: Send + Sync + Default + Clone + 'static,
	<T as Add<Rhs>>::Output: Send
		+ Sync
		+ Default
		+ Clone
		+ Add<<T as Add<Rhs>>::Output, Output = <T as Add<Rhs>>::Output>
		+ 'static,
{
	type Output = RxAttribute<<T as Add<Rhs>>::Output>;

	fn add(self, rhs: &'b RxAttribute<Rhs>) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a + b, self.signal(), rhs.signal()))
	}
}

impl<'a, 'b, T: Sub<Rhs> + 'static, Rhs> Sub<&'b RxAttribute<Rhs>> for &'a RxAttribute<T>
where
	T: Send + Sync + Default + Clone,
	Rhs: Send + Sync + Default + Clone + 'static,
	<T as Sub<Rhs>>::Output: Send
		+ Sync
		+ Default
		+ Clone
		+ Add<<T as Sub<Rhs>>::Output, Output = <T as Sub<Rhs>>::Output> + 'static,
{
	type Output = RxAttribute<T::Output>;

	fn sub(self, rhs: &'b RxAttribute<Rhs>) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a - b, self.signal(), rhs.signal()))
	}
}

impl<'a, T: Neg + 'static> Neg for &'a RxAttribute<T>
where
	T: Send + Sync + Default + Clone,
	<T as Neg>::Output:
		Send + Sync + Default + Clone + Add<<T as Neg>::Output, Output = <T as Neg>::Output>,
{
	type Output = RxAttribute<T::Output>;

	fn neg(self) -> Self::Output {
		self.map(|a| -a)
	}
}

impl<T: Serialize + 'static> Serialize for RxAttribute<T>
where
	T: Send + Sync + Default + Clone,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.base_value.sample().serialize(serializer)
	}
}

#[derive(Clone, Serialize, Debug)]
pub struct RxAttributes {
	pub intelligence: RxAttribute<u8>,
	pub wits: RxAttribute<u8>,
	pub resolve: RxAttribute<u8>,

	pub strength: RxAttribute<u8>,
	pub dexterity: RxAttribute<u8>,
	pub stamina: RxAttribute<u8>,

	pub presence: RxAttribute<u8>,
	pub manipulation: RxAttribute<u8>,
	pub composure: RxAttribute<u8>,
}

impl RxAttributes {
	pub fn get(&self, attribute: Attribute) -> &RxAttribute<u8> {
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
			intelligence: RxAttribute::new(1),
			wits: RxAttribute::new(1),
			resolve: RxAttribute::new(1),
			//
			strength: RxAttribute::new(1),
			dexterity: RxAttribute::new(1),
			stamina: RxAttribute::new(1),
			//
			presence: RxAttribute::new(1),
			manipulation: RxAttribute::new(1),
			composure: RxAttribute::new(1),
		}
	}
}

impl From<Attributes> for RxAttributes {
	fn from(other: Attributes) -> Self {
		Self {
			intelligence: RxAttribute::new(other.intelligence),
			wits: RxAttribute::new(other.wits),
			resolve: RxAttribute::new(other.resolve),
			//
			strength: RxAttribute::new(other.strength),
			dexterity: RxAttribute::new(other.dexterity),
			stamina: RxAttribute::new(other.stamina),
			//
			presence: RxAttribute::new(other.presence),
			manipulation: RxAttribute::new(other.manipulation),
			composure: RxAttribute::new(other.composure),
		}
	}
}

#[derive(Clone, Default, Serialize, Debug)]
#[serde(default)]
pub struct RxSkills {
	pub academics: RxAttribute<u8>,
	pub computer: RxAttribute<u8>,
	pub crafts: RxAttribute<u8>,
	pub investigation: RxAttribute<u8>,
	pub medicine: RxAttribute<u8>,
	pub occult: RxAttribute<u8>,
	pub politics: RxAttribute<u8>,
	pub science: RxAttribute<u8>,

	pub athletics: RxAttribute<u8>,
	pub brawl: RxAttribute<u8>,
	pub drive: RxAttribute<u8>,
	pub firearms: RxAttribute<u8>,
	pub larceny: RxAttribute<u8>,
	pub stealth: RxAttribute<u8>,
	pub survival: RxAttribute<u8>,
	pub weaponry: RxAttribute<u8>,

	pub animal_ken: RxAttribute<u8>,
	pub empathy: RxAttribute<u8>,
	pub expression: RxAttribute<u8>,
	pub intimidation: RxAttribute<u8>,
	pub persuasion: RxAttribute<u8>,
	pub socialize: RxAttribute<u8>,
	pub streetwise: RxAttribute<u8>,
	pub subterfuge: RxAttribute<u8>,
}

impl RxSkills {
	pub fn get(&self, skill: Skill) -> &RxAttribute<u8> {
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
			academics: RxAttribute::new(other.academics),
			computer: RxAttribute::new(other.computer),
			crafts: RxAttribute::new(other.crafts),
			investigation: RxAttribute::new(other.investigation),
			medicine: RxAttribute::new(other.medicine),
			occult: RxAttribute::new(other.occult),
			politics: RxAttribute::new(other.politics),
			science: RxAttribute::new(other.science),
			//
			athletics: RxAttribute::new(other.athletics),
			brawl: RxAttribute::new(other.brawl),
			drive: RxAttribute::new(other.drive),
			firearms: RxAttribute::new(other.firearms),
			larceny: RxAttribute::new(other.larceny),
			stealth: RxAttribute::new(other.stealth),
			survival: RxAttribute::new(other.survival),
			weaponry: RxAttribute::new(other.weaponry),
			//
			animal_ken: RxAttribute::new(other.animal_ken),
			empathy: RxAttribute::new(other.empathy),
			expression: RxAttribute::new(other.expression),
			intimidation: RxAttribute::new(other.intimidation),
			persuasion: RxAttribute::new(other.persuasion),
			socialize: RxAttribute::new(other.socialize),
			streetwise: RxAttribute::new(other.streetwise),
			subterfuge: RxAttribute::new(other.subterfuge),
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

	let defense = RxAttribute::from(defense_calc);
	assert_eq!(defense.value(), 2);

	attributes.dexterity.set(4);
	assert_eq!(defense.value(), 4);

	attributes.wits.set(4);
	assert_eq!(defense.value(), 5);
}
