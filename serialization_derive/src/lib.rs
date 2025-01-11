use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, GenericParam, Index};

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let crate_path = quote!(serialization);
    let ident = input.ident;
    let (_impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let mut impl_generics = input.generics.params.clone();
    let mut where_clause = where_clause
        .cloned()
        .unwrap_or_else(|| parse_quote!(where))
        .predicates;
    for impl_generic in impl_generics.iter_mut() {
        match impl_generic {
            GenericParam::Lifetime(lifetime_param) => {
                lifetime_param.bounds.clear();
                let lt = &lifetime_param.lifetime;
                //TODO TRY REMOVE or not
                where_clause.push(parse_quote!(#lt: 'static));
            }
            GenericParam::Type(type_param) => {
                type_param.bounds.clear();
            }
            GenericParam::Const(_const_param) => {}
        }
    }
    let impl_generics = impl_generics.into_iter();
    let where_clause = where_clause.into_iter();
    match input.data {
        Data::Struct(data_struct) => {
            let fields = data_struct.fields;
            let mut i = 0;
            let types: Vec<_> = fields.iter().map(|field| field.ty.clone()).collect();
            let idents: Vec<_> = fields
                .into_iter()
                .map(|field| {
                    field
                        .ident
                        .map(|field| field.to_token_stream())
                        .unwrap_or_else(|| {
                            let index = Index {
                                index: i,
                                span: Span::call_site(),
                            }
                            .to_token_stream();
                            i += 1;
                            index
                        })
                })
                .collect();
            quote! {
                #crate_path::impl_meshup!((#ident), {#type_generics}, impl {#(#impl_generics,)*} (#(#where_clause,)*); #(#idents => {#types}),*);
            }
        }
        Data::Enum(_data_enum) => {
            quote! {}
        }
        Data::Union(_data_union) => {
            panic!("union not support")
        }
    }
    .into()
}
