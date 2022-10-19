#[proc_macro_derive(FromPrimitive)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input = syn::parse_macro_input!(input as syn::DeriveInput);

    unimplemented!()
}
