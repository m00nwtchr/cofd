use std::{
	fmt,
	fmt::{Debug, Formatter},
	ops::{Add, Neg, Sub},
};

use carboxyl::{lift, Signal, Sink};
use serde::{Deserializer, Serialize, Serializer};

use crate::prelude::Attributes;

#[derive(Clone)]
pub struct RxAttribute<T> {
	sink: Option<Sink<T>>,
	mod_sink: Sink<Signal<T>>,

	value: Signal<T>,
	base_value: Signal<T>,
}

impl<T> Debug for RxAttribute<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("RxAttribute")
			.field("value", &self.value)
			.field("base_value", &self.base_value)
			.finish()
	}
}

fn init_value(sink: &Sink<Signal<i8>>, base_value: Signal<i8>) -> Signal<i8> {
	sink.stream()
		.hold(Signal::new(0))
		.map(move |_mod| lift!(|a, b| a + b, &base_value, &_mod))
		.switch()
}

impl<T> RxAttribute<T> {
	pub fn new(default: T) -> Self {
		let sink = Sink::new();
		let mod_sink = Sink::new();

		let base_value = sink.stream().hold(default);
		let value = init_value(&mod_sink, base_value.clone());

		Self {
			sink: Some(sink),
			mod_sink,
			value,
			base_value,
		}
	}

	pub fn map<F, B>(&self, function: F) -> RxAttribute<B>
	where
		B: Clone + Send + Sync,
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

impl<T: Default> Default for RxAttribute<T> {
	fn default() -> Self {
		RxAttribute::new(Default::default())
	}
}

impl<T> From<Signal<i8>> for RxAttribute<T> {
	fn from(base_value: Signal<i8>) -> Self {
		let mod_sink = Sink::new();
		let value = init_value(&mod_sink, base_value.clone());

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

impl<'a, 'b, T: Add<Rhs>, Rhs> Add<&'b RxAttribute<Rhs>> for &'a RxAttribute<T> {
	type Output = RxAttribute<T::Output>;

	fn add(self, rhs: &'b RxAttribute<Rhs>) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a + b, self.signal(), rhs.signal()))
	}
}

impl<'a, 'b, T: Sub<Rhs>, Rhs> Sub<&'b RxAttribute<Rhs>> for &'a RxAttribute<T> {
	type Output = RxAttribute<T::Output>;

	fn sub(self, rhs: &'b RxAttribute<Rhs>) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a - b, self.signal(), rhs.signal()))
	}
}

impl<'a, T: Neg> Neg for &'a RxAttribute<T> {
	type Output = RxAttribute<T::Output>;

	fn neg(self) -> Self::Output {
		self.map(|a| -a)
	}
}

impl<T: Serialize> Serialize for RxAttribute<T> {
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

#[derive(Clone)]
pub struct DefenseCalc {
	attributes: (Signal<i8>, Signal<i8>),
	skill: Signal<i8>,
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

	let defense_calc_sink: Sink<DefenseCalc> = Sink::new();
	let defense_calc: Signal<i8> = defense_calc_sink
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
					i8::max(a1, a2) + skill
				} else {
					i8::min(a1, a2) + skill
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
