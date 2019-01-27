// wengwengweng

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::DeriveInput;
use syn::Data;
use syn::Fields;

#[proc_macro_derive(Comp)]
pub fn comp_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

	let input: DeriveInput = syn::parse(tokens).unwrap();
	let name = &input.ident;
	let data = &input.data;

	let expanded = quote! {
		impl Comp for #name {}
	};

	return proc_macro::TokenStream::from(expanded);

}

