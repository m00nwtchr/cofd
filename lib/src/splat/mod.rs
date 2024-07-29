use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use cofd_util::{AllVariants, NameKey, SplatEnum, VariantName};

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

use mage::*;
use vampire::*;
use werewolf::*;
// use promethean::*;
use changeling::*;
use cofd_schema::template::Template;
// use hunter::*;
use geist::*;
use mortal::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;

#[derive(Clone, Serialize, Deserialize, Debug, VariantName, AllVariants)]
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
trait SplatTrait {
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
	Vampire(Clan),
	Werewolf(Auspice),
	Mage(Path),
	Changeling(Seeming),
	Bound(Burden),
}
#[derive(Debug, Clone, PartialEq, Eq, VariantName, derive_more::From, derive_more::TryInto)]
pub enum YSplat {
	Vampire(Covenant),
	Werewolf(Tribe),
	Mage(Order),
	Changeling(Court),
	Bound(Archetype),
}
#[derive(Debug, Clone, PartialEq, Eq, VariantName, derive_more::From, derive_more::TryInto)]
pub enum ZSplat {
	Vampire(Bloodline),
	Werewolf(Lodge),
	Mage(Legacy),
	Changeling(Kith),
}

impl XSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Clan::_Custom(name, ..))
			| Self::Werewolf(Auspice::_Custom(name, ..))
			| Self::Mage(Path::_Custom(name, ..))
			| Self::Changeling(Seeming::_Custom(name, ..))
			| Self::Bound(Burden::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Self::Vampire(Clan::_Custom(..))
				| Self::Werewolf(Auspice::_Custom(..))
				| Self::Mage(Path::_Custom(..))
				| Self::Changeling(Seeming::_Custom(..))
				| Self::Bound(Burden::_Custom(..))
		)
	}
}

impl YSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Covenant::_Custom(name))
			| Self::Werewolf(Tribe::_Custom(name, ..))
			| Self::Mage(
				Order::_Custom(name, ..)
				| Order::SeersOfTheThrone(Some(Ministry::_Custom(name, ..))),
			)
			| Self::Changeling(Court::_Custom(name))
			| Self::Bound(Archetype::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			YSplat::Vampire(Covenant::_Custom(..))
				| YSplat::Werewolf(Tribe::_Custom(..))
				| YSplat::Mage(
					Order::_Custom(..) | Order::SeersOfTheThrone(Some(Ministry::_Custom(..))),
				) | YSplat::Changeling(Court::_Custom(..))
				| Self::Bound(Archetype::_Custom(..))
		)
	}
}

impl ZSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Vampire(Bloodline::_Custom(name, ..))
			| ZSplat::Werewolf(Lodge::_Custom(name))
			| ZSplat::Mage(Legacy::_Custom(name, ..))
			| ZSplat::Changeling(Kith::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			ZSplat::Vampire(Bloodline::_Custom(..))
				| ZSplat::Werewolf(Lodge::_Custom(..))
				| ZSplat::Mage(Legacy::_Custom(..))
				| ZSplat::Changeling(Kith::_Custom(..))
		)
	}
}
