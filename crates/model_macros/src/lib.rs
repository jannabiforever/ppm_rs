use proc_macro::TokenStream;

mod model_id;
mod model_name;

#[proc_macro_attribute]
pub fn model_id(attr: TokenStream, item: TokenStream) -> TokenStream {
	model_id::expand(attr, item)
}

#[proc_macro_attribute]
pub fn model_name(attr: TokenStream, item: TokenStream) -> TokenStream {
	model_name::expand(attr, item)
}
