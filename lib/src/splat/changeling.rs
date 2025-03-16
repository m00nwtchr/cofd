pub use cofd_schema::template::changeling as schema;
use derive_more::{From, TryInto};
use serde::{Deserialize, Serialize};
use strum::VariantArray;

use super::{Merit, SplatTrait, XSplat, YSplat, ZSplat};
use crate::{
	character::{Character, damage::Damage},
	prelude::*,
};

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum Regalia {
	Base(schema::Regalia),
	Custom(String),
}

impl Changeling {
	#[must_use]
	pub fn new(seeming: Seeming) -> Self {
		Self {
			seeming,
			..Default::default()
		}
	}

	// pub fn max_clarity(&self, attributes: &Attributes) -> u16 {
	// 	attributes.wits + attributes.composure
	// }

	#[must_use]
	pub fn attr_bonus(&self) -> Attribute {
		self.attr_bonus
	}

	pub fn set_attr_bonus(&mut self, attribute: Attribute) {
		if self.seeming.favored_attributes().contains(&attribute) {
			self.attr_bonus = attribute;
		}
	}
}

impl SplatTrait for Changeling {
	fn template(&self) -> Template {
		Template::Changeling
	}

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
		// Seeming::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Court::all().into_iter().map(Into::into).collect()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Kith::all().into_iter().map(Into::into).collect()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(
			Seeming::Custom {
				name,
				regalia: Regalia::from(schema::Regalia::Crown),
				// AttributeType::Power
			}
			.into(),
		)
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(Court::Custom(name).into())
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(Kith::Custom(name).into())
	}

	fn merits(&self) -> Vec<Merit> {
		ChangelingMerit::all().map(Into::into).to_vec()
	}
}

impl Default for Changeling {
	fn default() -> Self {
		let seeming: schema::Seeming = schema::Seeming::default();
		let attr_bonus = seeming.favored_attributes()[0];
		let regalia = schema::Regalia::VARIANTS
			.iter()
			.find(|f| !seeming.favored_regalia().eq(f))
			.copied()
			.map(Regalia::Base)
			.unwrap();

		Self {
			seeming: Seeming::Base(seeming),
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
	Clone, Serialize, Deserialize, Debug, PartialEq, Eq, derive_more::From, derive_more::TryInto,
)]
pub enum Seeming {
	Base(schema::Seeming),
	Custom {
		name: String,
		regalia: Regalia,
		// category: AttributeType
	},
}

impl Seeming {
	#[must_use]
	pub fn favored_regalia(&self) -> Regalia {
		match self {
			Self::Base(s) => s.favored_regalia().into(),
			Self::Custom { regalia, .. } => regalia.clone(),
		}
	}

	#[must_use]
	pub fn favored_attributes(&self) -> [Attribute; 3] {
		todo!()
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Court {
	Spring,
	Summer,
	Autumn,
	Winter,
	Custom(String),
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
	Custom(String),
}

#[derive(
	Clone,
	Debug,
	PartialEq,
	Eq,
	Serialize,
	Deserialize,
	Hash,
	AllVariants,
	VariantName,
	derive_more::Display,
)]
pub enum ChangelingMerit {
	Mantle,
}

impl ChangelingMerit {
	#[must_use]
	pub fn is_available(&self, character: &Character<Changeling>) -> bool {
		true
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
