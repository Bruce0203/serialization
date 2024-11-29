use std::{any::type_name_of_val, iter::repeat_n};

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Error, Fields, Generics, Ident, Index, Item, ItemEnum,
};

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    match input {
        Item::Enum(item_enum) => {
            let enum_name = &item_enum.ident;
            let match_branches = item_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    let branch_suffix = match &variant.fields {
                        Fields::Named(fields_named) => {
                            let field_names = fields_named
                                .named
                                .iter()
                                .map(|field| &field.ident)
                                .collect::<Vec<_>>();
                            let value = quote! {
                                { #(#field_names),* } => {
                                    let mut struc = E::encode_struct(encoder)?;
                                    #(<E::StructEncoder as serialization::CompositeEncoder>::encode_element(&mut struc, &#field_names)?;)*
                                    <E::StructEncoder as serialization::CompositeEncoder>::end(struc)?
                                }
                            };
                            value
                        }
                        Fields::Unnamed(fields_unnamed) => {
                            let indexes = (0..fields_unnamed.unnamed.len())
                                .map(|index| format_ident!("v{index}"))
                                .collect::<Vec<_>>();
                            quote! {
                                (#(#indexes),*) => {
                                    let mut tup = E::encode_tuple(encoder)?;
                                    #(<E::TupleEncoder as serialization::CompositeEncoder>::encode_element(&mut tup, #indexes)?;)*
                                    <E::TupleEncoder as serialization::CompositeEncoder>::end(tup)?
                                }
                            }
                        }
                        Fields::Unit => quote! { => () },
                    };
                    quote! { #enum_name::#variant_name #branch_suffix }
                })
                .collect::<Vec<_>>();
            quote! {
            impl serialization::Encode for #enum_name {
                fn encode<E: serialization::Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
                    serialization::Encoder::encode_enum_variant_key(
                        &mut encoder,
                        std::any::type_name::<Self>(),
                        self.__variant_name(),
                        self.__variant_index(),
                    )?;
                    Ok(match self { #(#match_branches),* })
                }
            }
            }.into()
        }
        Item::Struct(item_struct) => {
            let struct_name = &item_struct.ident;
            let generic_params_without_bounds =
                generic_params_without_bounds(&item_struct.generics);
            let generic_params = &item_struct.generics.params.iter().collect::<Vec<_>>();
            let generic_where_clause = &item_struct.generics.where_clause;
            let fields = match item_struct.fields {
                Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .map(|field| field.ident.to_token_stream())
                    .collect::<Vec<_>>(),
                Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
                    .map(|index| {
                        Index {
                            index: index as u32,
                            span: fields_unnamed.span(),
                        }
                        .to_token_stream()
                    })
                    .collect::<Vec<_>>(),
                Fields::Unit => {
                    vec![]
                }
            };
            quote! {
                impl<#(#generic_params),*> serialization::Encode
                    for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
                        fn encode<E: serialization::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                            let mut struc = serialization::Encoder::encode_struct(encoder)?;
                            #(<E::StructEncoder as serialization::CompositeEncoder>::encode_element(&mut struc, &self.#fields)?;)*
                            Ok(<E::StructEncoder as serialization::CompositeEncoder>::end(struc)?)
                        }
                }
            }
            .into()
        }

        _ => unimplemented!("not a valid item keyword {}", type_name_of_val(&input)), // TODO Test
    }
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as Item);
    match item_enum {
        Item::Enum(item_enum) => {
            let enum_name = &item_enum.ident;
            let decoding_body = item_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Named(fields_named) => {
                            let field_names = fields_named
                                .named
                                .iter()
                                .map(|field| &field.ident)
                                .collect::<Vec<_>>();

                            quote! {{
                                let mut struc = serialization::Decoder::decode_struct(decoder)?;
                                let result = Self::#variant_name {
                                    #(#field_names: <D::StructDecoder as serialization::CompositeDecoder>::decode_element(&mut struc)?),*
                                };
                                <D::StructDecoder as serialization::CompositeDecoder>::end(struc)?;
                                result
                            }}
                        }
                        Fields::Unnamed(fields_unnamed) => {
                            let indexes = repeat_n(
                                quote! { <D::TupleDecoder as serialization::CompositeDecoder>::decode_element(&mut tup)? },
                                fields_unnamed.unnamed.len(),
                            );
                            quote! {{
                                let mut tup = serialization::Decoder::decode_tuple(decoder)?;
                                let result = Self::#variant_name(#(#indexes),*);
                                <D::TupleDecoder as serialization::CompositeDecoder>::end(tup)?;
                                result
                            }}
                        }
                        Fields::Unit => {
                            quote! { {Self::#variant_name} }
                        }
                    }
                })
                .collect::<Vec<_>>();

            let VariantNames {
                variant_names,
                generated_impl_block: generated_variant_names,
            } = variant_names(&item_enum);
            let VariantIndexes {
                variant_indexes,
                generated_impl_block: generated_variant_indexes,
            } = variant_indexes(&item_enum);
            quote! {
                #generated_variant_names
                #generated_variant_indexes
                impl serialization::Decode for #enum_name {
                    fn decode<D: serialization::Decoder>(mut decoder: D) -> Result<Self, D::Error> {
                        Ok(match serialization::Decoder::decode_enum(&mut decoder, std::any::type_name::<Self>())? {
                            serialization::EnumIdentifier::Name(name) => match name {
                                #(stringify!(#variant_names) => #decoding_body),*
                                name => Err(serialization::DecodeError::invalid_enum_variant_name(name))?,
                            },
                            serialization::EnumIdentifier::Index(index) => { 
                                #(const #variant_names: usize = #variant_indexes;)*
                                match index {
                                    #(#variant_names => #decoding_body),*
                                    index => Err(serialization::DecodeError::invalid_enum_variant_index(index))?,
                                }
                            },
                        })
                    }
                }
            }.into()
        }
        Item::Struct(item_struct) => {
            let struct_name = &item_struct.ident;
            let generic_params_without_bounds =
                generic_params_without_bounds(&item_struct.generics);
            let generic_params = &item_struct.generics.params.iter().collect::<Vec<_>>();
            let generic_where_clause = &item_struct.generics.where_clause;
            let fields = match item_struct.fields {
                Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .map(|field| field.ident.to_token_stream())
                    .collect::<Vec<_>>(),
                Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
                    .map(|index| {
                        Index {
                            index: index as u32,
                            span: fields_unnamed.span(),
                        }
                        .to_token_stream()
                    })
                    .collect::<Vec<_>>(),
                Fields::Unit => {
                    vec![]
                }
            };
            quote! {
            impl<#(#generic_params),*> serialization::Decode
                for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
                    fn decode<D: serialization::Decoder>(decoder: D) -> Result<Self, D::Error> {
                        let mut struc = serialization::Decoder::decode_struct(decoder)?;
                        let result = Self {
                            #(#fields: <D::StructDecoder as serialization::CompositeDecoder>::decode_element(&mut struc)?),*
                        };
                        <D::StructDecoder as serialization::CompositeDecoder>::end(struc)?;
                        Ok(result)
                    }
                }
            }
            .into()
        }
        item => Error::new(item.span(), "only enum and struct supported")
            .to_compile_error()
            .into(),
    }
}

