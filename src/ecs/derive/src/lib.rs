// wengwengweng

extern crate proc_macro;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Comp)]
pub fn comp_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

	let input: DeriveInput = syn::parse(tokens).unwrap();
	let name = &input.ident;

	let expanded = quote! {
		impl dirty::ecs::Comp for #name {}
	};

	return proc_macro::TokenStream::from(expanded);

}

