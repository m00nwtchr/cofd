use cofd_schema::template::Template;
pub use cofd_schema::template::mage as schema;
use cofd_util::{AllVariants, VariantName};
use derive_more::{From, TryInto};
use schema::{Arcanum, Legacy, Order, Rote};
use serde::{Deserialize, Serialize};

use super::{Merit, SplatTrait, XSplat, YSplat, ZSplat};
use crate::{
	ability::{Ability, AbilityTrait},
	prelude::{Attribute, Character, Skill},
};
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Mage {
	pub path: Path,
	pub order: Option<Order>, // TODO: Some(Order) = free order status, high speech merit
	pub legacy: Option<Legacy>,

	free_resistance_dot: Attribute,
	pub obsessions: Vec<String>,
	pub rotes: Vec<Rote>,
}

impl Mage {
	#[must_use]
	pub fn new(path: Path) -> Self {
		Self {
			path,
			..Default::default()
		}
	}

	#[must_use]
	pub fn with_order(mut self, order: Order) -> Self {
		self.order = Some(order);
		self
	}

	#[must_use]
	pub fn with_legacy(mut self, legacy: Legacy) -> Self {
		self.legacy = Some(legacy);
		self
	}

	#[must_use]
	pub fn with_attr_bonus(mut self, attribute: Attribute) -> Self {
		self.set_attr_bonus(attribute);
		self
	}

	#[must_use]
	pub fn with_obsessions(mut self, obsessions: Vec<String>) -> Self {
		self.obsessions = obsessions;
		self
	}

	#[must_use]
	pub fn with_rotes(mut self, rotes: Vec<Rote>) -> Self {
		self.rotes = rotes;
		self
	}

	#[must_use]
	pub fn attr_bonus(&self) -> Attribute {
		self.free_resistance_dot
	}

	pub fn set_attr_bonus(&mut self, attribute: Attribute) {
		if matches!(
			attribute,
			Attribute::Resolve | Attribute::Stamina | Attribute::Composure
		) {
			self.free_resistance_dot = attribute;
		}
	}
}

impl SplatTrait for Mage {
	fn template(&self) -> Template {
		Template::Mage
	}

	fn set_xsplat(&mut self, splat: Option<XSplat>) {
		if let Some(XSplat::Path(path)) = splat {
			self.path = path;
		}
	}

	fn set_ysplat(&mut self, splat: Option<YSplat>) {
		match splat {
			Some(YSplat::Order(order)) => self.order = Some(order),
			_ => self.order = None,
		}
	}

	fn set_zsplat(&mut self, splat: Option<ZSplat>) {
		match splat {
			Some(ZSplat::Legacy(legacy)) => self.legacy = Some(legacy),
			_ => self.legacy = None,
		}
	}

	fn xsplat(&self) -> Option<XSplat> {
		Some(self.path.clone().into())
	}

	fn ysplat(&self) -> Option<YSplat> {
		self.order.clone().map(Into::into)
	}

	fn zsplat(&self) -> Option<ZSplat> {
		self.legacy.clone().map(Into::into)
	}

	fn xsplats(&self) -> Vec<XSplat> {
		// Path::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		// Order::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Vec::default()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(
			Path::Custom {
				name,
				ruling_arcana: [Arcanum::Death, Arcanum::Fate],
				inferior_arcanum: Arcanum::Forces,
			}
			.into(),
		)
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(
			Order::Custom {
				name,
				rote_skills: [Skill::Academics, Skill::AnimalKen, Skill::Athletics],
			}
			.into(),
		)
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(
			Legacy {
				name,
				ruling_arcanum: Arcanum::Forces,
			}
			.into(),
		)
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		Some(Arcanum::all().iter().copied().map(Into::into).collect())
	}

	fn alternate_beats_optional(&self) -> bool {
		false
	}

	fn merits(&self) -> Vec<Merit> {
		MageMerit::all().map(Into::into).to_vec()
	}
}

impl Default for Mage {
	fn default() -> Self {
		Self {
			path: Path::Base(cofd_schema::template::mage::Path::Acanthus),
			order: None,
			legacy: None,

			free_resistance_dot: Attribute::Resolve,
			obsessions: Vec::new(),
			rotes: Vec::new(),
		}
	}
}

impl AbilityTrait for Arcanum {}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum Path {
	Base(cofd_schema::template::mage::Path),
	Custom {
		name: String,
		ruling_arcana: [Arcanum; 2],
		inferior_arcanum: Arcanum,
	},
}

impl Path {
	#[must_use]
	pub fn ruling_arcana(&self) -> [Arcanum; 2] {
		match self {
			Self::Base(p) => p.ruling_arcana(),
			Self::Custom { ruling_arcana, .. } => *ruling_arcana,
		}
	}

	#[must_use]
	pub fn inferior_arcanum(&self) -> Arcanum {
		match self {
			Self::Base(p) => p.inferior_arcanum(),
			Self::Custom {
				inferior_arcanum, ..
			} => *inferior_arcanum,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, AllVariants, VariantName)]
pub enum MageMerit {
	HighSpeech,
}

impl MageMerit {
	#[must_use]
	pub fn is_available(&self, character: &Character<Mage>) -> bool {
		true
	}
}
