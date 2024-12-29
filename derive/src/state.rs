use std::marker::PhantomData;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Data,
    DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Generics, Ident, ImplGenerics, Index,
    Type, TypeGenerics, Variant, WhereClause,
};

pub struct SerializableInput<'a> {
    pub crate_path: TokenStream,
    pub item: SerializableItem<'a>,
    pub attrs: &'a Vec<Attribute>,
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub has_type_generic: bool,
    pub impl_generics: ImplGenerics<'a>,
    pub type_generics: TypeGenerics<'a>,
    pub where_clause: TokenStream,
}

impl<'a> SerializableInput<'a> {
    pub fn new(
        DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        }: &'a DeriveInput,
    ) -> Self {
        let crate_path = private(&Ident::new("serialization", Span::call_site()));
        let (impl_generics, type_generics, _where_clause) = generics.split_for_impl();
        let input = data.into();
        SerializableInput {
            item: input,
            attrs,
            ident,
            impl_generics,
            type_generics,
            where_clause: bounds_added_where_clause(&crate_path, &generics),
            has_type_generic: has_type_generic(&generics),
            generics,
            crate_path,
        }
    }
}

pub enum SerializableItem<'a> {
    Struct(SerializableStruct<'a>),
    Enum(SerializableEnum<'a>),
}

impl<'a> From<&'a Data> for SerializableItem<'a> {
    fn from(value: &'a Data) -> Self {
        match value {
            Data::Struct(data_struct) => {
                SerializableItem::Struct(SerializableStruct::new(data_struct))
            }
            Data::Enum(data_enum) => SerializableItem::Enum(SerializableEnum::new(data_enum)),
            Data::Union(_) => panic!("union not supported"),
        }
    }
}

pub struct SerializableStruct<'a> {
    pub item: &'a DataStruct,
    pub fields: SerializableFields,
}

impl<'a> SerializableStruct<'a> {
    pub fn new(data_struct: &'a DataStruct) -> Self {
        Self {
            item: data_struct,
            fields: SerializableFields::new(&data_struct.fields),
        }
    }
}

pub struct SerializableEnum<'a> {
    pub item: &'a DataEnum,
    pub variants: Vec<SerializableVariant<'a>>,
    pub variant_idents: Vec<&'a Ident>,
    pub discriminants: Vec<TokenStream>,
}

pub struct SerializableVariant<'a> {
    pub ident: &'a Ident,
    pub fields: SerializableFields,
}

impl<'a> SerializableEnum<'a> {
    pub fn new(data_enum: &'a DataEnum) -> Self {
        let variants = data_enum
            .variants
            .iter()
            .map(|variant| SerializableVariant {
                ident: &variant.ident,
                fields: SerializableFields::new(&variant.fields),
            })
            .collect();

        Self {
            discriminants: Self::discriminants(&data_enum),
            item: data_enum,
            variants,
            variant_idents: data_enum
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect(),
        }
    }

    fn discriminants(data_enum: &'a DataEnum) -> Vec<TokenStream> {
        let mut last_index: proc_macro2::TokenStream = quote! {0};
        data_enum
            .variants
            .iter()
            .map(|variant| {
                if let Some(discriminant) = &variant.discriminant {
                    let index = &discriminant.1;
                    last_index = quote! { #index };
                    let result = last_index.clone();
                    //TODO check is this should covered by parenthesis
                    last_index = quote! {#last_index + 1};
                    result
                } else {
                    let result = last_index.clone();
                    last_index = quote! {#last_index + 1};
                    result
                }
            })
            .collect::<Vec<_>>()
    }
}

pub struct SerializableFields {
    pub body_type: ItemBody,
    pub field_idents: Vec<Ident>,
    pub field_indexes: Vec<usize>,
    pub field_types: Vec<Type>,
    pub destructing_part: TokenStream,
}

impl SerializableFields {
    pub fn new(fields: &Fields) -> Self {
        let field_idents = field_idents(fields);
        let body_type = ItemBody::new(fields);
        Self {
            destructing_part: Self::destructing_part(fields, &body_type, &field_idents),
            body_type,
            field_idents,
            field_indexes: field_indexes(fields),
            field_types: field_types(fields),
        }
    }

    fn destructing_part(
        fields: &Fields,
        body_type: &ItemBody,
        field_idents: &Vec<Ident>,
    ) -> TokenStream {
        if matches!(fields, Fields::Unit) {
            return quote! {};
        }
        match body_type {
            ItemBody::Struct => quote! { { #(#field_idents),* } },
            ItemBody::Tuple => quote! { ( #(#field_idents),*) },
            ItemBody::Unit => quote! { {} },
        }
    }
}

pub enum ItemBody {
    Struct,
    Tuple,
    Unit,
}

impl ItemBody {
    pub fn new(fields: &Fields) -> Self {
        fields
            .iter()
            .next()
            .map(|field| {
                if field.ident.is_some() {
                    Self::Struct
                } else {
                    Self::Tuple
                }
            })
            .unwrap_or(Self::Unit)
    }
}

fn bounds_added_where_clause(crate_path: &TokenStream, generics: &Generics) -> TokenStream {
    if let Some(where_clause) = &generics.where_clause {
        where_clause.to_token_stream()
    } else {
        let type_generic_params = generics
            .params
            .iter()
            .filter(|param| matches!(param, GenericParam::Type(_)))
            .collect::<Vec<_>>();

        let lifetime_generic_params = generics
            .params
            .iter()
            .filter(|param| matches!(param, GenericParam::Lifetime(_)))
            .collect::<Vec<_>>();
        quote! { where
            #(#type_generic_params: #crate_path::Encode + #crate_path::Decode + 'static,)*
            #(#lifetime_generic_params: 'static,)*
        }
    }
}

fn has_type_generic(generics: &Generics) -> bool {
    generics
        .params
        .iter()
        .any(|param| matches!(param, GenericParam::Type(_)))
}

fn private(crate_name: &Ident) -> TokenStream {
    quote! { #crate_name::__private }
}

fn field_idents(fields: &Fields) -> Vec<Ident> {
    let mut index = 0;
    fields
        .iter()
        .map(|field| {
            field
                .ident
                .clone()
                .map(|ident| ident.clone())
                .unwrap_or_else(|| {
                    let result = format_ident!("f{index}");
                    index += 1;
                    result
                })
        })
        .collect::<Vec<_>>()
}

fn field_indexes(fields: &Fields) -> Vec<usize> {
    let len = fields.len();
    (0..len).collect::<Vec<_>>()
}

fn field_types(fields: &Fields) -> Vec<Type> {
    fields.iter().map(|field| field.ty.clone()).collect()
}
