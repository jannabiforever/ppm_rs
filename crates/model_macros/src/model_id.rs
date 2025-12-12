use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
	Attribute, Error, Ident, ItemStruct, LitStr, Path, Result, Token, Type, parse_macro_input,
};

/// #[model_id(prefix = "task_", gen = crate::models::gen_id)]
struct ModelIdArgs {
	prefix: LitStr,
	r#gen: Path,
}

impl Parse for ModelIdArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		// prefix = "..."
		let prefix_ident: Ident = input.parse()?;
		if prefix_ident != "prefix" {
			return Err(Error::new(prefix_ident.span(), "expected `prefix = \"...\"`"));
		}
		input.parse::<Token![=]>()?;
		let prefix: LitStr = input.parse()?;

		input.parse::<Token![,]>()?;

		// gen = <path>
		let gen_ident: Ident = input.parse()?;
		if gen_ident != "gen" {
			return Err(Error::new(gen_ident.span(), "expected `gen = <path>`"));
		}
		input.parse::<Token![=]>()?;
		let r#gen: Path = input.parse()?;

		// trailing comma 허용
		let _ = input.parse::<Token![,]>();

		Ok(Self {
			prefix,
			r#gen,
		})
	}
}

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
	let args = parse_macro_input!(attr as ModelIdArgs);
	let st = parse_macro_input!(item as ItemStruct);

	match expand_impl(args, st) {
		Ok(ts) => ts.into(),
		Err(e) => e.to_compile_error().into(),
	}
}

fn expand_impl(args: ModelIdArgs, st: ItemStruct) -> Result<proc_macro2::TokenStream> {
	// newtype tuple struct: struct X(pub String);
	let ident = &st.ident;
	let vis = &st.vis;

	let (inner_is_string, inner_ty_tokens) = extract_single_tuple_field_type(&st)?;
	if !inner_is_string {
		return Err(Error::new(
			inner_ty_tokens.span(),
			"model_id only supports `struct X(pub String);`",
		));
	}

	// 기존 attrs는 보존하되, serde(transparent) 및 derive는 우리가 강제
	// (사용자가 또 derive를 붙이면 중복될 수 있으니, 사용 측에선 model_id만 쓰는 것으로 합의)
	let user_attrs = filter_non_derive_attrs(&st.attrs);

	let prefix = args.prefix;
	let r#gen = args.r#gen;

	Ok(quote! {
		#(#user_attrs)*
		#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
		#[serde(transparent)]
		#vis struct #ident(pub String);

		impl #ident {
			#[inline]
			pub fn new() -> Self {
				Self(format!("{}{}", #prefix, #r#gen()))
			}
		}

		impl ::core::default::Default for #ident {
			#[inline]
			fn default() -> Self {
				Self::new()
			}
		}

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
			"model_id requires a tuple struct: `pub struct X(pub String);`",
		));
	};
	if unnamed.unnamed.len() != 1 {
		return Err(Error::new(
			unnamed.span(),
			"model_id requires exactly one field: `pub struct X(pub String);`",
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
	// 사용자가 붙인 문서 주석, cfg, repr 같은 건 유지
	// derive는 우리가 강제하므로 제외
	attrs.iter().filter(|a| !a.path().is_ident("derive")).cloned().collect()
}
