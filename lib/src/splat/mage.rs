pub use cofd_schema::template::mage::Arcanum;
use cofd_util::{AllVariants, VariantName};
use serde::{Deserialize, Serialize};

use super::{ability::Ability, Merit, Splat, SplatTrait, XSplat, YSplat, ZSplat};
use crate::prelude::{Attribute, Character, Skill};

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

	pub fn attr_bonus(&self) -> &Attribute {
		&self.free_resistance_dot
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
		Path::all().into_iter().map(Into::into).collect()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Order::all().into_iter().map(Into::into).collect()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Legacy::all().into_iter().map(Into::into).collect()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(Path::Custom(name, [Arcanum::Death, Arcanum::Fate], Arcanum::Forces).into())
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(Order::Custom(name, [Skill::Academics, Skill::AnimalKen, Skill::Athletics]).into())
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(Legacy::_Custom(name, None).into())
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		Some(
			Arcanum::all()
				.iter()
				.copied()
				.map(Into::into)
				.collect(),
		)
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
			path: Path::default(),
			order: None,
			legacy: None,

			free_resistance_dot: Attribute::Resolve,
			obsessions: Vec::new(),
			rotes: Vec::new(),
		}
	}
}

#[derive(
	Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants, Default,
)]
pub enum Path {
	#[default]
	Acanthus,
	Mastigos,
	Moros,
	Obrimos,
	Thyrsus,
	Custom(String, [Arcanum; 2], Arcanum),
}

impl Path {
	fn get_ruling_arcana(&self) -> &[Arcanum; 2] {
		match self {
			Path::Acanthus => &[Arcanum::Time, Arcanum::Fate],
			Path::Mastigos => &[Arcanum::Space, Arcanum::Mind],
			Path::Moros => &[Arcanum::Matter, Arcanum::Death],
			Path::Obrimos => &[Arcanum::Forces, Arcanum::Prime],
			Path::Thyrsus => &[Arcanum::Life, Arcanum::Spirit],
			Path::Custom(_, ruling, _) => ruling,
		}
	}
	fn get_inferior_arcanum(&self) -> &Arcanum {
		match self {
			Path::Acanthus => &Arcanum::Forces,
			Path::Mastigos => &Arcanum::Matter,
			Path::Moros => &Arcanum::Spirit,
			Path::Obrimos => &Arcanum::Death,
			Path::Thyrsus => &Arcanum::Mind,
			Path::Custom(_, _, inferior) => inferior,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Order {
	AdamantineArrow,
	GuardiansOfTheVeil,
	Mysterium,
	SilverLadder,
	FreeCouncil,
	#[expand]
	SeersOfTheThrone(Option<Ministry>),
	Custom(String, [Skill; 3]),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Ministry {
	Hegemony,
	Panopticon,
	Paternoster,
	Praetorian,
	Custom(String, [Skill; 3]),
}

impl Order {
	pub fn get_rote_skills(&self) -> &[Skill; 3] {
		match self {
			Order::AdamantineArrow => &[Skill::Athletics, Skill::Intimidation, Skill::Medicine],
			Order::GuardiansOfTheVeil => &[Skill::Investigation, Skill::Stealth, Skill::Subterfuge],
			Order::Mysterium => &[Skill::Investigation, Skill::Occult, Skill::Survival],
			Order::SilverLadder => &[Skill::Expression, Skill::Persuasion, Skill::Subterfuge],
			Order::FreeCouncil => &[Skill::Crafts, Skill::Persuasion, Skill::Science],
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => match ministry {
					Ministry::Hegemony => &[Skill::Politics, Skill::Persuasion, Skill::Empathy],
					Ministry::Panopticon => {
						&[Skill::Investigation, Skill::Stealth, Skill::Subterfuge]
					}
					Ministry::Paternoster => &[Skill::Academics, Skill::Occult, Skill::Expression],
					Ministry::Praetorian => {
						&[Skill::Athletics, Skill::Larceny, Skill::Intimidation]
					}
					Ministry::Custom(_, skills) => skills,
				},
				None => &[Skill::Investigation, Skill::Occult, Skill::Persuasion],
			},
			Order::Custom(_, skills) => skills,
		}
	}
}

impl From<Ministry> for Order {
	fn from(ministry: Ministry) -> Self {
		Order::SeersOfTheThrone(Some(ministry))
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Legacy {
	_Custom(String, Option<Arcanum>),
}

impl From<Arcanum> for Ability {
	fn from(val: Arcanum) -> Self {
		Ability::Arcanum(val)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, AllVariants, VariantName)]
pub enum MageMerit {
	HighSpeech,
}

impl MageMerit {
	pub fn is_available(&self, character: &Character) -> bool {
		matches!(character.splat, Splat::Mage(..))
	}
}

impl From<MageMerit> for Merit {
	fn from(merit: MageMerit) -> Self {
		Merit::Mage(merit)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rote {
	pub arcanum: Arcanum,
	pub level: u16,
	pub spell: String,
	pub creator: String,
	pub skill: Skill,
}
