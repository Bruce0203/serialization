#![feature(concat_idents)]

use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, GenericParam, Ident, Type};

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let crate_path = quote!(serialization);
    let private = quote!(#crate_path::__private);
    let ident = input.ident;
    let mut impl_generics = input.generics.params.clone();
    let mut type_generics = input.generics.params.clone();
    for param in type_generics.iter_mut() {
        match param {
            GenericParam::Lifetime(lifetime_param) => {
                lifetime_param.bounds.clear();
            }
            GenericParam::Type(type_param) => {
                type_param.bounds.clear();
            }
            GenericParam::Const(_) => {}
        }
    }
    let mut where_clause = input
        .generics
        .where_clause
        .clone()
        .unwrap_or_else(|| parse_quote!(where))
        .predicates;
    for impl_generic in impl_generics.iter_mut() {
        match impl_generic {
            GenericParam::Lifetime(lifetime_param) => {
                lifetime_param.bounds.clear();
                let lt = &lifetime_param.lifetime;
                where_clause.push(parse_quote!(#lt: 'static));
            }
            GenericParam::Type(type_param) => {
                type_param.bounds.clear();
                let ident = &type_param.ident;
                where_clause.push(parse_quote!(#ident: #private::Edge));
            }
            GenericParam::Const(_) => {}
        }
    }
    match input.data {
        Data::Struct(data_struct) => {
            let impl_generics = impl_generics.iter();
            let where_clause = where_clause.iter();
            let Fields {
                types,
                idents,
                brace,
            } = data_struct.fields.into();
            quote! {
                #crate_path::impl_mesh!(
                    #brace,
                    (#ident), {#type_generics},
                    impl {#(#impl_generics,)*} (#(#where_clause,)*);
                    #(#idents => {#types}),*
                );
            }
        }
        Data::Enum(data_enum) => {
            let mut quotes = quote!();
            for variant in data_enum.variants.into_iter() {
                let impl_generics = impl_generics.iter();
                let where_clause = where_clause.iter();
                let Fields {
                    types,
                    idents,
                    brace,
                } = variant.fields.into();
                let variant_ident = variant.ident;
                let quote = quote! {
                    #crate_path::impl_enum_mesh!(
                        #brace,
                        (#ident), {#type_generics}, #variant_ident
                        impl {#(#impl_generics,)*} (#(#where_clause,)*);
                        #(#idents => {#types}),*
                    );
                };
                quotes.extend(quote);
            }
            quotes
        }
        Data::Union(_data_union) => {
            panic!("union not support")
        }
    }
    .into()
}

struct Fields {
    types: Vec<Type>,
    idents: Vec<Ident>,
    brace: Ident,
}

impl From<syn::Fields> for Fields {
    fn from(fields: syn::Fields) -> Self {
        let mut i = 0;
        let brace = match fields.iter().next() {
            Some(field) => match field.ident {
                Some(_) => Ident::new("brace", Span::call_site()),
                None => Ident::new("parentheses", Span::call_site()),
            },
            None => Ident::new("unit", Span::call_site()),
        };
        let types: Vec<_> = fields.iter().map(|field| field.ty.clone()).collect();
        let idents: Vec<_> = fields
            .into_iter()
            .map(|field| {
                field.ident.map(|field| field.clone()).unwrap_or_else(|| {
                    let result = format_ident!("v{}", i.to_string());
                    i += 1;
                    result
                })
            })
            .collect();

        Self {
            types,
            idents,
            brace,
        }
    }
}
