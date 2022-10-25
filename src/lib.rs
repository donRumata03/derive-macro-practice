use quote::quote;
use proc_macro2::TokenStream;
use syn::{Data, DataEnum, Variant};

#[proc_macro_derive(FromPrimitive)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let _input = syn::parse_macro_input!(input as syn::DeriveInput);
	let syn::DeriveInput {
		attrs, vis, ident: enum_ident, generics, data
	} = _input;

	let match_arms: proc_macro2::TokenStream = if let Data::Enum(DataEnum {
		                  enum_token,
		                  brace_token,
		                  variants
	                  }) = data {
		variants.into_iter()
			.enumerate()
			.map(|(i, Variant{attrs, ident, fields, discriminant})|
				quote! {#i => Ok(#enum_ident::#ident),})
			.collect()
	} else {panic!()};


	quote! {
	    impl TryFrom<usize> for #enum_ident {
		    type Error = ();

		    fn try_from(value: usize) -> Result<Self, Self::Error> {
		        match value {
					#match_arms
		            _ => Err(())
		        }
		    }
		}
    }.into()
}

