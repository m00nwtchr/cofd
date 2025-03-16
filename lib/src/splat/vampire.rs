use cofd_schema::template::Template;
use cofd_util::{AllVariants, VariantName};
use derive_more::{From, TryInto};
use serde::{Deserialize, Serialize};

use super::{Merit, SplatTrait, XSplat, YSplat, ZSplat, ability::Ability};
use crate::{
	dice_pool::DicePool,
	prelude::Attribute,
	splat::ability::{AbilityTrait, CModifier},
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Vampire {
	pub clan: Clan,
	pub covenant: Option<Covenant>,
	pub bloodline: Option<Bloodline>,

	attr_bonus: Attribute,
	pub banes: Vec<String>,
}

impl Vampire {
	#[must_use]
	pub fn new(clan: Clan) -> Self {
		Vampire {
			clan,
			..Default::default()
		}
	}

	#[must_use]
	pub fn attr_bonus(&self) -> Attribute {
		self.attr_bonus
	}

	pub fn set_attr_bonus(&mut self, attribute: Attribute) {
		if self.clan.favored_attributes().contains(&attribute) {
			self.attr_bonus = attribute;
		}
	}

	#[must_use]
	pub fn with_attr_bonus(mut self, attribute: Attribute) -> Self {
		self.set_attr_bonus(attribute);
		self
	}
}

impl SplatTrait for Vampire {
	fn template(&self) -> Template {
		Template::Hunter
	}

	fn set_xsplat(&mut self, splat: Option<XSplat>) {
		if let Some(XSplat::Clan(clan)) = splat {
			self.clan = clan;
		}
	}

	fn set_ysplat(&mut self, splat: Option<YSplat>) {
		match splat {
			Some(YSplat::Covenant(covenant)) => self.covenant = Some(covenant),
			_ => self.covenant = None,
		}
	}

	fn set_zsplat(&mut self, splat: Option<ZSplat>) {
		match splat {
			Some(ZSplat::Bloodline(bloodline)) => self.bloodline = Some(bloodline),
			_ => self.bloodline = None,
		}
	}

	fn xsplat(&self) -> Option<XSplat> {
		Some(self.clan.clone().into())
	}

	fn ysplat(&self) -> Option<YSplat> {
		self.covenant.clone().map(Into::into)
	}

	fn zsplat(&self) -> Option<ZSplat> {
		self.bloodline.clone().map(Into::into)
	}

	fn xsplats(&self) -> Vec<XSplat> {
		// Clan::all().into_iter().map(Into::into).collect()
		todo!()
	}

	fn ysplats(&self) -> Vec<YSplat> {
		Covenant::all().into_iter().map(Into::into).collect()
	}

	fn zsplats(&self) -> Vec<ZSplat> {
		Bloodline::all().into_iter().map(Into::into).collect()
	}

	fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		Some(
			Clan::Custom {
				name,
				disciplines: Box::new([
					Discipline::Animalism,
					Discipline::Auspex,
					Discipline::Celerity,
				]),
				favored_attributes: [Attribute::Composure, Attribute::Dexterity],
			}
			.into(),
		)
	}

	fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		Some(Covenant::Custom(name).into())
	}

	fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		Some(Bloodline::Custom(name, None).into())
	}

	fn all_abilities(&self) -> Option<Vec<Ability>> {
		Some(Discipline::all().into_iter().map(Into::into).collect())
	}

	fn custom_ability(&self, name: String) -> Option<Ability> {
		Some(Discipline::Custom(name).into())
	}

	fn merits(&self) -> Vec<Merit> {
		VampireMerit::all().map(Into::into).to_vec()
	}
}

impl Default for Vampire {
	fn default() -> Self {
		let clan = Clan::Base(cofd_schema::template::vampire::Clan::default());
		let attr_bonus = clan.favored_attributes()[0];
		Vampire {
			clan,
			covenant: None,
			bloodline: None,
			banes: Vec::new(),
			attr_bonus,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, From, TryInto)]
pub enum Clan {
	Base(cofd_schema::template::vampire::Clan),
	Custom {
		name: String,
		disciplines: Box<[Discipline; 3]>,
		favored_attributes: [Attribute; 2],
	},
}

impl Clan {
	#[must_use]
	pub fn disciplines(&self) -> [Discipline; 3] {
		match self {
			Self::Base(b) => match b {
				cofd_schema::template::vampire::Clan::Daeva => {
					[Discipline::Celerity, Discipline::Majesty, Discipline::Vigor]
				}
				cofd_schema::template::vampire::Clan::Gangrel => [
					Discipline::Animalism,
					Discipline::Protean,
					Discipline::Resilience,
				],
				cofd_schema::template::vampire::Clan::Mekhet => [
					Discipline::Auspex,
					Discipline::Celerity,
					Discipline::Obfuscate,
				],
				cofd_schema::template::vampire::Clan::Nosferatu => [
					Discipline::Nightmare,
					Discipline::Obfuscate,
					Discipline::Vigor,
				],
				cofd_schema::template::vampire::Clan::Ventrue => [
					Discipline::Animalism,
					Discipline::Dominate,
					Discipline::Resilience,
				],
			},
			Self::Custom { disciplines, .. } => disciplines.as_ref().clone(),
		}
	}

