use serde::{Deserialize, Serialize};
use strum::EnumString;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
pub enum Renown {
	Purity,
	Glory,
	Honor,
	Wisdom,
	Cunning,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq)]
pub enum Auspice {
	Cahalith,
	Elodoth,
	Irraka,
	Ithaeur,
	Rahu,
	// _Custom {
	// 	name: String,
	// 	skills: [Skill; 3],
	// 	renown: Renown,
	// 	// moon_gift: MoonGift,
	// 	// shadow_gifts: Box<[ShadowGift; 2]>,
	// 	// hunters_aspect: HuntersAspect,
	// },
}
