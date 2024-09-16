use std::i8;
use std::sync::RwLock;
use std::{
	fmt::{Debug, Formatter},
	ops::Add,
	ops::Deref,
	rc::Weak,
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

impl RxAttribute {
	pub fn new(default: i8) -> Self {
		let sink = Sink::new();
		let mod_sink = Sink::new();

		let base_value = sink.stream().hold(default);
		let value = init_value(&mod_sink, &base_value);

		Self {
			sink: Some(sink),
			mod_sink,
			value,
			base_value,
		}
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

	fn sink(&self) -> Option<&Sink<i8>> {
		self.sink.as_ref()
	}

	pub fn value(&self) -> i8 {
		self.value.sample()
	}
}

fn init_value(sink: &Sink<Signal<i8>>, base_value: &Signal<i8>) -> Signal<i8> {
	sink.stream()
		.hold(Signal::new(0))
		.map({
			let base_value = base_value.clone();
			move |_mod| lift!(|a, b| a + b, &base_value, &_mod)
		})
		.switch()
}

impl From<Signal<i8>> for RxAttribute {
	fn from(base_value: Signal<i8>) -> Self {
		let sink = Sink::new();
		let mod_sink = Sink::new();

		let value = init_value(&mod_sink, &base_value);

		Self {
			sink: Some(sink),
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

#[test]
pub fn test() {
	let wits = RxAttribute::new(1);
	let composure = RxAttribute::new(1);

	let perception = &wits + &composure;
	wits.set(3);
	composure.set(3);

	assert_eq!(perception.value(), 6);

	perception.apply(Signal::new(1));
	assert_eq!(perception.value(), 7);

	perception.apply(Signal::new(2));
	assert_eq!(perception.value(), 8);
}
