use proc_macro2::{Ident, Span, TokenStream};

use quote::quote;

pub fn expand_derive_builder(item: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &item.ident;
    let builder_type_ident = Ident::new(&format!("{}Builder", ident), Span::call_site());
    Ok(quote! {
        pub struct #builder_type_ident;

        impl #ident {
            fn builder() -> #builder_type_ident { #builder_type_ident }
        }
    })
}
