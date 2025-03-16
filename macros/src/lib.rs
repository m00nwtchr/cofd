#![feature(let_chains)]
use std::{env, fs, fs::File, path::Path};

use cofd_schema::{book::Book, item::gift::GiftKind};
use convert_case::Casing;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Error, ItemEnum, parse_macro_input};

macro_rules! derive_error {
	($string: tt) => {
		Error::new(Span::call_site(), $string)
			.to_compile_error()
			.into()
	};
}

fn gift_name_to_id(name: &str) -> &str {
	if name.contains("of") {
		name.split(' ').next_back().unwrap()
	} else {
		let next = name.split(' ').next().unwrap();
		if next.contains('\'') {
			next.strip_suffix("\'s").unwrap()
		} else {
			next
		}
	}
}

fn facet_name_to_id(name: &str) -> String {
	name.replace(['\'', ','], "")
		.to_case(convert_case::Case::Pascal)
}

#[proc_macro]
pub fn gifts(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect(""))
		.join("data")
		.join("Gifts.ron");

	let book: Book = ron::de::from_reader(File::open(path).expect("")).expect("Parsing error");

	let mut moon_gift_variants = TokenStream::new();
	let mut shadow_gift_variants = TokenStream::new();
	let mut wolf_gift_variants = TokenStream::new();
	let mut facet_variants = TokenStream::new();

	let mut shadow_gift_facets_variants = TokenStream::new();
	let mut wolf_gift_facets_variants = TokenStream::new();

	for gift in book.moon_gifts {
		if let Ok(name) = gift_name_to_id(&gift.name).parse::<TokenStream>() {
			moon_gift_variants.extend(quote! {
				#name,
			});

			for facet in gift.facets {
				if let Ok(facet_name) = facet_name_to_id(&facet.name).parse::<TokenStream>() {
					facet_variants.extend(quote! {
						#facet_name,
					});
				}
			}
		}
	}

	for gift in book.gifts {
		if let Ok(name) = gift_name_to_id(&gift.name).parse::<TokenStream>() {
			let (ts, ts2) = match gift.kind {
				GiftKind::Shadow => (&mut shadow_gift_variants, &mut shadow_gift_facets_variants),
				GiftKind::Wolf => (&mut wolf_gift_variants, &mut wolf_gift_facets_variants),
				_ => return derive_error!("Unkown type"),
			};

			ts.extend(quote! {
				#name,
			});

			let mut facets_arr = TokenStream::new();
			for facet in gift.facets {
				if let Ok(facet_name) = facet_name_to_id(&facet.name).parse::<TokenStream>() {
					facet_variants.extend(quote! {
						#facet_name,
					});
					facets_arr.extend(quote! {
						Facet::#facet_name,
					});
				}
			}

			ts2.extend(quote! {
				Self::#name => &[
					#facets_arr
				],
			});
		}
	}

	let expanded = quote! {
		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants, Hash, derive_more::Display,
		)]
		pub enum MoonGift {
			#moon_gift_variants
			Custom(String)
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum ShadowGift {
			#shadow_gift_variants
			Custom(String, [Facet; 5])
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum WolfGift {
			#wolf_gift_variants
			Custom(String, [Facet; 5])
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum Facet {
			#facet_variants
			Custom(String)
		}

		impl ShadowGift {
			pub fn get_facets(&self) -> &[Facet; 5] {
				match self {
					#shadow_gift_facets_variants
					Self::Custom(.., facets) => facets
				}
			}
		}

		impl WolfGift {
			pub fn get_facets(&self) -> &[Facet; 5] {
				match self {
					#wolf_gift_facets_variants
					Self::Custom(.., facets) => facets
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn merits(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect(""))
		.join("data")
		.join("merits_universal.ron");

	let Ok(str) = fs::read_to_string(path) else {
		return derive_error!("Error reading merits_universal.ron file");
	};

	let book: Book = ron::from_str(&str).expect("Parsing error");
	for merit in book.merits {}

	let expanded = quote! {};

	proc_macro::TokenStream::from(expanded)
}

/// Procedural macro to extend enums
#[proc_macro_attribute]
pub fn extend_enum(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	// Ensure we get the base enum name from the attribute
	let base_enum_name = parse_macro_input!(args as Ident);

	// Parse the input enum definition (ExtendedEnum)
	let item = parse_macro_input!(item as ItemEnum);
	let extended_enum_name = &item.ident;
	let extra_variants = item.variants;

	// Derive the necessary traits using derive_more
	// let derived_traits = quote! {
	// 	#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Into, Display)]
	// };

	// Build the final output with extra variants and base enum replication
	let output = quote! {
		#[derive(derive_more::Debug, derive_more::From, derive_more::TryInto)]
		enum #extended_enum_name {
			Base(#base_enum_name),
			#extra_variants
		}
	};

	output.into()
}
