use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Expr, GenericParam, Ident, Type};

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let crate_path = quote!(serialization);
    // let private = quote!(#crate_path::__private);
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

    let type_generics_without_lt = type_generics
        .iter()
        .filter(|param| match param {
            GenericParam::Lifetime(_) => false,
            _ => true,
        })
        .collect::<Vec<_>>();
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
            }
            GenericParam::Const(_) => {}
        }
    }
    match input.data {
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
                    #crate_path::impl_mesh!(
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
            let fields = data_enum.variants.iter().map(|variant| (&variant.fields).into()).collect::<Vec<Fields>>();
            let mut quotes = {
                let impl_generics = impl_generics.iter();
                let where_clause = where_clause.iter();
                let type_generics = type_generics.iter();
                let type_generics_without_lt = type_generics_without_lt.iter();
                let variants = data_enum.variants.iter().map(|variant| &variant.ident);
                let variant_indices = 0..data_enum.variants.len();
                let ref mut last_discriminant: Option<Expr> = None;
                let discriminants = data_enum.variants.iter().map(|variant| {
                        let result = variant
                           .discriminant
                           .as_ref()
                           .map(|(_eq, expr)| parse_quote!((#expr) as isize))
                           .unwrap_or_else(|| {
                               if let Some(last_discriminant) = last_discriminant {
                                   parse_quote!((#last_discriminant) as isize + 1_isize)
                               } else {
                                   let result: Expr = parse_quote!(0_isize);
                                   *last_discriminant = Some(result.clone());
                                   result
                               }
                           });
                    *last_discriminant= Some(result.clone());
                    result
                });
                let braces = fields.iter().map(|field| &field.brace);
                quote! {
                    #crate_path::impl_enum_mesh!(
                        {#(#type_generics_without_lt),*},
                        (#ident), {#(#type_generics),*}, (#(#variants),*), (#(#variant_indices),*), (#(#discriminants),*), (#(#braces),*),
                        impl {#(#impl_generics,)*} (#(#where_clause,)*);
                    );
                }
            };
            for (variant_index, variant) in data_enum.variants.into_iter().enumerate() {
                let impl_generics = impl_generics.iter();
                let where_clause = where_clause.iter();
                let type_generics = type_generics.iter();
                let type_generics_without_lt = type_generics_without_lt.iter();
                let Fields {
                    types,
                    idents,
                    brace,
                } = &fields[variant_index];
                let variant_ident = variant.ident;

                let quote = quote! {
                    #crate_path::impl_enum_variant_mesh!(
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
    .into()
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
