use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Attribute, Error, ItemStruct, Result, Type, parse_macro_input};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
	// model_name takes no arguments
	if !attr.is_empty() {
		return Error::new(proc_macro2::Span::call_site(), "model_name takes no arguments")
			.to_compile_error()
			.into();
	}

	let st = parse_macro_input!(item as ItemStruct);
	match expand_impl(st) {
		Ok(ts) => ts.into(),
		Err(e) => e.to_compile_error().into(),
	}
}

fn expand_impl(st: ItemStruct) -> Result<proc_macro2::TokenStream> {
	let ident = &st.ident;
	let vis = &st.vis;

	let (inner_is_string, inner_ty_tokens) = extract_single_tuple_field_type(&st)?;
	if !inner_is_string {
		return Err(Error::new(
			inner_ty_tokens.span(),
			"model_name only supports `struct X(pub String);`",
		));
	}

	let user_attrs = filter_non_derive_attrs(&st.attrs);

	Ok(quote! {
		#(#user_attrs)*
		#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
		#[serde(transparent)]
		#vis struct #ident(pub String);

		impl ::core::fmt::Display for #ident {
			#[inline]
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				self.0.fmt(f)
			}
		}

		impl ::core::convert::From<String> for #ident {
			#[inline]
			fn from(value: String) -> Self {
				Self(value)
			}
		}

		impl ::core::convert::From<&str> for #ident {
			#[inline]
			fn from(value: &str) -> Self {
				Self(value.to_string())
			}
		}

		impl ::core::convert::AsRef<str> for #ident {
			#[inline]
			fn as_ref(&self) -> &str {
				&self.0
			}
		}

		impl ::core::ops::Deref for #ident {
			type Target = str;
			#[inline]
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	})
}

fn extract_single_tuple_field_type(st: &ItemStruct) -> Result<(bool, proc_macro2::TokenStream)> {
	let fields = &st.fields;
	let syn::Fields::Unnamed(unnamed) = fields else {
		return Err(Error::new(
			fields.span(),
			"model_name requires a tuple struct: `pub struct X(pub String);`",
		));
	};
	if unnamed.unnamed.len() != 1 {
		return Err(Error::new(
			unnamed.span(),
			"model_name requires exactly one field: `pub struct X(pub String);`",
		));
	}

	let field = unnamed.unnamed.first().unwrap();
	let ty = &field.ty;

	let is_string = matches_string_type(ty);
	Ok((is_string, ty.to_token_stream()))
}

fn matches_string_type(ty: &Type) -> bool {
	match ty {
		Type::Path(tp) => tp.path.segments.last().is_some_and(|seg| seg.ident == "String"),
		_ => false,
	}
}

fn filter_non_derive_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
	attrs.iter().filter(|a| !a.path().is_ident("derive")).cloned().collect()
}
