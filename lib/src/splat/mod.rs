use crate::ModifierKey;
use cofd_schema::prelude::Attribute::Physical;
use cofd_schema::prelude::{MentalAttribute, PhysicalAttribute, SocialAttribute};
use cofd_schema::traits::DerivedTrait::{Health, Speed};
use cofd_schema::traits::{DerivedTrait, Trait};
use sos::prelude::*;
use std::sync::Arc;

mod mage;

#[derive(Eq, PartialEq, Hash)]
pub enum Splat {
	// #[strum(to_string = "Mortal", serialize = "Human")]
	Mortal,
	// #[strum(to_string = "Mage", serialize = "Awakened")]
	Mage, /*{
			  path: Path,
			  order: Order,
			  attribute_bonus: Attribute,
			  obsessions: Vec<String>,
			  rotes: Vec<Rote>,
		  }*/
	// #[strum(to_string = "Vampire", serialize = "Kindred")]
	Vampire,
	Werewolf,
	Promethean,
	Changeling,
	Hunter,
	Bound,
	Mummy,
	Demon,
	Beast,
	Deviant,
}

impl Splat {
	pub fn create_attributes(self) -> AttributeSupplierBuilder<Trait, ModifierKey> {
		match self {
			Splat::Mortal => {
				let mut health = AttributeInstance::default();
				health.add_modifier(
					ModifierKey::Trait(Trait::Attribute(Physical(PhysicalAttribute::Stamina))),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Physical(PhysicalAttribute::Stamina))),
						Operation::Add,
					),
				);
				health.add_modifier(
					ModifierKey::Trait(Trait::DerivedTrait(DerivedTrait::Size)),
					AttributeModifier::new(
						Value::Attribute(Trait::DerivedTrait(DerivedTrait::Size)),
						Operation::Add,
					),
				);

				let mut speed = AttributeInstance::new(Arc::new(Attribute::Value(5.0)));
				speed.add_modifier(
					ModifierKey::Trait(Trait::Attribute(Physical(PhysicalAttribute::Strength))),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Physical(PhysicalAttribute::Strength))),
						Operation::Add,
					),
				);
				speed.add_modifier(
					ModifierKey::Trait(Trait::Attribute(Physical(PhysicalAttribute::Dexterity))),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Physical(PhysicalAttribute::Dexterity))),
						Operation::Add,
					),
				);

				AttributeSupplier::builder()
					.add(
						Trait::Attribute(MentalAttribute::Intelligence.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(MentalAttribute::Wits.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(MentalAttribute::Resolve.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(PhysicalAttribute::Strength.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(PhysicalAttribute::Dexterity.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(PhysicalAttribute::Stamina.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(SocialAttribute::Presence.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(SocialAttribute::Manipulation.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add(
						Trait::Attribute(SocialAttribute::Composure.into()),
						Attribute::Ranged(1.0, 1.0, 255.0),
					)
					.add_instance(Trait::DerivedTrait(Health), health)
					.add_instance(Trait::DerivedTrait(Speed), speed)
			}

			_ => Splat::Mortal.create_attributes(),
		}
	}
}
