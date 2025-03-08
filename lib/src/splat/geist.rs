use cofd_util::{AllVariants, VariantName};
use serde::{Deserialize, Serialize};

use super::{ability::Ability, Merit, NameKey, SplatTrait, XSplat, YSplat, ZSplat};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bound {
	pub burden: Burden,
	pub archetype: Archetype,

	pub keys: Vec<Key>,
}

impl SplatTrait for Bound {
	fn set_xsplat(&mut self, splat: Option<XSplat>) {
		if let Some(XSplat::Burden(burden)) = splat {
			self.burden = burden;
		}
	}

	fn set_ysplat(&mut self, splat: Option<YSplat>) {
		if let Some(YSplat::Archetype(archetype)) = splat {
			self.archetype = archetype;
		}
	}

	fn set_zsplat(&mut self, _splat: Option<ZSplat>) {}

	fn xsplat(&self) -> Option<XSplat> {
		Some(self.burden.clone().into())
	}

	fn ysplat(&self) -> Option<YSplat> {
		Some(self.archetype.clone().into())
	}

	fn xsplats(&self) -> Vec<XSplat> {
		Burden::all().into_iter().map(Into::into).collect()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Archetype::all().into_iter().map(Into::into).collect()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Vec::new()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(Burden::Custom(name, [Haunt::Boneyard, Haunt::Caul, Haunt::Curse]).into())
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(Archetype::Custom(name).into())
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		Some(Haunt::all().into_iter().map(Into::into).collect())
	}

	fn alternate_beats_optional(&self) -> bool {
		false
	}

	fn merits(&self) -> Vec<Merit> {
		vec![]
	}
}

#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AllVariants, VariantName, Default,
)]
pub enum Burden {
	#[default]
	Abiding,
	Bereaved,
	Hungry,
	Kindly,
	Vengeful,
	Custom(String, [Haunt; 3]),
}

impl Burden {
	pub fn get_favoured_haunts(&self) -> &[Haunt; 3] {
		match self {
			Self::Abiding => &[Haunt::Caul, Haunt::Memoria, Haunt::Tomb],
			Self::Bereaved => &[Haunt::Curse, Haunt::Oracle, Haunt::Shroud],
			Self::Hungry => &[Haunt::Boneyard, Haunt::Marionette, Haunt::Caul],
			Self::Kindly => &[Haunt::Dirge, Haunt::Marionette, Haunt::Shroud],
			Self::Vengeful => &[Haunt::Curse, Haunt::Memoria, Haunt::Rage],
			Self::Custom(_, haunts) => haunts,
		}
	}
}

#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AllVariants, VariantName, Default,
)]
pub enum Archetype {
	#[default]
	Furies,
	Mourners,
	Necropolitans,
	Pilgrims,
	Undertakers,
	Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AllVariants, VariantName)]
pub enum Haunt {
	Boneyard,
	Caul,
	Curse,
	Dirge,
	Marionette,
	Memoria,
	Oracle,
	Rage,
	Shroud,
	Tomb,
	Custom(String),
}

impl From<Haunt> for Ability {
	fn from(val: Haunt) -> Self {
		Ability::Haunt(val)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, AllVariants, VariantName)]
pub enum Key {
	Beasts,
	Blood,
	Chance,
	ColdWind,
	Disease,
	GraveDirt,
	PyreFlame,
	Stillness,
}

impl NameKey for Key {
	fn name_key(&self) -> String {
		format!("geist.{}", self.name())
	}
}
