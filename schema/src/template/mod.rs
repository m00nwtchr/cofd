use cofd_util::{SplatEnum, VariantName};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, VariantArray};

pub mod changeling;
pub mod mage;
pub mod vampire;
pub mod werewolf;

#[derive(
	SplatEnum,
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	Display,
	PartialEq,
	Eq,
	Hash,
	VariantName,
	VariantArray,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[strum(ascii_case_insensitive)]
pub enum Template {
	#[strum(to_string = "Mortal", serialize = "Human")]
	Mortal,
	#[splat(
		xsplat = "path",
		ysplat = "order",
		zsplat = "legacy",
		ability = "arcana",
		st = SupernaturalTolerance::Gnosis,
		alt_beats = "arcane",
		fuel = Fuel::Mana,
		integrity = Integrity::Wisdom,
	)]
	#[strum(to_string = "Mage", serialize = "Awakened")]
	Mage,
	#[splat(
		xsplat = "clan",
		ysplat = "covenant",
		zsplat = "bloodline",
		virtue_anchor = Anchor::Mask,
		vice_anchor = Anchor::Dirge,
		ability = "disciplines",
		st = SupernaturalTolerance::BloodPotency,
		alt_beats = "blood",
		fuel = Fuel::Vitae,
		integrity = Integrity::Humanity,
		abilities_finite = false
	)]
	#[strum(to_string = "Vampire", serialize = "Kindred")]
	Vampire,
	#[splat(
		xsplat = "auspice",
		ysplat = "tribe",
		zsplat = "lodge",
		virtue_anchor = Anchor::Blood,
		vice_anchor = Anchor::Bone,
		ability = "renown",
		st = SupernaturalTolerance::PrimalUrge,
		fuel = Fuel::Essence,
		integrity = Integrity::Harmony
	)]
	Werewolf,
	Promethean,
	#[splat(
		xsplat = "seeming",
		ysplat = "court",
		zsplat = "kith",
		virtue_anchor = Anchor::Thread,
		vice_anchor = Anchor::Needle,
		st = SupernaturalTolerance::Wyrd,
		fuel = Fuel::Glamour,
		integrity = Integrity::Clarity,
		abilities_finite = false
	)]
	Changeling,
	Hunter,
	#[splat(
		xsplat = "burden",
		ysplat = "archetype",
		virtue_anchor = Anchor::Root,
		vice_anchor = Anchor::Bloom,
		ability = "haunts",
		st = SupernaturalTolerance::Synergy,
		fuel = Fuel::Plasm,
		integrity = Integrity::Synergy,
		abilities_finite = false
	)]
	Bound,
	Mummy,
	Demon,
	Beast,
	Deviant,

	// Mage
	Sleepwalker,
	Proximi,
	// Vampire
	Ghoul,
	// Werewolf
	#[strum(to_string = "Wolf-Blooded")]
	WolfBlooded,
	// Changeling
	#[strum(to_string = "Fae-Blooded")]
	FaeTouched,
	// Mummy
	Endless,
	// Demon
	#[strum(to_string = "Demon-Blooded")]
	DemonBlooded,
	Stigmatic,
}

#[derive(
	Debug,
	Clone,
	Hash,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	PartialEq,
	Eq,
	Display,
	VariantName,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[strum(ascii_case_insensitive)]
pub enum SupernaturalTolerance {
	Gnosis,
	#[strum(to_string = "Blood Potency")]
	BloodPotency,
	#[strum(to_string = "Primal Urge")]
	PrimalUrge,
	Azoth,
	Wyrd,
	Synergy,
	Sekhem,
	Primium,
	Lair,
}

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	PartialEq,
	Eq,
	Display,
	VariantName,
)]
#[strum(ascii_case_insensitive)]
pub enum Anchor {
	Virtue,
	Vice,

	Mask,
	Dirge,

	Blood,
	Bone,

	// Promethean
	Thread,
	Needle,

	Root,
	Bloom,

	// Mummy
	Life,
	Legend,
}

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	PartialEq,
	Eq,
	Display,
	VariantName,
)]
#[strum(ascii_case_insensitive)]
pub enum Integrity {
	Integrity,
	Wisdom,
	Humanity,
	Harmony,
	Pilgrimage,
	Clarity,
	Synergy,
	Memory,
	Cover,
	// Beast, Nothing
	ConvictionLoyalty,
}

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	EnumString,
	AsRefStr,
	PartialEq,
	Eq,
	Display,
	VariantName,
)]
#[strum(ascii_case_insensitive)]
pub enum Fuel {
	Mana,
	Vitae,
	Essence,
	Pyros,
	Glamour,
	Plasm,
	PillarPoint,
	Aether,
	// Satiety,
}
