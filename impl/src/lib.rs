use std::iter::repeat_n;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, parse_str,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Plus},
    Error, Fields, GenericParam, Generics, Ident, Index, Item, ItemEnum, ItemStruct, Type,
    TypeParamBound,
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
            #[cfg(feature = "fast_binary_format")]
            let serial_descriptor = {
                if has_type_generic(&item_struct.generics.params) {
                    quote! {}
                } else {
                    impl_serial_descriptor(item_struct)
                }
            };
            #[cfg(not(feature = "fast_binary_format"))]
            let serial_descriptor = quote! {};
            let encode = impl_encode_struct(item_struct);
            let decode = impl_decode_struct(item_struct);
            quote! {
                #serial_descriptor
                #encode
                #decode
            }
        }
        item => Error::new(item.span(), "only enum and struct supported").to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    match input {
        Item::Enum(ref item_enum) => {
            let ref variant_state = variant_state(&item_enum);
            impl_encode_enum(item_enum, variant_state)
        }
        Item::Struct(ref item_struct) => impl_encode_struct(item_struct),
        item => Error::new(item.span(), "only enum and struct supported").to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    match input {
        Item::Enum(ref item_enum) => {
            let ref variant_state = variant_state(&item_enum);
            impl_decode_enum(item_enum, variant_state)
        }
        Item::Struct(ref item_struct) => impl_decode_struct(item_struct),
        item => Error::new(item.span(), "only enum and struct supported").to_compile_error(),
    }
    .into()
}

