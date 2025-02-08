use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Attribute, Data,
    DeriveInput, Expr, GenericParam, Ident, Path, Token, Type, WherePredicate,
};

#[proc_macro_derive(Serialize)]
pub fn serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ref input = parse_macro_input!(input as DeriveInput);
    let ref input: Input = input.into();
    impl_codec(input, parse_quote!(__impl_encode)).into()
}

#[proc_macro_derive(Deserialize)]
pub fn deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ref input = parse_macro_input!(input as DeriveInput);
    let ref input: Input = input.into();
    impl_codec(input, parse_quote!(__impl_decode)).into()
}

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ref input = parse_macro_input!(input as DeriveInput);
    let ref input: Input = input.into();
    impl_serializable(input).into()
}

fn impl_serializable(
    Input {
        input,
        crate_path,
        ident,
        impl_generics,
        type_generics,
        type_generics_without_lt,
        where_clause,
    }: &Input,
) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(data_struct) => {
            let impl_generics = impl_generics.iter();
            let where_clause = where_clause.iter();
            let type_generics = type_generics.iter();
            let type_generics_without_lt = type_generics_without_lt.iter();
            let Fields {
                types,
                idents,
                brace,
            } = (&data_struct.fields).into();
            quote! {
                const _: () = {
                    #crate_path::__impl_mesh!(
                        {#(#type_generics_without_lt),*},
                        #brace,
                        (#ident), {#(#type_generics),*},
                        impl {#(#impl_generics,)*} (#(#where_clause,)*);
                        #(#idents => {#types}),*
                    );
                };
            }
        }
        Data::Enum(data_enum) => {
            let fields = data_enum
                .variants
                .iter()
                .map(|variant| (&variant.fields).into())
                .collect::<Vec<Fields>>();
            let mut quotes = {
                let impl_generics = impl_generics.iter();
                let where_clause = where_clause.iter();
                let type_generics = type_generics.iter();
                let type_generics_without_lt = type_generics_without_lt.iter();
                let variants = data_enum.variants.iter().map(|variant| &variant.ident);
                let variant_indices = 0..data_enum.variants.len();
                let ref mut last_discriminant: Option<Expr> = None;
                let discriminants = data_enum.variants.iter().map(|variant| {
                    let result: Expr = variant
                        .discriminant
                        .as_ref()
                        .map(|(_eq, expr)| parse_quote!((#expr) as isize))
                        .unwrap_or_else(|| {
                            if let Some(last_discriminant) = last_discriminant {
                                parse_quote!((#last_discriminant) as isize + 1_isize)
                            } else {
                                parse_quote!(0_isize)
                            }
                        });
                    *last_discriminant = Some(result.clone());
                    result
                });
                let braces = fields.iter().map(|field| &field.brace);
                quote! {
                    #crate_path::__impl_enum_mesh!(
                        {#(#type_generics_without_lt),*},
                        (#ident), {#(#type_generics),*}, (#(#variants),*), (#(#variant_indices),*), (#(#discriminants),*), (#(#braces),*),
                        impl {#(#impl_generics,)*} (#(#where_clause,)*);
                    );
                }
            };
            for (variant_index, variant) in data_enum.variants.iter().enumerate() {
                let impl_generics = impl_generics.iter();
                let where_clause = where_clause.iter();
                let type_generics = type_generics.iter();
                let type_generics_without_lt = type_generics_without_lt.iter();
                let Fields {
                    types,
                    idents,
                    brace,
                } = &fields[variant_index];
                let variant_ident = &variant.ident;

                let quote = quote! {
                    #crate_path::__impl_enum_variant_mesh!(
                        {#(#type_generics_without_lt),*},
                        #brace,
                        (#ident), {#(#type_generics),*}, #variant_ident, #variant_index,
                        impl {#(#impl_generics,)*} (#(#where_clause,)*);
                        #(#idents => {#types}),*
                    );
                };
                quotes.extend(quote);
            }
            quote! {
                const _: () = {
                    #quotes
                };
            }
        }
        Data::Union(_data_union) => {
            panic!("union not support")
        }
    }
}

struct Input<'a> {
    input: &'a DeriveInput,
    crate_path: TokenStream,
    ident: &'a Ident,
    impl_generics: Punctuated<GenericParam, Comma>,
    type_generics: Punctuated<GenericParam, Comma>,
    type_generics_without_lt: Vec<GenericParam>,
    where_clause: Punctuated<WherePredicate, Comma>,
}

impl<'a> From<&'a DeriveInput> for Input<'a> {
    fn from(value: &'a DeriveInput) -> Self {
        let crate_path = quote!(serialization);
        let ident = &value.ident;
        let mut impl_generics = value.generics.params.clone();
        let mut type_generics = value.generics.params.clone();
        for param in type_generics.iter_mut() {
            match param {
                GenericParam::Lifetime(lifetime_param) => lifetime_param.bounds.clear(),
                GenericParam::Type(type_param) => type_param.bounds.clear(),
                GenericParam::Const(_) => {}
            }
        }

        let type_generics_without_lt = type_generics
            .iter()
            .filter(|param| match param {
                GenericParam::Lifetime(_) => false,
                _ => true,
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut where_clause = value
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
                GenericParam::Type(type_param) => type_param.bounds.clear(),
                GenericParam::Const(_) => {}
            }
        }
        Input {
            input: value,
            crate_path,
            ident,
            impl_generics,
            type_generics,
            type_generics_without_lt,
            where_clause,
        }
    }
}

struct Fields {
    types: Vec<Type>,
    idents: Vec<Ident>,
    brace: Ident,
}

impl From<&syn::Fields> for Fields {
    fn from(fields: &syn::Fields) -> Self {
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
                field.ident.clone().unwrap_or_else(|| {
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

fn should_expand_mesh(attrs: &Vec<Attribute>) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            !attr
                .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                .unwrap()
                .iter()
                .any(|derive| derive.is_ident("Serialize") || derive.is_ident("Deserialize"))
        } else {
            false
        }
    })
}

fn impl_codec(input: &Input, codec_type: Ident) -> proc_macro2::TokenStream {
    let Input {
        crate_path,
        ident,
        impl_generics,
        type_generics,
        where_clause,
        ..
    } = input;
    let impl_generics = impl_generics.iter();
    let where_clause = where_clause.iter();
    let type_generics = type_generics.iter();
    quote! {
        const _: () = {
            #crate_path::#codec_type!(
                (#ident), {#(#type_generics),*},
                impl {#(#impl_generics,)*} (#(#where_clause,)*)
            );
        };
    }
}
