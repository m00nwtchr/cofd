use cofd_util::VariantName;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, VariantArray};

use crate::traits::TraitCategory;

#[derive(
	Clone,
	Copy,
	Debug,
	Hash,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	VariantArray,
	VariantName,
	derive_more::Display,
	Default,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[strum(ascii_case_insensitive)]
pub enum Skill {
	#[default]
	Academics,
	Computer,
	Crafts,
	Investigation,
	Medicine,
	Occult,
	Politics,
	Science,

	Athletics,
	Brawl,
	Drive,
	Firearms,
	Larceny,
	Stealth,
	Survival,
	Weaponry,

	AnimalKen,
	Empathy,
	Expression,
	Intimidation,
	Persuasion,
	Socialize,
	Streetwise,
	Subterfuge,
}

impl Skill {
	#[must_use]
	pub fn all() -> &'static [Self] {
		Self::VARIANTS
	}

	/// Returns the `TraitCategory` of the skill.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::{skill::Skill, TraitCategory};
	/// let skill = Skill::Drive;
	/// assert_eq!(skill.category(), TraitCategory::Physical);
	/// ```
	#[must_use]
	pub fn category(&self) -> TraitCategory {
		match self {
			// Mental skills
			Skill::Academics
			| Skill::Computer
			| Skill::Crafts
			| Skill::Investigation
			| Skill::Medicine
			| Skill::Occult
			| Skill::Politics
			| Skill::Science => TraitCategory::Mental,

			// Physical skills
			Skill::Athletics
			| Skill::Brawl
			| Skill::Drive
			| Skill::Firearms
			| Skill::Larceny
			| Skill::Stealth
			| Skill::Survival
			| Skill::Weaponry => TraitCategory::Physical,

			// Social skills
			Skill::AnimalKen
			| Skill::Empathy
			| Skill::Expression
			| Skill::Intimidation
			| Skill::Persuasion
			| Skill::Socialize
			| Skill::Streetwise
			| Skill::Subterfuge => TraitCategory::Social,
		}
	}

	/// Retrieves all skills that belong to a specific `TraitCategory`.
	///
	/// # Parameters
	///
	/// - `category`: The `TraitCategory` to filter by.
	///
	/// # Examples
	///
	/// ```
	/// # use cofd_schema::traits::{skill::Skill, TraitCategory};
	/// let physical_skills = Skill::get_by_category(TraitCategory::Physical);
	/// assert!(physical_skills.contains(&Skill::Drive));
	/// ```
	#[must_use]
	pub fn get_by_category(category: TraitCategory) -> Vec<Skill> {
		Self::VARIANTS
			.iter()
			.filter(|&skill| skill.category() == category)
			.copied()
			.collect()
	}

	#[must_use]
	pub fn unskilled(&self) -> u8 {
		self.category().unskilled()
	}
}
