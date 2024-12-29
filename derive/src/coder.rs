use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident};

use crate::{
    ItemBody, SerializableFields, SerializableInput, SerializableItem, SerializableStruct,
    SerializableVariant,
};

pub enum Item {
    Encode,
    Decode,
}

pub fn impl_encode(input: &SerializableInput) -> TokenStream {
    impl_coder(input, Item::Encode)
}

pub fn impl_decode(input: &SerializableInput) -> TokenStream {
    impl_coder(input, Item::Decode)
}

fn impl_coder(input: &SerializableInput, impl_item: Item) -> TokenStream {
    let SerializableInput {
        ident,
        item,
        generics,
        crate_path,
        has_type_generic,
        attrs,
        where_clause,
        impl_generics,
        type_generics,
    } = &input;
    match impl_item {
        Item::Encode => {
            let body = match item {
                SerializableItem::Struct(item_struct) => {
                    let body = impl_encode_struct(input, &ident, &item_struct.fields, !*has_type_generic);
                    let destructing_part = &item_struct.fields.destructing_part;
                    quote! {
                        let #ident #destructing_part = self;
                        #body
                    }
                }
                SerializableItem::Enum(item_enum) => {
                    let variant_idents = &item_enum.variant_idents;
                    let destructing_parts: Vec<&TokenStream> = item_enum
                        .variants
                        .iter()
                        .map(|variant| &variant.fields.destructing_part)
                        .collect();

                    let variant_names_match_branches = {
                        item_enum
                            .item
                            .variants
                            .iter()
                            .map(|variant| {
                                let variant_name = &variant.ident;
                                let fields = match variant.fields {
                                    Fields::Named(_) => quote! { {..} },
                                    Fields::Unnamed(_) => quote! { (..) },
                                    Fields::Unit => quote! {},
                                };
                                quote! { #ident::#variant_name #fields => stringify!(#variant_name) }
                            })
                            .collect::<Vec<_>>()
                    };
                    let match_branches = {
                        item_enum
                            .variants
                            .iter()
                            .map(
                                |SerializableVariant {
                                     ident: variant_ident,
                                     fields,
                                 }| {
                                    let branch_body = impl_encode_struct(input, &ident, fields, false);
                                    let destructing_part = &fields.destructing_part;
                                    quote! { #ident::#variant_ident #destructing_part => #branch_body }
                                },
                            )
                            .collect::<Vec<_>>()
                    };
                    let variant_indexes_match_branches = &item_enum.discriminants;
                    quote! {
                        #crate_path::Encoder::encode_enum_variant_key(
                            encoder,
                            core::any::type_name::<Self>(),
                            match self {
                                #(#ident::#variant_idents #destructing_parts => stringify!(#variant_idents),)*
                                #[allow(unreachable_patterns)]
                                _ => unreachable!()
                            },
                            #[allow(unreachable_code)]
                            match self {
                                #(#ident::#variant_idents #destructing_parts => { #variant_indexes_match_branches })*
                                #[allow(unreachable_patterns)]
                                _ => unreachable!()
                            },
                        )?;
                        match self {
                            #(#match_branches,)*
                            #[allow(unreachable_patterns)]
                            _ => unreachable!()
                        }
                    }
                }
            };

            quote! {
            impl #impl_generics #crate_path::Encode for #ident #type_generics #where_clause {
                fn encode<__E: #crate_path::Encoder>(&self, encoder: &mut __E) -> core::result::Result<(), __E::Error> {
                    #body
                }
            }
            }
        }
        Item::Decode => {
            let body = match item {
                SerializableItem::Struct(SerializableStruct { fields, .. }) => {
                    if *has_type_generic {
                        let body = impl_decode_struct(input, fields);
                        let destructing_part = &fields.destructing_part;
                        quote! {{
                            let #ident #destructing_part = unsafe { out.assume_init_mut() };
                            #body
                        }}
                    } else {
                        quote! {{
                            #crate_path::decode_struct(decoder, out)
                        }}
                    }
                }
                SerializableItem::Enum(item_enum) => {
                    let variant_idents: Vec<&Ident> = item_enum
                        .variants
                        .iter()
                        .map(|variant| variant.ident)
                        .collect();
                    let body: Vec<TokenStream> = item_enum
                        .variants
                        .iter()
                        .map(|variant| {
                            let destructing_part = &variant.fields.destructing_part;
                            let variant_ident = &variant.ident;
                            let field_idents = &variant.fields.field_idents;
                            let field_types = &variant.fields.field_types;
                            quote! {{
                                let tup = #crate_path::Decoder::decode_tuple(decoder)?;
                                #(let #field_idents = unsafe { core::mem::MaybeUninit::uninit().assume_init() };)* 
                                *out = core::mem::MaybeUninit::new(#ident::#variant_ident #destructing_part);
                                        match unsafe { out.assume_init_mut() } {
                                            #ident::#variant_ident #destructing_part => {
                                                #(
                                                let value_place: &mut core::mem::MaybeUninit<#field_types> = unsafe { core::mem::transmute(#field_idents) };
                                                <__D::TupleDecoder as #crate_path::CompositeDecoder>::decode_element::<#field_types>(tup, value_place)?;
                                                )*
                                            }
                                            _ => unreachable!()
                                        }
                                <__D::TupleDecoder as #crate_path::CompositeDecoder>::end(tup)
                            }}
                        }).collect();
                    let discriminants = &item_enum.discriminants;
                    quote! {
                            match #crate_path::Decoder::decode_enum(decoder, core::any::type_name::<Self>())? {
                                #crate_path::EnumIdentifier::Name(name) => match name {
                                    #(stringify!(#variant_idents) => #body)*
                                    #[allow(unreachable_patterns)]
                                    _ => Err(#crate_path::DecodeError::invalid_enum_variant_name())?
                                },
                                #crate_path::EnumIdentifier::Index(index) => {
                                    #(#[allow(non_upper_case_globals)] const #variant_idents: usize = #discriminants;)*
                                    #[allow(non_upper_case_globals)]
                                    match index {
                                        #(#variant_idents => #body)*
                                        #[allow(unreachable_patterns)]
                                        _ => Err(#crate_path::DecodeError::invalid_enum_variant_index())?
                                    }
                                },
                    }
                        }
                }
            };
            quote! {
                impl #impl_generics #crate_path::Decode for #ident #type_generics #where_clause {
                    fn decode_in_place<__D: #crate_path::Decoder>(
                        decoder: &mut __D,
                        out: &mut core::mem::MaybeUninit<Self>,
                    ) -> Result<(), __D::Error> {
                        #body
                    }
                }
            }
        }
    }
}