struct VariantNames<'a> {
    variant_names: Vec<&'a Ident>,
    generated_impl_block: proc_macro2::TokenStream,
}

fn variant_names(item_enum: &ItemEnum) -> VariantNames {
    let enum_name = &item_enum.ident;
    let match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields = match variant.fields {
                syn::Fields::Named(_) => quote! { {..} },
                syn::Fields::Unnamed(_) => quote! { (..) },
                syn::Fields::Unit => quote! {},
            };
            quote! { #enum_name::#variant_name #fields => stringify!(#variant_name) }
        })
        .collect::<Vec<_>>();
    let enum_generic_params_without_bounds = &item_enum
        .generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime.ident,
            syn::GenericParam::Type(type_param) => &type_param.ident,
            syn::GenericParam::Const(const_param) => &const_param.ident,
        })
        .collect::<Vec<_>>();
    let enum_generic_params = &item_enum.generics.params.iter().collect::<Vec<_>>();
    let enum_generic_where_clause = &item_enum.generics.where_clause;
    let variant_names = item_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect::<Vec<_>>();
    VariantNames {
        variant_names,
        generated_impl_block: quote! {
        impl<#(#enum_generic_params),*>
            #enum_name<#(#enum_generic_params_without_bounds),*> #enum_generic_where_clause {

                fn __variant_name(&self) -> &'static str {
                    match self {
                        #(#match_branches),*
                    }
                }
            }
        },
    }
}

struct VariantIndexes {
    variant_indexes: Vec<proc_macro2::TokenStream>,
    generated_impl_block: proc_macro2::TokenStream,
}

fn generic_params_without_bounds(generics: &Generics) -> Vec<&Ident> {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime.ident,
            syn::GenericParam::Type(type_param) => &type_param.ident,
            syn::GenericParam::Const(const_param) => &const_param.ident,
        })
        .collect::<Vec<_>>()
}

fn variant_indexes(item_enum: &ItemEnum) -> VariantIndexes {
    let enum_name = &item_enum.ident;
    let enum_generic_params = &item_enum.generics.params.iter().collect::<Vec<_>>();
    let enum_generic_params_without_bounds = generic_params_without_bounds(&item_enum.generics);
    let enum_generic_where_clause = &item_enum.generics.where_clause;
    let mut last_index: proc_macro2::TokenStream = quote! {0};
    let mut variant_indexes = Vec::with_capacity(item_enum.variants.len());
    let match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let index = if let Some(discriminant) = &variant.discriminant {
                let index = &discriminant.1;
                last_index = quote! { #index };
                last_index.clone()
            } else {
                last_index = quote! {#last_index + 1};
                last_index.clone()
            };

            let variant_name = &variant.ident;
            let fields = match variant.fields {
                syn::Fields::Named(_) => quote! { {..} },
                syn::Fields::Unnamed(_) => quote! { (..) },
                syn::Fields::Unit => quote! {},
            };
            let result = quote! {
                #enum_name::#variant_name #fields => #index
            };
            variant_indexes.push(index);
            result
        })
        .collect::<Vec<_>>();
    VariantIndexes {
        variant_indexes,
        generated_impl_block: quote! {
        impl<#(#enum_generic_params),*>
            #enum_name<#(#enum_generic_params_without_bounds),*> #enum_generic_where_clause {

                fn __variant_index(&self) -> usize {
                    match self {
                        #(#match_branches),*
                    }
                }
            }
        },
    }
}
