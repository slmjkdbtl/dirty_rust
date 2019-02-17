// wengwengweng

extern crate proc_macro;

use quote::quote;
use syn::DeriveInput;
use syn::Data;

#[proc_macro_derive(Comp)]
pub fn comp_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

	let input: DeriveInput = syn::parse(tokens).unwrap();
	let name = &input.ident;

	if let Data::Struct(_) = input.data {} else {
		panic!("can only generate comp for structs");
	}

	return proc_macro::TokenStream::from(quote! {
		impl dirty::ecs::Comp for #name {}
	});

}