fn impl_encode_struct(
    SerializableInput {
        crate_path,
        has_type_generic,
        item,
        ..
    }: &SerializableInput,
    ident: &Ident,
    SerializableFields {
        field_idents,
        field_indexes,
        field_types,
        body_type,
        destructing_part,
    }: &SerializableFields,
    enable_fast_encoding: bool
) -> TokenStream {
    if enable_fast_encoding {quote! {{
            #crate_path::encode_struct(self, encoder)
        }}
           } else {
 quote! {{
            pub use #crate_path::CompositeEncoder;
            let ref mut compound = #crate_path::Encoder::encode_struct(encoder)?;
            #(<__E::StructEncoder>::encode_element(compound, #field_idents)?;)*
            <__E::StructEncoder>::end(compound)
        }}

        
    }
}

fn impl_decode_struct(
    SerializableInput { crate_path, .. }: &SerializableInput,
    SerializableFields {
        field_idents,
        field_indexes,
        field_types,
        body_type,
        destructing_part,
    }: &SerializableFields,
) -> TokenStream {
    quote! {
        pub use #crate_path::CompositeDecoder;
        let compound = #crate_path::Decoder::decode_struct(decoder)?;
        #(let __v: &mut core::mem::MaybeUninit<#field_types> = unsafe { core::mem::transmute(#field_idents) };
        <__D::StructDecoder>::decode_element(compound, __v)?;)*
        <__D::StructDecoder>::end(compound)
    }
}
