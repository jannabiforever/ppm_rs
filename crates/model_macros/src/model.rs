use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Item, Meta, Path, Result, parse_macro_input};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
	if !attr.is_empty() {
		return Error::new(proc_macro2::Span::call_site(), "#[model] takes no arguments")
			.to_compile_error()
			.into();
	}

	let mut item = parse_macro_input!(item as Item);

	match apply_model_derives(&mut item) {
		Ok(()) => quote!(#item).into(),
		Err(e) => e.to_compile_error().into(),
	}
}

fn apply_model_derives(item: &mut Item) -> Result<()> {
	let attrs = match item {
		Item::Struct(s) => &mut s.attrs,
		Item::Enum(e) => &mut e.attrs,
		_ => {
			return Err(Error::new(
				item.span(),
				"#[model] can only be applied to a struct or enum",
			));
		}
	};

	// 강제할 derive
	let required = [
		parse_path("Debug"),
		parse_path("Clone"),
		parse_path("serde::Serialize"),
		parse_path("serde::Deserialize"),
	];

	if let Some(attr) = attrs.iter_mut().find(|a| a.path().is_ident("derive")) {
		merge_derive_attr(attr, &required)?;
	} else {
		let new_attr: Attribute = syn::parse_quote!(
			#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
		);
		attrs.push(new_attr);
	}

	Ok(())
}

fn merge_derive_attr(attr: &mut Attribute, required: &[Path]) -> Result<()> {
	let Meta::List(meta_list) = &attr.meta else {
		return Err(Error::new(attr.span(), "malformed derive attribute"));
	};

	// 기존 derive들 수집
	let mut existing: Vec<Path> = meta_list
		.parse_args_with(syn::punctuated::Punctuated::<Path, syn::Token![,]>::parse_terminated)?
		.into_iter()
		.collect();

	// 필요한 derive가 없으면 추가
	for req in required {
		if !existing.iter().any(|p| same_last_ident(p, req)) {
			existing.push(req.clone());
		}
	}

	*attr = syn::parse_quote!(#[derive(#(#existing),*)]);
	Ok(())
}

fn parse_path(s: &str) -> Path {
	syn::parse_str::<Path>(s).expect("valid path")
}

fn same_last_ident(a: &Path, b: &Path) -> bool {
	a.segments.last().map(|s| &s.ident) == b.segments.last().map(|s| &s.ident)
}
