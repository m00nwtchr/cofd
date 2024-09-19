use serde::{Deserialize, Serialize};

use super::{Merit, NameKey, Splat, SplatTrait, XSplat, YSplat, ZSplat};
use crate::{
	character::Character,
	prelude::*,
};
use crate::character::health::Damage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Changeling {
	pub seeming: Seeming,
	pub court: Option<Court>,
	pub kith: Option<Kith>,

	attr_bonus: Attribute,
	pub regalia: Regalia,
	pub frailties: Vec<String>,
	pub clarity: Damage,
	pub contracts: Vec<Contract>,
}

impl Changeling {
	pub fn new(seeming: Seeming) -> Self {
		Self {
			seeming,
			..Default::default()
		}
	}

	pub fn max_clarity(&self, attributes: &Attributes) -> u8 {
		attributes.wits + attributes.composure
	}

	pub fn attr_bonus(&self) -> &Attribute {
		&self.attr_bonus
	}

	pub fn set_attr_bonus(&mut self, attribute: Attribute) {
		if self.seeming.favored_attributes().contains(&attribute) {
			self.attr_bonus = attribute;
		}
	}
}

impl SplatTrait for Changeling {
	fn set_xsplat(&mut self, splat: Option<XSplat>) {
		if let Some(XSplat::Seeming(seeming)) = splat {
			self.seeming = seeming;
		}
	}

	fn set_ysplat(&mut self, splat: Option<YSplat>) {
		match splat {
			Some(YSplat::Court(court)) => self.court = Some(court),
			_ => self.court = None,
		}
	}

	fn set_zsplat(&mut self, splat: Option<ZSplat>) {
		match splat {
			Some(ZSplat::Kith(kith)) => self.kith = Some(kith),
			_ => self.kith = None,
		}
	}

	fn xsplat(&self) -> Option<XSplat> {
		Some(self.seeming.clone().into())
	}

	fn ysplat(&self) -> Option<YSplat> {
		self.court.clone().map(Into::into)
	}

	fn zsplat(&self) -> Option<ZSplat> {
		self.kith.clone().map(Into::into)
	}

	fn xsplats(&self) -> Vec<XSplat> {
		Seeming::all().into_iter().map(Into::into).collect()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Court::all().into_iter().map(Into::into).collect()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Kith::all().into_iter().map(Into::into).collect()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(
			Seeming::_Custom(
				name,
				Regalia::Crown,
				// AttributeType::Power
			)
			.into(),
		)
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(Court::_Custom(name).into())
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(Kith::_Custom(name).into())
	}

	fn merits(&self) -> Vec<Merit> {
		ChangelingMerit::all().map(Into::into).to_vec()
	}
}

impl Default for Changeling {
	fn default() -> Self {
		let seeming = Seeming::default();
		let attr_bonus = seeming.favored_attributes()[0];
		let regalia = Regalia::all()
			.into_iter()
			.find(|f| !seeming.get_favored_regalia().eq(f))
			.unwrap();

		Self {
			seeming: Seeming::default(),
			court: None,
			kith: None,

			attr_bonus,
			regalia,
			frailties: Vec::new(),
			clarity: Damage::default(),
			contracts: Vec::new(),
		}
	}
}

#[derive(
	Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants, Default,
)]
pub enum Seeming {
	#[default]
	Beast,
	Darkling,
	Elemental,
	Fairest,
	Ogre,
	Wizened,
	_Custom(
		String,
		Regalia,
		// AttributeType
	),
}

impl Seeming {
	pub fn get_favored_regalia(&self) -> &Regalia {
		match self {
			Seeming::Beast => &Regalia::Steed,
			Seeming::Darkling => &Regalia::Mirror,
			Seeming::Elemental => &Regalia::Sword,
			Seeming::Fairest => &Regalia::Crown,
			Seeming::Ogre => &Regalia::Shield,
			Seeming::Wizened => &Regalia::Jewels,
			Seeming::_Custom(_, regalia, ..) => regalia,
		}
	}

	pub fn favored_attributes(&self) -> [Attribute; 3] {
		// Attribute::get(AttributeCategory::Type(match self {
		// 	Seeming::Beast => AttributeType::Resistance,
		// 	Seeming::Darkling => AttributeType::Finesse,
		// 	Seeming::Elemental => AttributeType::Resistance,
		// 	Seeming::Fairest => AttributeType::Power,
		// 	Seeming::Ogre => AttributeType::Power,
		// 	Seeming::Wizened => AttributeType::Finesse,
		// 	Seeming::_Custom(.., _type) => _type.clone(),
		// }))
		// TODO
		[
			Attribute::Intelligence,
			Attribute::Intelligence,
			Attribute::Intelligence,
		]
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Court {
	Spring,
	Summer,
	Autumn,
	Winter,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Kith {
	Artist,
	BrightOne,
	Chatelane,
	Gristlegrinder,
	Helldiver,
	Hunterheart,
	Leechfinger,
	Mirrorskin,
	Nightsinger,
	Notary,
	Playmate,
	Snowskin,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Regalia {
	Crown,
	Jewels,
	Mirror,
	Shield,
	Steed,
	Sword,
	_Custom(String),
}

impl NameKey for Regalia {
	fn name_key(&self) -> String {
		format!("changeling.{}", self.name())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
pub enum ChangelingMerit {
	Mantle,
}

impl ChangelingMerit {
	pub fn is_available(&self, character: &Character) -> bool {
		matches!(character.splat, Splat::Changeling(..))
	}
}

impl From<ChangelingMerit> for Merit {
	fn from(merit: ChangelingMerit) -> Self {
		Merit::Changeling(merit)
	}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contract {
	pub name: String,
	pub goblin: bool,
	pub cost: String,
	pub dice: String,
	pub action: String,
	pub duration: String,
	pub loophole: String,
	pub seeming_benefit: String,
}
