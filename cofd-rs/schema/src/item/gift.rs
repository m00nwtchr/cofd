use super::{ActionFields, Item};
use crate::template::werewolf::{Auspice, Renown};

#[derive(Clone, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Moon {
	pub auspice: Auspice,
	pub level: u8,
}

#[derive(Clone, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Other {
	pub renown: Renown,
}

#[derive(Clone, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Facet<T> {
	// #[serde(flatten)]
	pub action: Option<ActionFields>,

	// #[serde(flatten)]
	pub inner: T,
}

#[derive(Clone, Debug, facet::Facet, Hash, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum GiftKind {
	Moon,
	Shadow,
	Wolf,
}

#[derive(Clone, Debug, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gift<T: 'static> {
	pub name: String,
	pub facets: Vec<Item<Facet<T>>>,
	pub kind: GiftKind,
}
