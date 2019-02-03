use proc_macro2::TokenStream;

pub fn expand_derive_builder(_item: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    Err(vec![])
}
