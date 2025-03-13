#![feature(let_chains)]
#![warn(clippy::pedantic)]
#![allow(
	clippy::must_use_candidate,
	clippy::used_underscore_binding,
	clippy::unused_self,
	clippy::match_wildcard_for_single_variants,
	clippy::module_name_repetitions,
	clippy::wildcard_imports,
	clippy::match_same_arms,
	clippy::default_trait_access
)]

#[macro_use]
extern crate cofd_util;

use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use systema::prelude::{
	AttributeInstance, AttributeMap, AttributeModifier, AttributeSupplier,
	AttributeSupplierBuilder, Operation, System, Value,
};

pub mod character;
pub mod dice_pool;
pub mod splat;
pub mod traits;
mod util;

pub use cofd_schema::template;
use cofd_schema::{
	prelude::{Attribute, Skill},
	template::{SupernaturalTolerance, Template},
	traits::{DerivedTrait, Trait},
};
use once_cell::sync::Lazy;
use strum::VariantArray;

use crate::{prelude::Character, splat::werewolf::Form};

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

	pub use crate::{character::Character, splat::SplatTrait};
}

fn base_attributes() -> AttributeSupplierBuilder<Trait, Modifier, u8> {
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
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Strength)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Strength)),
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::DerivedTrait(DerivedTrait::Speed)),
					AttributeModifier::new(Value::Value(5), Operation::Add).base(),
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
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						Operation::Add,
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
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						Operation::Add,
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
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Size),
					AttributeModifier::new(Value::Attribute(Trait::Size), Operation::Add).base(),
				),
		)
		.add(
			Trait::DerivedTrait(DerivedTrait::Willpower),
			AttributeInstance::builder(systema::prelude::Attribute::Derived)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Resolve)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Resolve)),
						Operation::Add,
					)
					.base(),
				)
				.modifier(
					Modifier::Trait(Trait::Attribute(Attribute::Composure)),
					AttributeModifier::new(
						Value::Attribute(Trait::Attribute(Attribute::Composure)),
						Operation::Add,
					)
					.base(),
				),
		)
}

type SupplierMap = HashMap<Template, Arc<AttributeSupplier<Trait, Modifier, u8>>>;

fn splat_attribute_builder(template: Template) -> AttributeSupplierBuilder<Trait, Modifier, u8> {
	let mut builder = base_attributes();

	if let Some(st) = template.supernatural_tolerance() {
		builder = builder.add(
			Trait::SupernaturalTolerance(st),
			AttributeInstance::builder(systema::prelude::Attribute::Value(1)),
		);
	}

	builder
}

pub fn splat_attributes(template: Template) -> Arc<AttributeSupplier<Trait, Modifier, u8>> {
	static ATTRIBUTES: Lazy<Mutex<SupplierMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

	ATTRIBUTES
		.lock()
		.unwrap()
		.entry(template)
		.or_insert_with(|| Arc::new(splat_attribute_builder(template).build()))
		.clone()
}

pub struct CofDSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modifier {
	Trait(Trait),

	Form(Form),
}

impl System for CofDSystem {
	type AttributeKey = Trait;
	type ModifierKey = Modifier;
	type AttributeValue = u8;
	type Actor = Character;
}
