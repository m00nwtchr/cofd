use std::{
	fmt::{Debug, Formatter},
	ops::{Add, Deref, Neg, Sub},
	rc::Weak,
	sync::RwLock,
};

use carboxyl::{lift, Signal, Sink, Stream};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct RxAttribute {
	sink: Option<Sink<i8>>,
	mod_sink: Sink<Signal<i8>>,

	value: Signal<i8>,
	base_value: Signal<i8>,
}

fn init_value(sink: &Sink<Signal<i8>>, base_value: Signal<i8>) -> Signal<i8> {
	sink.stream()
		.hold(Signal::new(0))
		.map(move |_mod| lift!(|a, b| a + b, &base_value, &_mod))
		.switch()
}

impl RxAttribute {
	pub fn new(default: i8) -> Self {
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

	pub fn map<F>(&self, function: F) -> Self
	where
		F: Fn(i8) -> i8 + Send + Sync + 'static,
	{
		RxAttribute::from(self.signal().map(function))
	}

	pub fn apply(&self, signal: Signal<i8>) {
		self.mod_sink.send(signal);
	}

	pub fn set(&self, value: i8) {
		if let Some(sink) = &self.sink {
			sink.send(value);
		}
	}

	pub fn signal(&self) -> &Signal<i8> {
		&self.value
	}

	pub fn value(&self) -> i8 {
		self.value.sample()
	}
}

impl Default for RxAttribute {
	fn default() -> Self {
		RxAttribute::new(1)
	}
}

impl From<Signal<i8>> for RxAttribute {
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

impl AsRef<Signal<i8>> for RxAttribute {
	fn as_ref(&self) -> &Signal<i8> {
		&self.value
	}
}

impl<'a, 'b> Add<&'b RxAttribute> for &'a RxAttribute {
	type Output = RxAttribute;

	fn add(self, rhs: &'b RxAttribute) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a + b, self.signal(), rhs.signal()))
	}
}

impl<'a, 'b> Sub<&'b RxAttribute> for &'a RxAttribute {
	type Output = RxAttribute;

	fn sub(self, rhs: &'b RxAttribute) -> Self::Output {
		RxAttribute::from(lift!(|a, b| a - b, self.signal(), rhs.signal()))
	}
}

impl<'a> Neg for &'a RxAttribute {
	type Output = RxAttribute;

	fn neg(self) -> Self::Output {
		self.map(|a| -a)
	}
}

impl Serialize for RxAttribute {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.base_value.sample().serialize(serializer)
	}
}

#[derive(Default)]
pub struct RxAttributes {
	pub intelligence: RxAttribute,
	pub wits: RxAttribute,
	pub resolve: RxAttribute,

	pub strength: RxAttribute,
	pub dexterity: RxAttribute,
	pub stamina: RxAttribute,

	pub presence: RxAttribute,
	pub manipulation: RxAttribute,
	pub composure: RxAttribute,
}

impl RxAttributes {}

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
