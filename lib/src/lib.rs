use crate::splat::Splat;
use cofd_schema::prelude::Attribute;
use cofd_schema::splat::werewolf::Form;
use cofd_schema::traits::Trait;
use std::collections::HashMap;
use std::sync::Arc;

pub mod wound;
// mod character;
mod splat;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(n: &u16) -> bool {
	*n == 0
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ModifierKey {
	Form(Form),

	Trait(Trait),
}

pub struct CofDSystem {
	inner: sos::System<Splat, Trait, ModifierKey>,
}

impl CofDSystem {
	pub fn new() -> Self {
		CofDSystem {
			inner: sos::System::new(HashMap::from([
				(
					Splat::Mage,
					Arc::new(Splat::Mage.create_attributes().build()),
				),
				(
					Splat::Mortal,
					Arc::new(Splat::Mortal.create_attributes().build()),
				),
			])),
		}
	}

	pub fn new_character(&self, splat: &Splat) -> CofDActor {
		CofDActor {
			inner: self.inner.new_actor(&splat),
		}
	}
}

pub struct CofDActor {
	inner: sos::prelude::Actor<Trait, ModifierKey>,
}

impl CofDActor {}

#[cfg(test)]
mod tests {
	use crate::splat::Splat;
	use crate::{CofDSystem, ModifierKey};
	use cofd_schema::prelude::PhysicalAttribute;
	use cofd_schema::splat::werewolf::Form;
	use cofd_schema::traits::DerivedTrait::Speed;
	use cofd_schema::traits::Trait;
	use sos::prelude::{AttributeModifier, Operation};

	#[test]
	pub fn test() {
		let system = CofDSystem::new();

		let mut actor = system.new_character(&Splat::Mortal);

		assert_eq!(
			actor.inner.attributes.value(&Trait::DerivedTrait(Speed)),
			Some(7.0)
		);

		actor.inner.attributes.add_modifier(
			Trait::Attribute(PhysicalAttribute::Strength.into()),
			ModifierKey::Form(Form::Gauru),
			AttributeModifier::new(3.0f32, Operation::Add),
		);

		assert_eq!(
			actor.inner.attributes.value(&Trait::DerivedTrait(Speed)),
			Some(10.0)
		);
	}
}
