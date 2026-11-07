use std::collections::HashMap;

use convert_case::Casing;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	Data, DeriveInput, Error, Fields, GenericArgument, PathArguments, Type, Variant,
	parse_macro_input, spanned::Spanned,
};

macro_rules! derive_error {
	($string: tt) => {
		Error::new(Span::call_site(), $string)
			.to_compile_error()
			.into()
	};
}

fn parse_args(variant: &Variant, map: &mut HashMap<String, TokenStream>) -> syn::Result<()> {
	map.clear();
	for attr in &variant.attrs {
		if attr.path().is_ident("splat") {
			attr.parse_nested_meta(|meta| {
				let val: syn::Expr = meta.value()?.parse()?;

				map.insert(meta.path.get_ident().unwrap().to_string(), quote! { #val });
				Ok(())
			})?;
			break;
		}
	}

	Ok(())
}

fn variant_fields(variant: &Variant) -> TokenStream {
	match &variant.fields {
		Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
		Fields::Unit => quote_spanned! { variant.span()=> },
		Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
	}
}

#[proc_macro_derive(SplatEnum, attributes(splat, skip))]
pub fn derive_splat_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let data = &input.data;

	let mut variants_map: HashMap<String, TokenStream> = HashMap::new();

	if let Data::Enum(data_enum) = data {
		let mut args = HashMap::new();

		for variant in &data_enum.variants {
			let variant_name = &variant.ident;

			parse_args(variant, &mut args).unwrap();

			let fields_in_variant = variant_fields(variant);
			let mut gen_match_arm = |key: &str, b: bool| {
				if let Some(val) = args.get(key) {
					let v = if b {
						quote_spanned! { variant.span()=> Some(#val) }
					} else {
						quote_spanned! { variant.span()=> #val }
					};

					variants_map.entry(key.to_string()).or_default().extend(
						quote_spanned! {variant.span()=>
							#name::#variant_name #fields_in_variant => #v,
						},
					);
				}
			};

			gen_match_arm("virtue_anchor", false);
			gen_match_arm("vice_anchor", false);

			gen_match_arm("xsplat", true);
			gen_match_arm("ysplat", true);
			gen_match_arm("zsplat", true);

			gen_match_arm("ability", true);
			gen_match_arm("abilities_finite", false);

			gen_match_arm("st", true);
			gen_match_arm("alt_beats", true);

			gen_match_arm("fuel", true);
			gen_match_arm("integrity", false);
		}
	} else {
		return derive_error!("SplatEnum is only implemented for enums");
	}

	let mut funcs = TokenStream::new();
	let mut gen_func = |key: &str, name: &str, output_type: &str, default: Option<&str>| {
		if let Some(val) = variants_map.get(key) {
			let ident = syn::Ident::new(name, Span::call_site());
			let output_type: TokenStream = output_type.parse().unwrap();

			let mut def_match = TokenStream::new();
			if let Some(default) = default {
				let v: TokenStream = default.parse().unwrap();
				def_match.extend(quote! {
					_ => #v
				});
			} else {
				def_match.extend(quote! {
					_ => Default::default()
				});
			}

			funcs.extend(quote! {
				pub fn #ident(&self) -> #output_type {
					match self {
						#val
						#def_match
					}
				}
			});
		}
	};

	gen_func(
		"virtue_anchor",
		"virtue_anchor",
		"Anchor",
		Some("Anchor::Virtue"),
	);
	gen_func("vice_anchor", "vice_anchor", "Anchor", Some("Anchor::Vice"));

	gen_func("xsplat", "xsplat_name", "Option<&str>", None);
	gen_func("ysplat", "ysplat_name", "Option<&str>", None);
	gen_func("zsplat", "zsplat_name", "Option<&str>", None);

	gen_func("ability", "ability_name", "Option<&str>", None);
	gen_func(
		"abilities_finite",
		"are_abilities_finite",
		"bool",
		Some("true"),
	);

	gen_func(
		"st",
		"supernatural_tolerance",
		"Option<SupernaturalTolerance>",
		None,
	);
	gen_func("alt_beats", "alternate_beats", "Option<&str>", None);

	gen_func("fuel", "fuel", "Option<Fuel>", None);
	gen_func(
		"integrity",
		"integrity",
		"Integrity",
		Some("Integrity::Integrity"),
	);

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	let expanded = quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			#funcs
		}
	};

	proc_macro::TokenStream::from(expanded)
}

/// Returns `true` when the variant is annotated with `#[expand]`.
fn is_expand(variant: &Variant) -> bool {
	variant
		.attrs
		.iter()
		.any(|attr| attr.path().is_ident("expand"))
}

/// Describes the single field of an `#[expand]` variant.
///
/// Returns `None` when the variant cannot be expanded, i.e. when it does not
/// hold exactly one unnamed field.
fn expand_field(variant: &Variant) -> Option<Option<Type>> {
	let Fields::Unnamed(fields) = &variant.fields else {
		return None;
	};
	if fields.unnamed.len() != 1 {
		return None;
	}
	let field = fields.unnamed.first()?;
	let Type::Path(type_path) = &field.ty else {
		return None;
	};
	let segment = type_path.path.segments.last()?;
	if segment.ident != "Option" {
		return Some(None);
	}
	let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
		return None;
	};
	let Some(GenericArgument::Type(inner)) = arguments.args.first() else {
		return None;
	};
	Some(Some(inner.clone()))
}

