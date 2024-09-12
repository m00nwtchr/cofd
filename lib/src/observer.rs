use std::{
	fmt::{Debug, Formatter},
	ops::Deref,
	rc::Weak,
};

use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct State<T: Clone> {
	value: T,

	observers: Vec<Weak<Self>>,
}

impl<T: Clone> State<T> {
	pub fn register(&mut self, observer: Weak<Self>) {
		self.observers.push(observer);
	}

	fn notify(&self) {}

	pub fn set(&mut self, value: T) {
		self.value = value;
		for observer in &self.observers {
			if let Some(observer) = observer.upgrade() {
				observer.notify();
			}
		}
	}
}

impl<T: Clone> Deref for State<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T: Clone> AsRef<T> for State<T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

impl<T: Clone + Debug> Debug for State<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.value.fmt(f)
	}
}

impl<T: Clone + Serialize> Serialize for State<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.value.serialize(serializer)
	}
}
