use proc_macro2::{Ident, Span, TokenStream};

use quote::quote;

use syn::{Data, DataStruct, DeriveInput, Field, Fields};

pub fn expand_derive_builder(item: &DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let vis = &item.vis;
    let ident = &item.ident;
    let generics = &item.generics;
    let data = &item.data;

    let builder_type_ident = Ident::new(&format!("{}Builder", ident), Span::call_site());

    let fields = get_fields(&data);

    let builder_fields = make_builder_fields(&fields);
    let builder_fns = make_builder_fns(&fields);
    let constructor_fields = make_constructor_fields(&fields);

    Ok(quote! {
        #[derive(Default)]
        #vis struct #builder_type_ident #generics {
            #( #builder_fields ),*
        }

        impl #builder_type_ident {
            #( #builder_fns )*

            #vis fn build(self) -> #ident {
                #ident {
                    #( #constructor_fields ),*
                }
            }
        }

        impl #ident {
            #vis fn builder() -> #builder_type_ident {
                #builder_type_ident::default()
            }
        }
    })
}

fn get_fields(data: &Data) -> Vec<&Field> {
    match data {
        Data::Struct(data) => get_struct_fields(data),
        _ => vec![],
    }
}

fn get_struct_fields(data: &DataStruct) -> Vec<&Field> {
    match &data.fields {
        Fields::Named(named_fields) => named_fields.named.iter().collect(),
        _ => vec![],
    }
}

fn make_builder_fields<'a>(fields: &'a Vec<&'a Field>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                        #ident: Option<#ty>,
            }
        })
        .collect()
}

fn make_builder_fns<'a>(fields: &'a Vec<&'a Field>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                fn #ident(mut self, #ident: #ty) -> Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        })
        .collect()
}

fn make_constructor_fields<'a>(fields: &'a Vec<&'a Field>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            quote! {
                #ident: self.#ident.unwrap_or_else(Default::default)
            }
        })
        .collect()
}
