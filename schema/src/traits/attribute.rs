use cofd_util::VariantName;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, VariantArray};

use crate::traits::TraitCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeKind {
	Power,
	Finesse,
	Resistance,
}

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	PartialEq,
	Eq,
	Hash,
	Display,
	AsRefStr,
	EnumString,
	VariantArray,
	VariantName,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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

impl Attribute {
	/// Returns the `TraitCategory` of the attribute.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::{Attribute, TraitCategory};
	/// let attr = Attribute::Dexterity;
	/// assert_eq!(attr.category(), TraitCategory::Physical);
	/// ```
	#[must_use] pub fn category(&self) -> TraitCategory {
		match self {
			Attribute::Intelligence | Attribute::Wits | Attribute::Resolve => TraitCategory::Mental,
			Attribute::Strength | Attribute::Dexterity | Attribute::Stamina => {
				TraitCategory::Physical
			}
			Attribute::Presence | Attribute::Manipulation | Attribute::Composure => {
				TraitCategory::Social
			}
		}
	}

	/// Returns the `AttributeKind` of the attribute.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::attribute::{Attribute, AttributeKind};
	/// let attr = Attribute::Wits;
	/// assert_eq!(attr.kind(), AttributeKind::Finesse);
	/// ```
	#[must_use] pub fn kind(&self) -> AttributeKind {
		match self {
			Attribute::Intelligence | Attribute::Strength | Attribute::Presence => {
				AttributeKind::Power
			}
			Attribute::Wits | Attribute::Dexterity | Attribute::Manipulation => {
				AttributeKind::Finesse
			}
			Attribute::Resolve | Attribute::Stamina | Attribute::Composure => {
				AttributeKind::Resistance
			}
		}
	}

	/// Retrieves all attributes that belong to a specific `TraitCategory`.
	///
	/// # Parameters
	///
	/// - `category`: The `TraitCategory` to filter by.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::{Attribute, TraitCategory};
	/// let physical_attributes = Attribute::get_by_category(TraitCategory::Physical);
	/// assert!(physical_attributes.contains(&Attribute::Strength));
	/// ```
	#[must_use] pub fn get_by_category(category: TraitCategory) -> [Attribute; 3] {
		match category {
			TraitCategory::Mental => [Attribute::Intelligence, Attribute::Wits, Attribute::Resolve],
			TraitCategory::Physical => [
				Attribute::Strength,
				Attribute::Dexterity,
				Attribute::Stamina,
			],
			TraitCategory::Social => [
				Attribute::Presence,
				Attribute::Manipulation,
				Attribute::Composure,
			],
		}
	}

	/// Retrieves all attributes that match a specific `AttributeKind`.
	///
	/// # Parameters
	///
	/// - `kind`: The `AttributeKind` to filter by.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::attribute::{Attribute, AttributeKind};
	/// let finesse_attributes = Attribute::get_by_kind(AttributeKind::Finesse);
	/// assert!(finesse_attributes.contains(&Attribute::Dexterity));
	/// ```
	#[must_use] pub fn get_by_kind(kind: AttributeKind) -> [Attribute; 3] {
		match kind {
			AttributeKind::Power => [
				Attribute::Intelligence,
				Attribute::Strength,
				Attribute::Presence,
			],
			AttributeKind::Finesse => [
				Attribute::Wits,
				Attribute::Dexterity,
				Attribute::Manipulation,
			],
			AttributeKind::Resistance => {
				[Attribute::Resolve, Attribute::Stamina, Attribute::Composure]
			}
		}
	}
}
