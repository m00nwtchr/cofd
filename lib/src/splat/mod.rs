#![allow(clippy::wildcard_imports)]
use cofd_schema::template::Template;
use cofd_util::AllVariants;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;
use systema::prelude::{Actor, AttributeMap};

use crate::{
	CofDSystem,
	ability::Ability,
	character::{Character, CharacterTrait, info::CharacterInfo},
	merits::*,
};

pub mod mage;
pub mod vampire;
pub mod werewolf;
// pub mod promethean;
pub mod changeling;
// pub mod hunter;
pub mod geist;
pub mod mortal;
// pub mod mummy;
// pub mod demon;
// pub mod beast;
// pub mod deviant;

// use promethean::*;
use changeling::*;
// use hunter::*;
use geist::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;
use mage::{
	schema::{Legacy, Ministry, Order},
	*,
};
use mortal::*;
use vampire::*;
use werewolf::{schema::Lodge, *};

#[derive(
	Clone,
	// PartialEq,
	// Eq,
	Serialize,
	Deserialize,
	// Debug,
	strum::AsRefStr,
	AllVariants,
	EnumDiscriminants,
)]
// #[strum_discriminants(name(SplatKind), derive(strum::VariantArray, strum::AsRefStr))]
#[enum_dispatch(CharacterTrait)]
pub enum SplatCharacter {
	Mortal(Character<Mortal>),
	Vampire(Character<Vampire>),
	Werewolf(Character<Werewolf>),
	Mage(Character<Mage>),
	// Promethean(Lineage),
	Changeling(Character<Changeling>),
	// Hunter(Tier),
	Bound(Character<Bound>),
	// Mummy(Decree, Guild),
	// Demon(Incarnation, Vec<Agenda>),
	// Beast(Hunger),
	// Deviant(Origin, Clade, Vec<Form>),
}

impl Actor for SplatCharacter {
	type System = CofDSystem;
	type Kind = Template;

	fn new(kind: Self::Kind) -> Self {
		match kind {
			Template::Mortal => SplatCharacter::Mortal(Character::default()),
			Template::Vampire => SplatCharacter::Vampire(Character::default()),
			Template::Werewolf => SplatCharacter::Werewolf(Character::default()),
			Template::Mage => SplatCharacter::Mage(Character::default()),
			Template::Changeling => SplatCharacter::Changeling(Character::default()),
			Template::Bound => SplatCharacter::Bound(Character::default()),
			_ => {
				unimplemented!("Character splat not yet implemented")
			}
		}
	}

	fn attributes(&self) -> &AttributeMap<Self::System> {
		match self {
			Self::Mortal(c) => Actor::attributes(c),
			Self::Vampire(c) => Actor::attributes(c),
			Self::Werewolf(c) => Actor::attributes(c),
			Self::Mage(c) => Actor::attributes(c),
			Self::Changeling(c) => Actor::attributes(c),
			Self::Bound(c) => Actor::attributes(c),
		}
	}

	fn attributes_mut(&mut self) -> &mut AttributeMap<Self::System> {
		match self {
			Self::Mortal(c) => Actor::attributes_mut(c),
			Self::Vampire(c) => Actor::attributes_mut(c),
			Self::Werewolf(c) => Actor::attributes_mut(c),
			Self::Mage(c) => Actor::attributes_mut(c),
			Self::Changeling(c) => Actor::attributes_mut(c),
			Self::Bound(c) => Actor::attributes_mut(c),
		}
	}
}

#[enum_dispatch]
pub trait SplatTrait: Default {
	fn template(&self) -> Template;

	#[allow(unused_mut)]
	fn init(mut character: Character<Self>) -> Character<Self> {
		character
	}

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

#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::From, derive_more::TryInto,
)]
pub enum XSplat {
	// #[expand]
	Clan(Clan),
	// #[expand]
	Auspice(Auspice),
	// #[expand]
	Path(Path),
	// #[expand]
	Seeming(Seeming),
	// #[expand]
	Burden(Burden),
}
#[derive(Debug, Clone, PartialEq, Eq, derive_more::From, derive_more::TryInto)]
pub enum YSplat {
	// #[expand]
	Covenant(Covenant),
	// #[expand]
	Tribe(Tribe),
	// #[expand]
	Order(Order),
	// #[expand]
	Court(Court),
	// #[expand]
	Archetype(Archetype),
}
#[derive(Debug, Clone, PartialEq, Eq, derive_more::From, derive_more::TryInto)]
pub enum ZSplat {
	// #[expand]
	Bloodline(Bloodline),
	// #[expand]
	Lodge(Lodge),
	// #[expand]
	Legacy(Legacy),
	// #[expand]
	Kith(Kith),
}

impl XSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Clan(Clan::Custom { name, .. })
			| Self::Auspice(Auspice::Custom(CustomAuspice { name, .. }))
			| Self::Path(Path::Custom { name, .. })
			| Self::Seeming(Seeming::Custom { name, .. })
			| Self::Burden(Burden::Custom { name, .. }) => Some(name),
			_ => None,
		}
	}

	#[must_use]
	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Self::Clan(Clan::Custom { .. })
				| Self::Auspice(Auspice::Custom { .. })
				| Self::Path(Path::Custom { .. })
				| Self::Seeming(Seeming::Custom { .. })
				| Self::Burden(Burden::Custom { .. })
		)
	}
}

impl YSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Covenant(Covenant::Custom(name))
			| Self::Tribe(
				Tribe::Forsaken(ForsakenTribe::Custom { name, .. })
				| Tribe::Pure(PureTribe::Custom { name, .. }),
			)
			| Self::Order(
				Order::Custom { name, .. }
				| Order::SeersOfTheThrone(Some(Ministry::Custom { name, .. })),
			)
			| Self::Court(Court::Custom(name))
			| Self::Archetype(Archetype::Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	#[must_use]
	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			YSplat::Covenant(Covenant::Custom(..))
				| YSplat::Tribe(
					Tribe::Forsaken(ForsakenTribe::Custom { .. })
						| Tribe::Pure(PureTribe::Custom { .. }),
				) | YSplat::Order(
				Order::Custom { .. } | Order::SeersOfTheThrone(Some(Ministry::Custom { .. })),
			) | YSplat::Court(Court::Custom(..))
				| Self::Archetype(Archetype::Custom(..))
		)
	}
}

impl ZSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Bloodline(Bloodline::Custom(name, ..))
			| ZSplat::Lodge(Lodge { name })
			| ZSplat::Legacy(Legacy { name, .. })
			| ZSplat::Kith(Kith::Custom(name)) => Some(name),
			_ => None,
		}
	}

	#[must_use]
	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			ZSplat::Bloodline(Bloodline::Custom(..))
				| ZSplat::Lodge(_)
				| ZSplat::Legacy(_)
				| ZSplat::Kith(Kith::Custom(..))
		)
	}
}
