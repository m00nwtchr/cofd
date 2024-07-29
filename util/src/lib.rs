#![feature(generic_const_exprs)]
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

pub trait AllVariants
where
	Self: Sized,
{
	const N: usize;
	fn all() -> [Self; Self::N];
}
