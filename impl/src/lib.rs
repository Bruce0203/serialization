use std::iter::repeat_n;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Error, Fields, GenericParam, Generics, Ident,
    Index, Item, ItemEnum, ItemStruct, TypeParamBound,
};

#[proc_macro_derive(Serializable)]
pub fn serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    match input {
        Item::Enum(ref item_enum) => {
            let ref variant_state = variant_state(&item_enum);
            let encode = impl_encode_enum(item_enum, variant_state);
            let decode = impl_decode_enum(item_enum, variant_state);
            quote! {
                #encode
                #decode
            }
        }
        Item::Struct(ref item_struct) => {
            let encode = impl_encode_struct(item_struct);
            let decode = impl_decode_struct(item_struct);
            quote! {
                #encode
                #decode
            }
        }
        item => Error::new(item.span(), "only enum and struct supported").to_compile_error(),
    }
    .into()
}

fn impl_encode_enum(
    item_enum: &ItemEnum,
    variant_state: &VariantState<'_>,
) -> proc_macro2::TokenStream {
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
                        .map(|field| field.ident.clone().unwrap().to_token_stream())
                        .collect::<Vec<_>>();
                    let encode_struct = encode_struct(&field_names);
                    quote! { { #(#field_names),* } => #encode_struct }
                }
                Fields::Unnamed(fields_unnamed) => {
                    let indexes = (0..fields_unnamed.unnamed.len())
                        .map(|index| format_ident!("v{index}"))
                        .collect::<Vec<_>>();
                    let encode_tuple = encode_tuple(&indexes);
                    quote! { (#(#indexes),*) => #encode_tuple }
                }
                Fields::Unit => quote! { => Ok(()) },
            };
            quote! { #enum_name::#variant_name #branch_suffix }
        })
        .collect::<Vec<_>>();
    let enum_name = &item_enum.ident;
    let variant_names_match_branches = &variant_state.variant_names_match_branches;
    let variant_indexes_match_branches = &variant_state.variant_indexes_match_branches;
    let generic_params_without_bounds = generic_params_without_bounds(&item_enum.generics);
    let generic_params = generic_params_with_bounds(&item_enum.generics, || {
        parse_quote! { serialization::Encode }
    });
    let generic_where_clause = &item_enum.generics.where_clause;
    quote! {
        impl<#(#generic_params),*> serialization::Encode
            for #enum_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn encode<E: serialization::Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
                serialization::Encoder::encode_enum_variant_key(
                    &mut encoder,
                    std::any::type_name::<Self>(),
                    match self { #(#variant_names_match_branches),* },
                    match self { #(#variant_indexes_match_branches),* },
                )?;
                match self { #(#match_branches),* }
            }
    }
    }
}

fn impl_encode_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &item_struct.ident;
    let generic_params = generic_params_with_bounds(&item_struct.generics, || {
        parse_quote! { serialization::Encode }
    });

    let generic_where_clause = &item_struct.generics.where_clause;
    let generic_params_without_bounds = generic_params_without_bounds(&item_struct.generics);
    let fields = match &item_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                quote! { self.#field_name }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
            .map(|index| {
                let index = Index {
                    index: index as u32,
                    span: fields_unnamed.span(),
                };
                quote! { self.#index }
            })
            .collect::<Vec<_>>(),
        Fields::Unit => {
            vec![]
        }
    };
    let encode_struct = encode_struct(&fields);
    quote! {
        impl<#(#generic_params),*> serialization::Encode
            for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
                fn encode<E: serialization::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                    #encode_struct
                }
        }
    }
}

fn encode_struct(fields: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {{
        let mut struc = E::encode_struct(encoder)?;
        #(<E::StructEncoder as serialization::CompositeEncoder>::encode_element(&mut struc, &#fields)?;)*
        Ok(<E::StructEncoder as serialization::CompositeEncoder>::end(struc)?)
    }}
}
fn encode_tuple(fields: &Vec<Ident>) -> proc_macro2::TokenStream {
    quote! {{
        let mut tup = E::encode_tuple(encoder)?;
        #(<E::TupleEncoder as serialization::CompositeEncoder>::encode_element(&mut tup, #fields)?;)*
        Ok(<E::TupleEncoder as serialization::CompositeEncoder>::end(tup)?)
    }}
}

