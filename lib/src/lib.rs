#![feature(let_chains)]
#![warn(clippy::pedantic)]

#[macro_use]
extern crate cofd_util;

use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};
use strum::VariantArray;
use systema::{
	attribute::modifier::Op,
	prelude::{
		AttributeInstance, AttributeModifier, AttributeSupplier, AttributeSupplierBuilder, System,
		Value,
	},
};

pub mod ability;
pub mod character;
pub mod dice_pool;
pub mod merits;
pub mod splat;
pub mod traits;
mod util;

pub use cofd_schema as schema;
use cofd_schema::{
	prelude::{Attribute, Skill},
	template::{Template, werewolf::Form},
	traits::DerivedTrait,
};
use traits::Trait;

use crate::splat::SplatCharacter;

pub mod prelude {
	pub use cofd_schema::{
		template::Template,
		traits::{
			TraitCategory,
			attribute::{Attribute, AttributeKind},
			skill::Skill,
		},
	};
	pub use cofd_util::{AllVariants, VariantName};
	pub use systema::prelude::Actor;

	pub use crate::{character::Character, splat::SplatTrait, traits::Trait};

	pub mod werewolf {
		pub use crate::splat::werewolf::{
			Werewolf, WerewolfExt, WerewolfMerit,
			schema::{Auspice, Form, ForsakenTribe, Lodge, Renown},
		};
	}
}

#[allow(clippy::too_many_lines)]
fn base_attributes() -> AttributeSupplierBuilder<Trait, Modifier, u8, COp> {
	let mut builder = AttributeSupplier::builder();

	for attribute in Attribute::VARIANTS {
		builder = builder.add(
			Trait::Attribute(*attribute),
			AttributeInstance::builder(systema::prelude::Attribute::Value(1)),
		);
	}

	for skill in Skill::VARIANTS {
		builder = builder.add(
			Trait::Skill(*skill),
			AttributeInstance::builder(systema::prelude::Attribute::Value(0)),
		);
	}

	builder = builder.add(
		Trait::Size,
		AttributeInstance::builder(systema::prelude::Attribute::Value(5)),
	);

	builder
		.add(
			Trait::DerivedTrait(DerivedTrait::Speed),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Dexterity)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Dexterity)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Strength)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Strength)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::DerivedTrait(DerivedTrait::Speed)),
					AttributeModifier::new(Value::Value(5), COp::Add).base(),
				),
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Defense),
			AttributeInstance::builder(systema::prelude::Attribute::Derived), // Defense
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Initiative),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Dexterity)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Dexterity)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						COp::Add,
					)
					.base(),
				),
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Perception),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Wits)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Wits)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						COp::Add,
					)
					.base(),
				),
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Health),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Stamina)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Stamina)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Size),
					AttributeModifier::new(Value::Attribute(Trait::Size), COp::Add).base(),
				),
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Willpower),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Resolve)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Resolve)),
						COp::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						COp::Add,
					)
					.base(),
				),
		)
}

type SupplierMap = HashMap<Template, Arc<AttributeSupplier<Trait, Modifier, u8, COp>>>;

fn splat_attribute_builder(
	template: Template,
) -> AttributeSupplierBuilder<Trait, Modifier, u8, COp> {
	let mut builder = base_attributes();

	if let Some(st) = template.supernatural_tolerance() {
		builder = builder.add(
			Trait::SupernaturalTolerance(st),
			AttributeInstance::builder(systema::prelude::Attribute::Value(1)),
		);
	}

	match template {
		Template::Werewolf => {
			// builder = builder.add(Trait::);
		}
		_ => todo!(),
	}

	builder
}

fn splat_attributes(template: Template) -> Arc<AttributeSupplier<Trait, Modifier, u8, COp>> {
	static ATTRIBUTES: LazyLock<Mutex<SupplierMap>> = LazyLock::new(|| Mutex::new(HashMap::new()));

	ATTRIBUTES
		.lock()
		.unwrap()
		.entry(template)
		.or_insert_with(|| Arc::new(splat_attribute_builder(template).build()))
		.clone()
}

pub struct CofDSystem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modifier {
	Trait(Trait),

	Form(Form),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum COp {
	Add,
	Sub,
	GreaterThan(u8, Box<COp>),
}

impl Op<u8> for COp {
	fn apply(&self, a: u8, b: u8) -> u8 {
		match self {
			COp::Add => a + b,
			COp::Sub => a - b,
			COp::GreaterThan(v, op) => {
				if a.gt(v) {
					op.apply(a, b)
				} else {
					a
				}
			}
		}
	}
}

impl System for CofDSystem {
	type AttributeKey = Trait;
	type ModifierKey = Modifier;
	type AttributeValue = u8;
	type Operation = COp;
	type Actor = SplatCharacter;
}
