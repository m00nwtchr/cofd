extern crate cofd_derive;

pub use cofd_derive::*;

pub trait VariantName {
	fn name(&self) -> &str;
}

impl<T> VariantName for Box<T>
where
	T: VariantName,
{
	fn name(&self) -> &str {
		self.as_ref().name()
	}
}

pub trait AllVariants: strum::VariantArray
where
	Self: Clone,
{
	fn all() -> Vec<Self> {
		Self::VARIANTS.to_vec()
	}
}

impl<T> AllVariants for T
where
	T: strum::VariantArray + Clone,
{}