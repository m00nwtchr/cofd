use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIs, EnumString};

pub(crate) trait AttributeMarker {}

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumIs,
	PartialEq,
	Eq,
	derive_more::Display,
	AsRefStr,
	EnumString,
	Hash,
)]
#[serde(untagged)]
#[strum(ascii_case_insensitive)]
pub enum Attribute {
	Intelligence,
	Wits,
	Resolve,
	Strength,
	Dexterity,
	Stamina,
	Presence,
	Manipulation,
	Composure,
}
