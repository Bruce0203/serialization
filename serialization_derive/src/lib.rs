use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Index};

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let crate_path = quote!(serialization);
    let ident = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
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
                const _: () = {
                    #crate_path::impl_meshup!(#ident; #(#idents => #types),*);
                };
            }
        }
        Data::Enum(data_enum) => {
            todo!()
        }
        Data::Union(_data_union) => {
            panic!("union not support")
        }
    }
    .into()
}
