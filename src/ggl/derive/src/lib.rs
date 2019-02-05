// wengwengweng

#![recursion_limit="128"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::DeriveInput;
use syn::Data;
use syn::Fields;

#[proc_macro_derive(Vertex)]
pub fn comp_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

	let input: DeriveInput = syn::parse(tokens).unwrap();
	let name = &input.ident;
	let data = &input.data;

	let (stride, push_block, attr_block) = get_data(data);

	let expanded = quote! {

		impl ggl::VertexLayout for #name {

			const STRIDE: usize = #stride;

			fn push(&self, queue: &mut Vec<f32>) {
				#push_block
			}

			fn attrs() -> Vec<ggl::VertexAttr> {
				return vec![
					#attr_block
				];
			}

		}

	};

	return proc_macro::TokenStream::from(expanded);

}

fn get_data(data: &Data) -> (TokenStream, TokenStream, TokenStream) {

	return match *data {

		Data::Struct(ref data) => {

			match data.fields {

				Fields::Named(ref fields) => {

					let mut stride = 0 as usize;
					let mut push_recurse = Vec::new();
					let mut attr_recurse = Vec::new();

					for f in &fields.named {

						let name = &f.ident;

						if let syn::Type::Array(arr) = &f.ty {

							let elem = &arr.elem;

							if quote!(#elem).to_string() != "f32" {
								panic!("only accept f32");
							}

							if let syn::Expr::Lit(lit) = &arr.len {

								if let syn::Lit::Int(int) = &lit.lit {

									let len = int.value();
									let name = quote!(#name).to_string();

									attr_recurse.push(quote_spanned! {f.span() =>
										ggl::VertexAttr::new(#name, #len as i32, #stride),
									});

									stride += len as usize;

								} else {
									panic!("length has to be integer")
								}

							} else {
								panic!("length has to be literal");
							}

						} else {
							panic!("only accept fixed length arrays");
						}

						push_recurse.push(quote_spanned! {f.span() =>
							for v in &self.#name {
								queue.push(*v);
							}
						});

					}

					return (quote! {
						#stride
					}, quote!{
						#(#push_recurse)*
					}, quote!{
						#(#attr_recurse)*
					});

				},

				_ => panic!("cannot have unamed fields"),

			}

		},

		Data::Enum(_) => panic!("cannot generate vertex for enums"),
		Data::Union(_) => panic!("cannot generate vertex for unions"),

	};

}