#[cfg(feature = "fast_binary_format")]
fn impl_serial_descriptor(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    use syn::parse_str;

    let struct_name = &item_struct.ident;
    let field_types = item_struct
        .fields
        .iter()
        .map(|field| {
            match &field.ty {
                syn::Type::Reference(type_reference) => {
                    syn::Type::Reference(if let Some(_lifetime) = &type_reference.lifetime {
                        let mut type_reference = type_reference.clone();
                        type_reference.lifetime = Some(parse_quote!('static));
                        type_reference
                    } else {
                        type_reference.clone()
                    })
                }
                v => v.clone(),
            }
            .clone()
        })
        .collect::<Vec<_>>();
    let field_names = match &item_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                quote! { #field_name }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
            .map(|index| {
                let index = Index {
                    index: index as u32,
                    span: fields_unnamed.span(),
                };
                quote! { #index }
            })
            .collect::<Vec<_>>(),
        Fields::Unit => {
            vec![]
        }
    };

    let field_count = item_struct.fields.len();

    let mut generic_params = generic_params_with_bounds(&item_struct.generics, || parse_quote! {});
    for param in generic_params.iter_mut() {
        match param {
            GenericParam::Type(type_param) => {
                type_param
                    .bounds
                    .push(TypeParamBound::Lifetime(parse_quote!('static)));
            }
            _ => {}
        }
    }
    let generic_where_clause = item_struct
        .generics
        .where_clause
        .clone()
        .map(|v| v.predicates)
        .unwrap_or_else(|| Punctuated::new());
    let generic_where_clause = generic_where_clause.iter().collect::<Vec<_>>();
    let generic_params_without_bounds_and_lifetimes = generic_params_without_bounds(
        &generic_params_without_lifetimes(&item_struct.generics.params),
    );

    let generic_params_without_bounds = generic_params_without_bounds(&item_struct.generics.params);
    let field_index = (0..field_names.len())
        .map(|i| {
            let s = format!("{{ {} as serialization::binary_format::Field }}", i);
            parse_str::<proc_macro2::TokenStream>(&s).unwrap()
        })
        .collect::<Vec<_>>();
    quote! {
        impl <#(#generic_params),*> const serialization::binary_format::SerialDescriptor
            for #struct_name<#(#generic_params_without_bounds),*> where #(#generic_where_clause),* {
            const N: usize = #(<#field_types as serialization::binary_format::SerialDescriptor>::N +)* #field_count + 1;
            fn fields<_C: const serialization::CheckPrimitiveTypeSize>(
            ) -> serialization::constvec::ConstVec<[serialization::binary_format::SerialSize; <Self as serialization::binary_format::SerialDescriptor>::N]> {
                serialization::binary_format::compact_fields({
                    #[allow(invalid_value)]
                    let value: std::mem::MaybeUninit<#struct_name<#(#generic_params_without_bounds_and_lifetimes),*>>
                        = std::mem::MaybeUninit::zeroed();
                    let value = unsafe { value.assume_init_ref() };
                    let mut padding_calc = serialization::binary_format::SizeCalcState::new(value);
                    #(
                    serialization::binary_format::SizeCalcState::next_field::<_, _C, #field_index>(
                        &mut padding_calc,
                        &value.#field_names,
                    );
                    )*
                    serialization::binary_format::SizeCalcState::finish(padding_calc)
                },
                serialization::binary_format::SerialSize::unsized_field_of())
            }
        }
    }
}

fn impl_encode_enum(
    item_enum: &ItemEnum,
    variant_state: &VariantState<'_>,
) -> proc_macro2::TokenStream {
    let enum_name = &item_enum.ident;

    let generic_params_without_bounds = generic_params_without_bounds(&item_enum.generics.params);
    let generic_params = generic_params_with_bounds(&item_enum.generics, || {
        parse_quote! { serialization::Encode }
    });
    let generic_where_clause = &item_enum.generics.where_clause;
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
                    let encode_tuple = encode_enum_tuple(&indexes);
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
    quote! {
        impl<#(#generic_params),*> serialization::Encode
            for #enum_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn encode<_E: serialization::Encoder>(&self, mut encoder: _E) -> Result<(), _E::Error> {
                serialization::Encoder::encode_enum_variant_key(
                    &mut encoder,
                    std::any::type_name::<Self>(),
                    match self { #(#variant_names_match_branches,)* #[allow(unreachable_patterns)] _ => unreachable!() },
                    #[allow(unreachable_code)]
                    match self { #(#variant_indexes_match_branches,)* #[allow(unreachable_patterns)] _ => unreachable!() },
                )?;
                match self { #(#match_branches,)* #[allow(unreachable_patterns)] _ => unreachable!() }
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
    let generic_params_without_bounds_and_lifetimes = generic_params_without_bounds(
        &generic_params_without_lifetimes(&item_struct.generics.params),
    );
    let generic_params_without_bounds = generic_params_without_bounds(&item_struct.generics.params);
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
    let encode_struct = if has_type_generic(&item_struct.generics.params) {
        encode_struct(&fields)
    } else {
        encode_struct_fast(
            quote! {#struct_name::<#(#generic_params_without_bounds_and_lifetimes),*>},
            &fields,
        )
    };
    let part1 = quote! {
        impl<#(#generic_params),*> serialization::Encode
            for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
                fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
                    #encode_struct
                }
        }
    };
    #[cfg(not(feature = "fast_binary_format"))]
    let part2 = quote! {};
    #[cfg(feature = "fast_binary_format")]
    let part2 = {
        if has_type_generic(&item_struct.generics.params) {
            return part1;
        }
        let generic_params = generic_params_with_bounds(&item_struct.generics, || {
            parse_quote! { serialization::binary_format::EncodeField + serialization::Encode }
        });
        let indexes = (0..fields.len()).collect::<Vec<_>>();
        quote! {
        impl<#(#generic_params),*> serialization::binary_format::EncodeField
            for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn encode_field<_E: serialization::Encoder>(
                &self,
                fields: &mut serialization::binary_format::Fields,
                encoder: _E,
            ) -> Result<(), _E::Error> {
                match *fields.pop_last() as usize {
                    #(#indexes => #fields.encode_field(fields, encoder),)*
                    _ => unreachable!(),
                }
            }
        }
        }
    };
    quote! {
        #part1
        #part2
    }
}

fn encode_struct(fields: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {{
        let mut struc = _E::encode_struct(encoder)?;
        #(<_E::StructEncoder as serialization::CompositeEncoder>::encode_element(&mut struc, &#fields)?;)*
        Ok(<_E::StructEncoder as serialization::CompositeEncoder>::end(struc)?)
    }}
}

fn encode_struct_fast(
    struct_name: proc_macro2::TokenStream,
    fields: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    #[cfg(feature = "fast_binary_format")]
    {
        quote! {{
            if const { serialization::binary_format::is_not_fast_binary::<#struct_name, _E>() } {
                let mut struc = encoder.encode_struct()?;
                #(serialization::CompositeEncoder::encode_element(&mut struc, &#fields)?;)*
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }}
    }
    #[cfg(not(feature = "fast_binary_format"))]
    quote! {{
        let mut struc = _E::encode_struct(encoder)?;
        #(<_E::StructEncoder as serialization::CompositeEncoder>::encode_element(&mut struc, &#fields)?;)*
        Ok(<_E::StructEncoder as serialization::CompositeEncoder>::end(struc)?)
    }}
}

fn encode_enum_tuple(fields: &Vec<Ident>) -> proc_macro2::TokenStream {
    quote! {{
        let mut tup = _E::encode_tuple(encoder)?;
        #(<_E::TupleEncoder as serialization::CompositeEncoder>::encode_element(&mut tup, #fields)?;)*
        Ok(<_E::TupleEncoder as serialization::CompositeEncoder>::end(tup)?)
    }}
}

fn encode_tuple(
    struct_name: proc_macro2::TokenStream,
    fields: &Vec<Ident>,
) -> proc_macro2::TokenStream {
    #[cfg(feature = "fast_binary_format")]
    {
        quote! {{
            if const { serialization::binary_format::is_not_fast_binary::<#struct_name, _E>() } {
                let mut struc = encoder.encode_tuple()?;
                #(serialization::CompositeEncoder::encode_element(&mut struc, &#fields)?;)*
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }}
    }
    #[cfg(not(feature = "fast_binary_format"))]
    quote! {{
        let mut tup = _E::encode_tuple(encoder)?;
        #(<_E::TupleEncoder as serialization::CompositeEncoder>::encode_element(&mut tup, #fields)?;)*
        Ok(<_E::TupleEncoder as serialization::CompositeEncoder>::end(tup)?)
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
                    let field_types = fields_named
                        .named
                        .iter()
                        .map(|field| field.ty.clone())
                        .collect::<Vec<_>>();
                    decode_structed_enum(quote! { Self::#variant_name }, &field_names, &field_types)
                }
                Fields::Unnamed(fields_unnamed) => {
                    let field_types = fields_unnamed
                        .unnamed
                        .iter()
                        .map(|field| field.ty.clone())
                        .collect::<Vec<_>>();
                    decode_enum_tuple(
                        quote! { Self::#variant_name },
                        fields_unnamed.unnamed.len(),
                        &field_types,
                    )
                }
                Fields::Unit => {
                    quote! { {
                        *place = std::mem::MaybeUninit::new(Self::#variant_name);
                        Ok(())
                    } }
                }
            }
        })
        .collect::<Vec<_>>();

    let variant_names = &variant_state.variant_names;
    let variant_indexes = &variant_state.variant_indexes;
    let generic_params_without_bounds = generic_params_without_bounds(&item_enum.generics.params);
    let generic_params = generic_params_with_bounds(&item_enum.generics, || {
        parse_quote! { serialization::Decode<'de> }
    });

    let generic_where_clause = &item_enum.generics.where_clause;

    quote! {
        impl<#(#generic_params),*> serialization::Decode<'de>
            for #enum_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn decode<_D: serialization::Decoder<'de>>(mut decoder: _D, place: &mut std::mem::MaybeUninit<Self>) -> Result<(), _D::Error> {
                match serialization::Decoder::<'de>::decode_enum(&mut decoder, std::any::type_name::<Self>())? {
                    serialization::EnumIdentifier::Name(name) => match name {
                        #(stringify!(#variant_names) => {#decode})*
                            #[allow(unreachable_patterns)]
                        _ => Err(serialization::DecodeError::invalid_enum_variant_name())?
                    },
                    serialization::EnumIdentifier::Index(index) => {
                        #(#[allow(non_upper_case_globals)] const #variant_names: usize = #variant_indexes;)*
                        #[allow(non_upper_case_globals)]
                        match index {
                            #(#variant_names => {#decode})*
                            #[allow(unreachable_patterns)]
                            _ => Err(serialization::DecodeError::invalid_enum_variant_index())?
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
    let generic_params_without_bounds_and_lifetimes = generic_params_without_bounds(
        &generic_params_without_lifetimes(&item_struct.generics.params),
    );
    let generic_params_without_bounds = generic_params_without_bounds(&item_struct.generics.params);
    let generic_params = generic_params_with_bounds(&item_struct.generics, || {
        parse_quote! { serialization::Decode<'de> }
    });
    let generic_where_clause = &item_struct.generics.where_clause;

    let field_types = item_struct
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .collect::<Vec<_>>();

    let decode_struct = if has_type_generic(&item_struct.generics.params) {
        decode_struct(quote! { #struct_name }, &fields, &field_types)
    } else {
        decode_struct_fast(
            quote! { #struct_name::<#(#generic_params_without_bounds_and_lifetimes),*> },
            &fields,
            &field_types,
        )
    };
    let part1 = quote! {
    impl<#(#generic_params),*> serialization::Decode<'de>
        for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            fn decode<_D: serialization::Decoder<'de>>(decoder: _D, place: &mut std::mem::MaybeUninit<Self>) -> Result<(), _D::Error> {
                #decode_struct
            }
        }
    };
    #[cfg(not(feature = "fast_binary_format"))]
    let part2 = quote! {};
    #[cfg(feature = "fast_binary_format")]
    let part2 = {
        if has_type_generic(&item_struct.generics.params) {
            return part1;
        }
        let generic_params = generic_params_with_bounds(&item_struct.generics, || {
            parse_quote! { serialization::binary_format::DecodeField<'de> + serialization::Decode<'de> }
        });
        let field_index = (0..fields.len())
            .map(|i| {
                let s = format!("{}_u16", i);
                parse_str::<proc_macro2::TokenStream>(&s).unwrap()
            })
            .collect::<Vec<_>>();

        quote! {
            impl<#(#generic_params),*> serialization::binary_format::DecodeField<'de>
                for #struct_name<#(#generic_params_without_bounds),*> #generic_where_clause {
            unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
                fields: &mut serialization::binary_format::Fields,
                field: &mut #struct_name<#(#generic_params_without_bounds),*>,
                decoder: &mut _D,
            ) -> Result<(), _D::Error> {
                match fields.pop_last() {
                    #(#field_index => {
                        serialization::binary_format::DecodeField::decode_field(
                            fields,
                            &mut field.#fields,
                            decoder,
                        )
                    })*
                    _ => unreachable!(),
                }
            }
        }

            }
    };
    quote! {
        #part1
        #part2
    }
}

fn decode_structed_enum(
    struct_name: proc_macro2::TokenStream,
    field_names: &Vec<proc_macro2::TokenStream>,
    field_types: &Vec<Type>,
) -> proc_macro2::TokenStream {
    let uninits = repeat_n(
        quote! {unsafe { std::mem::MaybeUninit::uninit().assume_init() }},
        field_names.len(),
    );
    quote! {{
        let mut tup = serialization::Decoder::decode_tuple(decoder)?;
        *place = std::mem::MaybeUninit::new(#struct_name { #(#field_names: #uninits),* } );
                match unsafe { place.assume_init_mut() } {
                    #struct_name {#(ref mut #field_names),*} => {
                        #(
                        let value_place: &mut std::mem::MaybeUninit<#field_types> = unsafe { serialization::const_transmute(#field_names) };
                <_D::TupleDecoder as serialization::CompositeDecoder>::decode_element::<#field_types>(&mut tup, value_place)?;
                        )*
                    }
                    _ => unreachable!()
                }
        <_D::TupleDecoder as serialization::CompositeDecoder>::end(tup)?;
        Ok(())
    }}
}

fn decode_struct(
    struct_name: proc_macro2::TokenStream,
    fields: &Vec<proc_macro2::TokenStream>,
    field_types: &Vec<Type>,
) -> proc_macro2::TokenStream {
    return quote! {{
        let mut struc = serialization::Decoder::<'de>::decode_struct(decoder)?;
        #(let value_place: &mut std::mem::MaybeUninit<#field_types> = unsafe { serialization::const_transmute(&mut place.assume_init_mut().#fields) };
        <_D::StructDecoder as serialization::CompositeDecoder>::decode_element::<#field_types>(&mut struc, value_place)?;)*
        <_D::StructDecoder as serialization::CompositeDecoder>::end(struc)?;
        Ok(())
    }};
}

fn decode_struct_fast(
    struct_name: proc_macro2::TokenStream,
    fields: &Vec<proc_macro2::TokenStream>,
    field_types: &Vec<Type>,
) -> proc_macro2::TokenStream {
    #[cfg(not(feature = "fast_binary_format"))]
    return decode_struct(struct_name, fields, field_types);
    #[cfg(feature = "fast_binary_format")]
    {
        quote! {{
            if const { serialization::binary_format::is_not_fast_binary::<#struct_name, _D>() } {
                let mut struc = decoder.decode_struct()?;
                #(let value_place: &mut std::mem::MaybeUninit<#field_types> = unsafe { serialization::const_transmute(&mut place.assume_init_mut().#fields) };
                serialization::CompositeDecoder::decode_element::<#field_types>(&mut struc, value_place)?;)*
                serialization::CompositeDecoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::decode2(decoder, place)
            }
        }}
    }
}

fn decode_enum_tuple(
    tuple_name: proc_macro2::TokenStream,
    size: usize,
    field_types: &Vec<Type>,
) -> proc_macro2::TokenStream {
    let values = (0..size)
        .map(|index| format_ident!("v{}", index))
        .collect::<Vec<_>>();
    let uninits = repeat_n(
        quote! {unsafe { std::mem::MaybeUninit::uninit().assume_init() }},
        size,
    );
    quote! {{
        let mut tup = serialization::Decoder::decode_tuple(decoder)?;
        *place = std::mem::MaybeUninit::new(#tuple_name(#(#uninits),*));
                match unsafe { place.assume_init_mut() } {
                    #tuple_name(#(ref mut #values),*) => {
                        #(
                        let value_place: &mut std::mem::MaybeUninit<#field_types> = unsafe { serialization::const_transmute(#values) };
                <_D::TupleDecoder as serialization::CompositeDecoder>::decode_element::<#field_types>(&mut tup, value_place)?;
                        )*
                    }
                    _ => unreachable!()
                }
        <_D::TupleDecoder as serialization::CompositeDecoder>::end(tup)?;
        Ok(())
    }}
}

fn generic_params_without_bounds(
    params: &Punctuated<GenericParam, Comma>,
) -> Vec<proc_macro2::TokenStream> {
    params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(lifetime_param) => lifetime_param.lifetime.to_token_stream(),
            GenericParam::Type(type_param) => type_param.ident.to_token_stream(),
            GenericParam::Const(const_param) => const_param.ident.to_token_stream(),
        })
        .collect::<Vec<_>>()
}

fn generic_params_without_lifetimes(
    generics: &Punctuated<GenericParam, Comma>,
) -> Punctuated<GenericParam, Comma> {
    let mut params = Punctuated::new();
    for ele in generics.iter() {
        match ele {
            GenericParam::Lifetime(_) => {}
            v => params.push(v.clone()),
        }
    }
    params
}

fn generic_params_with_bounds<F: Fn() -> Punctuated<TypeParamBound, Plus>>(
    generics: &Generics,
    bound: F,
) -> Vec<GenericParam> {
    let mut generic_params: Vec<GenericParam> = vec![parse_quote!('de)];
    for param in generics.params.iter() {
        let param = match param {
            GenericParam::Type(type_param) => {
                let mut param = type_param.clone();
                for elem in bound().into_iter() {
                    param.bounds.push(elem);
                }
                param.into()
            }
            GenericParam::Lifetime(lifetime) => {
                let mut param = lifetime.clone();
                if lifetime.lifetime.ident != format_ident!("static") {
                    param.bounds.push(parse_quote!('de));
                    match &mut generic_params[0] {
                        GenericParam::Lifetime(ref mut lifetime_param) => {
                            lifetime_param.bounds.push(lifetime.lifetime.clone());
                        }
                        _ => unreachable!(),
                    };
                }
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
                let result = last_index.clone();
                last_index = quote! {#last_index + 1};
                result
            } else {
                let result = last_index.clone();
                last_index = quote! {#last_index + 1};
                result
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

fn has_type_generic(generics: &Punctuated<GenericParam, Comma>) -> bool {
    generics.iter().any(|generic| match generic {
        GenericParam::Type(_) | GenericParam::Const(_) => true,
        _ => false,
    })
}