/// Builds the pattern and value expression used to read the name of a `Custom` variant.
fn custom_variant_binding(variant: &Variant) -> (TokenStream, TokenStream) {
	match &variant.fields {
		Fields::Unnamed(fields) => {
			if fields.unnamed.len() > 1 {
				(quote! { (name, ..) }, quote! { name })
			} else {
				(quote! { (name) }, quote! { name })
			}
		}
		Fields::Named(fields) => {
			if fields
				.named
				.iter()
				.any(|field| field.ident.as_ref().is_some_and(|ident| ident == "name"))
			{
				(quote! { { name, .. } }, quote! { name })
			} else {
				(quote! { { .. } }, quote! { "" })
			}
		}
		Fields::Unit => (quote! {}, quote! { "" }),
	}
}

#[proc_macro_derive(VariantName, attributes(expand))]
pub fn derive_variant_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let Data::Enum(data) = &input.data else {
		return derive_error!("VariantName is only implemented for enums");
	};

	let mut arms = TokenStream::new();

	for variant in &data.variants {
		let variant_ident = &variant.ident;
		let kebab = variant_ident.to_string().to_case(convert_case::Case::Kebab);

		if variant_ident == "Custom" {
			let (pattern, value) = custom_variant_binding(variant);
			arms.extend(quote! { #name::#variant_ident #pattern => #value, });
			continue;
		}

		if is_expand(variant)
			&& let Some(optional) = expand_field(variant)
		{
			let value = if optional.is_some() {
				quote! {
					match value {
						Some(inner) => cofd_util::VariantName::name(inner),
						None => #kebab,
					}
				}
			} else {
				quote! { cofd_util::VariantName::name(value) }
			};
			arms.extend(quote! { #name::#variant_ident (value) => #value, });
			continue;
		}

		let fields = variant_fields(variant);
		arms.extend(quote! { #name::#variant_ident #fields => #kebab, });
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	quote! {
		impl #impl_generics cofd_util::VariantName for #name #ty_generics #where_clause {
			fn name(&self) -> &str {
				match self {
					#arms
				}
			}
		}
	}
	.into()
}

#[proc_macro_derive(AllVariants, attributes(expand))]
pub fn derive_all_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let Data::Enum(data) = &input.data else {
		return derive_error!("AllVariants is only implemented for enums");
	};

	let mut variants = TokenStream::new();

	for variant in &data.variants {
		if variant.ident == "Custom" {
			continue;
		}
		let variant_ident = &variant.ident;
		let fields = match &variant.fields {
			Fields::Unit => quote! {},
			Fields::Unnamed(fields) => {
				let defaults = fields
					.unnamed
					.iter()
					.map(|_| quote! { Default::default(), });
				quote! { (#(#defaults)*) }
			}
			Fields::Named(fields) => {
				let defaults = fields.named.iter().map(|field| {
					let ident = field.ident.as_ref().unwrap();
					quote! { #ident: Default::default(), }
				});
				quote! { { #(#defaults)* } }
			}
		};
		variants.extend(quote! { #name::#variant_ident #fields, });
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	quote! {
		impl #impl_generics cofd_util::AllVariants for #name #ty_generics #where_clause {
			fn all() -> Vec<Self> {
				vec![#variants]
			}
		}
	}
	.into()
}

#[proc_macro_derive(NameKey, attributes(expand))]
pub fn derive_name_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut match_cases = TokenStream::new();

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			match_cases.extend(quote_spanned! {variant.span()=>
				#name::#variant_name(val) => val.name_key(),
			});
		}
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics NameKey for #name #ty_generics #where_clause {
			fn name_key(&self) -> String {
				match self {
					#match_cases
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}