fn impl_decode_enum(
    item_enum: &ItemEnum,
    variant_state: &VariantState<'_>,
) -> proc_macro2::TokenStream {
    let enum_name = &item_enum.ident;
    let decode = item_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Named(fields_named) => {
                    let field_names = fields_named
                        .named
                        .iter()
                        .map(|field| field.ident.clone().unwrap().to_token_stream())
                        .collect::<Vec<_>>();
                    decode_struct(quote! { Self::#variant_name }, &field_names)
                }
                Fields::Unnamed(fields_unnamed) => {
                    decode_tuple(quote! { Self::#variant_name }, fields_unnamed.unnamed.len())
                }
                Fields::Unit => quote! { Ok(Self::#variant_name) },
            }
        })
        .collect::<Vec<_>>();

    let variant_names = &variant_state.variant_names;
    let variant_indexes = &variant_state.variant_indexes;
    let generic_params_without_bounds = generic_params_without_bounds(&item_enum.generics);
    let generic_params = generic_params_with_bounds(&item_enum.generics, || {
        parse_quote! { serialization::Decode<'de> }
    });

    let generic_where_clause = &item_enum.generics.where_clause;

    quote! {
        impl<#(#generic_params),*> serialization::Decode<'de>
            for #enum_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn decode<D: serialization::Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
                match serialization::Decoder::<'de>::decode_enum(&mut decoder, std::any::type_name::<Self>())? {
                    serialization::EnumIdentifier::Name(name) => match name {
                        #(stringify!(#variant_names) => #decode,)*
                        name => Err(serialization::DecodeError::invalid_enum_variant_name(name))?,
                    },
                    serialization::EnumIdentifier::Index(index) => {
                        #(const #variant_names: usize = #variant_indexes;)*
                        match index {
                            #(#variant_names => #decode,)*
                            index => Err(serialization::DecodeError::invalid_enum_variant_index(index))?,
                        }
                    },
                }
            }
        }
    }
}

fn impl_decode_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &item_struct.ident;
    let fields = match &item_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| field.ident.to_token_stream())
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
            .map(|index| {
                Index {
                    index: index as u32,
                    span: Span::call_site(),
                }
                .to_token_stream()
            })
            .collect::<Vec<_>>(),
        Fields::Unit => {
            vec![]
        }
    };
    let generic_params_without_bounds = generic_params_without_bounds(&item_struct.generics);
    let generic_params = generic_params_with_bounds(&item_struct.generics, || {
        parse_quote! { serialization::Decode<'de> }
    });
    let generic_where_clause = &item_struct.generics.where_clause;
    let decode_struct = decode_struct(quote! { Self }, &fields);
    quote! {
    impl<#(#generic_params),*> serialization::Decode<'de>
        for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn decode<D: serialization::Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
                #decode_struct
            }
        }
    }
}

fn decode_struct(
    struct_name: proc_macro2::TokenStream,
    fields: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {{
        let mut struc = serialization::Decoder::<'de>::decode_struct(decoder)?;
        let result = #struct_name {
            #(#fields: <D::StructDecoder as serialization::CompositeDecoder::<'de>>::decode_element(&mut struc)?),*
        };
        <D::StructDecoder as serialization::CompositeDecoder::<'de>>::end(struc)?;
        Ok(result)
    }}
}

fn decode_tuple(tuple_name: proc_macro2::TokenStream, size: usize) -> proc_macro2::TokenStream {
    let indexes = repeat_n(
        quote! { <D::TupleDecoder as serialization::CompositeDecoder::<'de>>::decode_element(&mut tup)? },
        size,
    );
    quote! {{
        let mut tup = serialization::Decoder::<'de>::decode_tuple(decoder)?;
        let result = #tuple_name(#(#indexes),*);
        <D::TupleDecoder as serialization::CompositeDecoder::<'de>>::end(tup)?;
        Ok(result)
    }}
}

fn generic_params_without_bounds(generics: &Generics) -> Vec<proc_macro2::TokenStream> {
    generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(lifetime_param) => lifetime_param.lifetime.to_token_stream(),
            GenericParam::Type(type_param) => type_param.ident.to_token_stream(),
            GenericParam::Const(const_param) => const_param.ident.to_token_stream(),
        })
        .collect::<Vec<_>>()
}

fn generic_params_with_bounds<F: Fn() -> TypeParamBound>(
    generics: &Generics,
    bound: F,
) -> Vec<GenericParam> {
    let mut generic_params: Vec<GenericParam> = vec![parse_quote!('de)];
    for param in generics.params.iter() {
        let param = match param {
            GenericParam::Type(type_param) => {
                let mut param = type_param.clone();
                param.bounds.push(bound());
                param.into()
            }
            GenericParam::Lifetime(lifetime) => {
                let param = lifetime.clone();
                match &mut generic_params[0] {
                    GenericParam::Lifetime(ref mut lifetime_param) => {
                        lifetime_param.bounds.push(lifetime.lifetime.clone());
                    }
                    _ => unreachable!(),
                };
                param.into()
            }
            v => v.clone(),
        };
        generic_params.push(param);
    }
    generic_params
}

struct VariantState<'a> {
    variant_names: Vec<&'a Ident>,
    variant_names_match_branches: Vec<proc_macro2::TokenStream>,
    variant_indexes: Vec<proc_macro2::TokenStream>,
    variant_indexes_match_branches: Vec<proc_macro2::TokenStream>,
}

fn variant_state<'a>(item_enum: &'a ItemEnum) -> VariantState<'a> {
    let enum_name = &item_enum.ident;
    let variant_names_match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields = match variant.fields {
                Fields::Named(_) => quote! { {..} },
                Fields::Unnamed(_) => quote! { (..) },
                Fields::Unit => quote! {},
            };
            quote! { #enum_name::#variant_name #fields => stringify!(#variant_name) }
        })
        .collect::<Vec<_>>();

    let variant_names = item_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect::<Vec<_>>();

    let mut last_index: proc_macro2::TokenStream = quote! {0};
    let mut variant_indexes = Vec::with_capacity(item_enum.variants.len());
    let variant_indexes_match = item_enum
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
                Fields::Named(_) => quote! { {..} },
                Fields::Unnamed(_) => quote! { (..) },
                Fields::Unit => quote! {},
            };
            let result = quote! {
                #enum_name::#variant_name #fields => #index
            };
            variant_indexes.push(index);
            result
        })
        .collect::<Vec<_>>();
    VariantState {
        variant_names,
        variant_names_match_branches,
        variant_indexes,
        variant_indexes_match_branches: variant_indexes_match,
    }
}
