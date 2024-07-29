#![feature(let_chains)]
use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, GenericArgument,
	PathArguments, Type, Variant,
};

use convert_case::Casing;

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
		if attr.path().is_ident("template") {
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

	// gen_func("xsplat", "xsplat_name", "Option<&str>", None);
	// gen_func("ysplat", "ysplat_name", "Option<&str>", None);
	// gen_func("zsplat", "zsplat_name", "Option<&str>", None);

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

#[proc_macro_derive(VariantName, attributes(expand))]
pub fn derive_variant_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut name_fun_variants = TokenStream::new();

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			let mut expand = false;

			for attr in &variant.attrs {
				if attr.path().is_ident("expand") {
					expand = true;
					break;
				}
			}

			if variant_name.eq("_Custom") {
				if let Fields::Unnamed(fields) = &variant.fields {
					if let Some(field) = fields.unnamed.first() {
						name_fun_variants.extend(quote_spanned! {field.span()=>
							#name::#variant_name (name, ..) => name,
						});
					}
				}
			} else {
				let mut match_arm = TokenStream::new();

				let fields_in_variant = if expand && variant.fields.len() == 1 {
					quote_spanned! {variant.span()=> (val) }
				} else {
					variant_fields(variant)
				};

				let variant_name_lower =
					variant_name.to_string().to_case(convert_case::Case::Snake);

				if expand {
					if let Fields::Unnamed(fields) = &variant.fields
						&& let Some(field) = fields.unnamed.first()
						&& let Type::Path(ty) = &field.ty
					{
						if ty.path.segments.first().unwrap().ident.eq("Option") {
							match_arm.extend(quote_spanned! {ty.span()=>
								match val {
									Some(val) => VariantName::name(val),
									None => #variant_name_lower
								}
							});
						} else {
							match_arm.extend(quote_spanned! {ty.span()=>
								VariantName::name(val)
							});
						}
					}
				} else {
					match_arm.extend(quote_spanned! {variant.span()=> #variant_name_lower });
				}

				name_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #match_arm,
				});
			}
		}
	} else {
		return derive_error!("VariantName is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics cofd_util::VariantName for #name #ty_generics #where_clause {
			fn name(&self) -> &str {
				match self {
					#name_fun_variants
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(AllVariants, attributes(expand))]
pub fn derive_all_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut all_variants = TokenStream::new();
	let mut sub_enums = TokenStream::new();

	let mut num: usize = 0;

	let mut flag = false;

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			if variant_name.eq("_Custom") {
				continue;
			}

			let mut flag1 = false;

			for attr in &variant.attrs {
				if attr.path().is_ident("expand") {
					flag1 = true;
					flag = true;
					break;
				}
			}

			let fields = match &variant.fields {
				Fields::Unnamed(fields) => {
					let mut field_tokens = TokenStream::new();

					if !flag1 {
						for field in &fields.unnamed {
							field_tokens
								.extend(quote_spanned! { field.span()=> Default::default(), });
						}
					} else if let Some(field) = fields.unnamed.first() {
						if let syn::Type::Path(ty) = &field.ty {
							if let Some(segment) = ty.path.segments.first() {
								if let PathArguments::AngleBracketed(arguments) = &segment.arguments
								{
									if let Some(GenericArgument::Type(ty2)) = arguments.args.first()
									{
										sub_enums.extend(
											quote_spanned! { field.span()=> vec.extend(<#ty2 as AllVariants>::all().map(Into::into)); },
										);
									}

									if segment.ident.to_string().eq("Box") {
										continue;
									} else {
										field_tokens.extend(
											quote_spanned! { field.span()=> Default::default(), },
										);
									}
								} else {
									sub_enums.extend(
										quote_spanned! { field.span()=> vec.extend(<#ty as AllVariants>::all().map(Into::into)); },
									);

									continue;
								}
							}
						}
					}
					quote_spanned! {variant.span()=> (#field_tokens) }
				}
				Fields::Unit => quote_spanned! { variant.span()=> },
				_ => continue,
			};

			all_variants.extend(quote_spanned! {variant.span()=>
				#name::#variant_name #fields,
			});
			num += 1;
		}
	} else {
		return derive_error!("AllVariants is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let mut all_vec = TokenStream::new();

	if flag {
		all_vec.extend(quote! {
			impl #impl_generics #name #ty_generics #where_clause {
				pub fn all() -> Vec<#name> {
					let mut vec = std::vec::Vec::from(<#name as cofd_util::AllVariants>::all());
					#sub_enums
					vec
				}
			}
		});
	}

	let expanded = quote! {
		impl #impl_generics cofd_util::AllVariants for #name #ty_generics #where_clause {
			const N: usize = #num;
			fn all() -> [Self; Self::N] {
				[
					#all_variants
				]
			}
		}
		#all_vec
	};

	proc_macro::TokenStream::from(expanded)
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
