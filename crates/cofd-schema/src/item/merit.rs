use strum::{AsRefStr, Display, EnumString};

use super::ActionFields;
use crate::{prelude::DotRange, prerequisites::Prerequisites};

#[derive(Debug, Clone, Copy, facet::Facet, EnumString, AsRefStr, PartialEq, Eq, Display)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum MeritTag {
	Style,
	#[strum(to_string = "Supernatural Merit")]
	Supernatural,

	Special,

	// TODO: pdf parse
	Mental,
	Social,
	Physical,

	Fighting,
}

#[derive(Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[serde(rename_all = "camelCase")]
pub struct MeritSubItem {
	pub name: String,
	pub description: Vec<String>,
	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub prerequisites: Prerequisites,
	pub dot_rating: DotRange,
	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub drawbacks: Vec<String>,
}

#[derive(Default, Debug, Clone, facet::Facet, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[serde(rename_all = "camelCase")]
pub struct Merit {
	pub dot_rating: DotRange,
	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub prerequisites: Prerequisites,
	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub tags: Vec<MeritTag>,
	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub drawbacks: Vec<String>,

	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub children: Vec<MeritSubItem>,

	// #[serde(default, skip_serializing_if = "Option::is_none")]
	// #[serde(flatten)]
	pub action: Option<ActionFields>,

	// #[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub notes: Vec<String>,
}