	#[must_use]
	pub fn favored_attributes(&self) -> [Attribute; 2] {
		match self {
			Self::Base(b) => b.favored_attributes(),
			Self::Custom {
				favored_attributes, ..
			} => *favored_attributes,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Covenant {
	CarthianMovement,
	CircleOfTheCrone,
	Invictus,
	LanceaEtSanctum,
	OrdoDracul,
	Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Bloodline {
	Custom(String, Option<[Discipline; 4]>),
}

#[derive(
	Clone,
	Debug,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	Hash,
	AllVariants,
	VariantName,
)]
pub enum Discipline {
	Animalism,
	Auspex,
	Celerity,
	Dominate,
	Majesty,
	Nightmare,
	Obfuscate,
	Protean,
	Resilience,
	Vigor,
	Custom(String),
}

impl Discipline {
	// #[warn(clippy::cast_possible_wrap)]
	// pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
	// 	match self {
	// 		Discipline::Celerity => {
	// 			vec![Modifier::new(
	// 				Trait::DerivedTrait(DerivedTrait::Defense),
	// 				value,
	// 				ModifierOp::Add,
	// 			)]
	// 		}
	// 		Discipline::Resilience => {
	// 			vec![Modifier::new(Attribute::Stamina, value, ModifierOp::Add)]
	// 		}
	// 		Discipline::Vigor => vec![Modifier::new(Attribute::Strength, value, ModifierOp::Add)],
	// 		_ => vec![],
	// 	}
	// }
}

impl AbilityTrait for Discipline {
	fn get_modifiers(&self) -> Box<[CModifier]> {
		Box::default()
	}
}

pub enum MaskDirge {
	Authoritarian,
	Child,
	Competitor,
	Conformist,
	Conspirator,
	Courtesan,
	CultLeader,
	Deviant,
	Follower,
	Guru,
	Idealist,
	Jester,
	Junkie,
	Martyr,
	Masochist,
	Monster,
	Nomad,
	Nurturer,
	Perfectionist,
	Penitent,
	Questioner,
	Rebel,
	Scholar,
	SocialChameleon,
	Spy,
	Survivor,
	Visionary,
	Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
pub enum VampireMerit {
	AcuteSenses,
	Atrocious,
	Beloved, // TY
	Bloodhound,
	CallTheBeast, // TY
	ClawsOfTheUnholy,
	CloseFamily,
	Cutthroat,
	DistinguishedPalate,
	DreamVisions, // Mekhet
	Enticing,
	FeedingGrounds(String),
	HeartOfStone, // TY
	Herd,
	HoneyTrap,
	KindredStatus(String), // Status?
	KissOfTheSuccubus,
	Lineage(String),
	LingeringDreams, // DE2
	MajorDomo,       // TY
	MarriedByBlood,  // TY
	PackAlpha,
	ReceptiveMind,    // TY
	RevenantImpostor, // HD
	SaviorOfTheLost,  // TY
	SpecialTreat,     // TY
	SwarmForm,
	Touchstone,
	UndeadMenses,
	UnnaturalAffinity,
	UnsettlingGaze,

	CacophonySavvy,
	Courtoisie,
	Crusade,
	DynastyMembership,
	KindredDueling,
	MobilizeOutrage, // SotC, Carthian
	RidingTheWave,
	RitesOfTheImpaled, // SotC, Ordo, Sworn
	TempleGuardian,    // SotC, Crone

	// Elder Merits,

	// Revenant Merits

	// Covenant Merits

	// Ordo Dracul
	IndependentStudy, // SotC
	SecretSocietyJunkie,
	Sworn(String),
	TwilightJudge, // SotC

	NestGuardian,
}

impl VampireMerit {
	// pub fn is_available(
	// 	&self,
	// 	character: &crate::prelude::Character,
	// 	attributes: &Attributes,
	// 	skills: &Skills,
	// ) -> bool {
	// 	matches!(character.splat, Splat::Vampire(..))
	// 		&& match self {
	// 			// VampireMerit::Atrocious => todo!(), // Not Enticing or Cutthroat
	// 			VampireMerit::Bloodhound => attributes.wits >= 3,
	// 			// VampireMerit::CallTheBeast => character.integrity < 5,
	// 			// VampireMerit::ClawsOfTheUnholy => {
	// 			// 	*character
	// 			// 		.abilities
	// 			// 		.get(&Discipline::Protean.into())
	// 			// 		.unwrap_or(&0) >= 4
	// 			// }
	// 			// VampireMerit::Cutthroat => todo!(), // Not Enticing or Atrocious
	// 			VampireMerit::DreamVisions => {
	// 				matches!(
	// 					character.splat,
	// 					Splat::Vampire(Vampire {
	// 						clan: Clan::Mekhet,
	// 						..
	// 					})
	// 				)
	// 			}
	// 			// VampireMerit::Enticing => todo!(), // Not Cutthroat or Atrocious
	// 			// VampireMerit::FeedingGrounds(_) => todo!(),
	// 			// VampireMerit::HeartOfStone => todo!(), // Feeding Grounds
	// 			// VampireMerit::HoneyTrap => todo!(), // Not a Revenant
	// 			// VampireMerit::KindredStatus(_) => todo!(),
	// 			VampireMerit::KissOfTheSuccubus => {
	// 				matches!(
	// 					character.splat,
	// 					Splat::Vampire(Vampire {
	// 						clan: Clan::Daeva,
	// 						..
	// 					})
	// 				)
	// 			}
	// 			// VampireMerit::Lineage(_) => todo!(), Clan Status
	// 			VampireMerit::LingeringDreams => {
	// 				matches!(
	// 					character.splat,
	// 					Splat::Vampire(Vampire {
	// 						clan: Clan::Mekhet,
	// 						..
	// 					})
	// 				)
	// 			} // Dream Visions
	// 			VampireMerit::PackAlpha => {
	// 				matches!(
	// 					character.splat,
	// 					Splat::Vampire(Vampire {
	// 						clan: Clan::Gangrel,
	// 						..
	// 					})
	// 				)
	// 			}
	// 			// VampireMerit::ReceptiveMind => {
	// 			// 	character.power >= 6
	// 			// 		&& *character
	// 			// 			.abilities
	// 			// 			.get(&Discipline::Auspex.into())
	// 			// 			.unwrap_or(&0) >= 4
	// 			// }
	// 			VampireMerit::RevenantImpostor => {
	// 				attributes.manipulation >= 3 && skills.subterfuge >= 2
	// 			}
	// 			// VampireMerit::SwarmForm => {
	// 			// 	*character
	// 			// 		.abilities
	// 			// 		.get(&Discipline::Protean.into())
	// 			// 		.unwrap_or(&0) >= 4
	// 			// }
	// 			VampireMerit::UnsettlingGaze => {
	// 				matches!(
	// 					character.splat,
	// 					Splat::Vampire(Vampire {
	// 						clan: Clan::Nosferatu,
	// 						..
	// 					})
	// 				)
	// 			}
	//
	// 			// VampireMerit::CacophonySavvy => todo!(), // City Status
	// 			VampireMerit::Courtoisie => {
	// 				attributes.composure >= 3 && skills.socialize >= 2 && skills.weaponry >= 2
	// 			} // Invictus Status
	// 			VampireMerit::Crusade => {
	// 				attributes.resolve >= 3 && skills.occult >= 2 && skills.weaponry >= 2
	// 			} // Theban Sorcery 2 or Sorc Eunuch
	// 			// VampireMerit::DynastyMembership => todo!(), // Clan Status
	// 			VampireMerit::KindredDueling => attributes.composure >= 3 && skills.weaponry >= 2,
	// 			// VampireMerit::MobilizeOutrage => {
	// 			// 	character.max_willpower() >= 5 && skills.brawl >= 2
	// 			// } // Carthian Status
	// 			VampireMerit::RidingTheWave => attributes.composure >= 3 && attributes.resolve >= 3,
	// 			VampireMerit::RitesOfTheImpaled => {
	// 				attributes.resolve >= 3 && attributes.stamina >= 3 && skills.weaponry >= 2
	// 			} // Sworn
	// 			VampireMerit::TempleGuardian => {
	// 				skills.athletics >= 2 && skills.brawl >= 2 && skills.weaponry >= 2
	// 			} // Crone Status
	// 			// VampireMerit::IndependentStudy => todo!(),
	// 			// VampireMerit::SecretSocietyJunkie => todo!(),
	// 			// VampireMerit::Sworn(_) => todo!(),
	// 			// VampireMerit::TwilightJudge => todo!(),
	// 			// VampireMerit::NestGuardian => todo!(),
	// 			_ => true,
	// 		}
	// }

	// pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
	// 	Vec::new()
	// }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devotion {
	name: String,
	cost: String,
	disciplines: Vec<(Discipline, u16)>,
	dice_pool: DicePool,
	book: String,
	page: u16,
}
