use std::ops::Deref;

use cofd_util::{AllVariants, NameKey, VariantName};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, VariantArray};

use self::ability::Ability;

pub mod ability;
pub mod merits;

pub use merits::*;

pub mod mage;
pub mod vampire;
pub mod werewolf;
// pub mod promethean;
pub mod changeling;
// pub mod hunter;
pub mod geist;
mod mortal;
// pub mod mummy;
// pub mod demon;
// pub mod beast;
// pub mod deviant;

// use promethean::*;
use changeling::*;
use cofd_schema::template::Template;
// use hunter::*;
use geist::*;
use mage::*;
use mortal::*;
use vampire::*;
use werewolf::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;

#[derive(
	Clone, PartialEq, Eq, Serialize, Deserialize, Debug, VariantName, AllVariants, EnumDiscriminants,
)]
#[strum_discriminants(name(SplatKind), derive(strum::VariantArray, VariantName))]
#[enum_dispatch(SplatTrait)]
pub enum Splat {
	Mortal(Mortal),
	Vampire(Vampire),
	Werewolf(Werewolf),
	Mage(Mage),
	// Promethean(Lineage),
	Changeling(Changeling),
	// Hunter(Tier),
	Bound(Bound),
	// Mummy(Decree, Guild),
	// Demon(Incarnation, Vec<Agenda>),
	// Beast(Hunger),
	// Deviant(Origin, Clade, Vec<Form>),
}

impl SplatKind {
	pub fn all() -> &'static [Self] {
		Self::VARIANTS
	}
}

impl From<SplatKind> for Splat {
	fn from(value: SplatKind) -> Self {
		match value {
			SplatKind::Mortal => Splat::Mortal(Mortal),
			SplatKind::Vampire => Splat::Vampire(Vampire::default()),
			SplatKind::Werewolf => Splat::Werewolf(Werewolf::default()),
			SplatKind::Mage => Splat::Mage(Mage::default()),
			SplatKind::Changeling => Splat::Changeling(Changeling::default()),
			SplatKind::Bound => Splat::Bound(Bound::default()),
		}
	}
}

impl Deref for Splat {
	type Target = Template;

	fn deref(&self) -> &Self::Target {
		match self {
			Splat::Mortal(_) => &Template::Mortal,
			Splat::Vampire(_) => &Template::Vampire,
			Splat::Werewolf(_) => &Template::Werewolf,
			Splat::Mage(_) => &Template::Mage,
			Splat::Changeling(_) => &Template::Changeling,
			Splat::Bound(_) => &Template::Bound,
		}
	}
}

#[enum_dispatch]
pub trait SplatTrait {
	fn set_xsplat(&mut self, splat: Option<XSplat>);

	fn set_ysplat(&mut self, splat: Option<YSplat>);

	fn set_zsplat(&mut self, splat: Option<ZSplat>);

	fn xsplat(&self) -> Option<XSplat> {
		// unimplemented!();
		None
	}
	fn ysplat(&self) -> Option<YSplat> {
		// unimplemented!();
		None
	}
	fn zsplat(&self) -> Option<ZSplat> {
		// unimplemented!();
		None
	}

	fn xsplats(&self) -> Vec<XSplat>;
	fn ysplats(&self) -> Vec<YSplat>;
	fn zsplats(&self) -> Vec<ZSplat>;

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		None
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		None
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		None
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		None
	}

	fn custom_ability(&self, name: String) -> Option<Ability> {
		None
	}

	fn alternate_beats_optional(&self) -> bool {
		true
	}

	fn merits(&self) -> Vec<Merit>;
}

impl Default for Splat {
	fn default() -> Self {
		Splat::Mortal(Mortal)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, VariantName, derive_more::From, derive_more::TryInto)]
pub enum XSplat {
	#[expand]
	Clan(Clan),
	#[expand]
	Auspice(Auspice),
	#[expand]
	Path(Path),
	#[expand]
	Seeming(Seeming),
	#[expand]
	Burden(Burden),
}
#[derive(Debug, Clone, PartialEq, Eq, VariantName, derive_more::From, derive_more::TryInto)]
pub enum YSplat {
	#[expand]
	Covenant(Covenant),
	#[expand]
	Tribe(Tribe),
	#[expand]
	Order(Order),
	#[expand]
	Court(Court),
	#[expand]
	Archetype(Archetype),
}
#[derive(Debug, Clone, PartialEq, Eq, VariantName, derive_more::From, derive_more::TryInto)]
pub enum ZSplat {
	#[expand]
	Bloodline(Bloodline),
	#[expand]
	Lodge(Lodge),
	#[expand]
	Legacy(Legacy),
	#[expand]
	Kith(Kith),
}

impl XSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Clan(Clan::_Custom(name, ..))
			| Self::Auspice(Auspice::_Custom(name, ..))
			| Self::Path(Path::_Custom(name, ..))
			| Self::Seeming(Seeming::_Custom(name, ..))
			| Self::Burden(Burden::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Self::Clan(Clan::_Custom(..))
				| Self::Auspice(Auspice::_Custom(..))
				| Self::Path(Path::_Custom(..))
				| Self::Seeming(Seeming::_Custom(..))
				| Self::Burden(Burden::_Custom(..))
		)
	}
}

impl YSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Covenant(Covenant::_Custom(name))
			| Self::Tribe(Tribe::_Custom(name, ..))
			| Self::Order(
				Order::_Custom(name, ..)
				| Order::SeersOfTheThrone(Some(Ministry::_Custom(name, ..))),
			)
			| Self::Court(Court::_Custom(name))
			| Self::Archetype(Archetype::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			YSplat::Covenant(Covenant::_Custom(..))
				| YSplat::Tribe(Tribe::_Custom(..))
				| YSplat::Order(
					Order::_Custom(..) | Order::SeersOfTheThrone(Some(Ministry::_Custom(..))),
				) | YSplat::Court(Court::_Custom(..))
				| Self::Archetype(Archetype::_Custom(..))
		)
	}
}

impl ZSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Bloodline(Bloodline::_Custom(name, ..))
			| ZSplat::Lodge(Lodge::_Custom(name))
			| ZSplat::Legacy(Legacy::_Custom(name, ..))
			| ZSplat::Kith(Kith::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			ZSplat::Bloodline(Bloodline::_Custom(..))
				| ZSplat::Lodge(Lodge::_Custom(..))
				| ZSplat::Legacy(Legacy::_Custom(..))
				| ZSplat::Kith(Kith::_Custom(..))
		)
	}
}
