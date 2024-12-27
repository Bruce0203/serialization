use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Data, DeriveInput, Fields, Index};

use crate::{
    shared::{
        field_idents_indexes, field_types, field_types_and_indexes, field_types_idents_and_indexes,
        FieldAndIndexes, FieldTypeAndIdentAndIndexes, FieldTypeAndIndexes,
    },
    state::SerializableInput,
};

pub fn impl_serial_descriptor(
    SerializableInput {
        input:
            DeriveInput {
                ident,
                generics,
                data,
                ..
            },
        crate_path,
    }: &SerializableInput,
) -> TokenStream {
    let FieldTypeAndIndexes {
        field_indexes,
        field_types,
    } = match data {
        Data::Struct(data_struct) => field_types_and_indexes(&data_struct.fields),
        _ => panic!("non struct in not supported yet"),
    };
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let where_clause = where_clause.map(|where_clause| {
        let mut where_clause = where_clause.clone();
        where_clause.predicates.push(parse_quote!(Self: 'static));
        where_clause
    });
    quote! {
        impl #impl_generics const #crate_path::SerialDescriptor for #ident #type_generics #where_clause {
            const SIZES_LEN: usize = 0
                #(+ <#field_types as #crate_path::SerialDescriptor>::SIZES_LEN)*;
            fn serial_sizes<S: const #crate_path::PrimitiveTypeSizeChecker>(
            ) ->#crate_path::fastbuf::Buffer<#crate_path::SerialSize, { Self::SIZES_LEN }> {
                pub use #crate_path::fastbuf::WriteBuf;
                #crate_path::order_sizes_by_repr_and_calc_offset::<Self,S, { Self::SIZES_LEN }>(
                    &[#(<#field_types as #crate_path::SerialDescriptor>::serial_sizes::<S>().as_slice(),)*]
                )
            }
        }
    }
}

pub fn impl_field_path_finder(
    SerializableInput {
        input:
            DeriveInput {
                ident,
                generics,
                data,
                ..
            },
        crate_path,
    }: &SerializableInput,
) -> TokenStream {
    let FieldTypeAndIdentAndIndexes {
        field_indexes,
        field_idents,
        field_types,
    } = match data {
        Data::Struct(data_struct) => field_types_idents_and_indexes(&data_struct.fields),
        _ => panic!("non struct in not supported yet"),
    };
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics const #crate_path::FieldPathFinder for #ident #type_generics #where_clause {
            fn find_encode<'a, E: #crate_path::Encoder>(
                mut path: #crate_path::FieldPath,
            ) -> &'a dyn #crate_path::CompositableEncode<E> {
                if path.remaining() == 0 {
                    return &#crate_path::CompositableWrapper::<()>(core::marker::PhantomData);
                }
                pub use #crate_path::fastbuf::ReadBuf;
                match path.read(1)[0] as usize {
                    #(#field_indexes => <#field_types>::find_encode(path),)*
                    _ => unreachable!(),
                }
            }

            fn find_decode<'a, D: #crate_path::Decoder>(
                mut path: #crate_path::FieldPath,
            ) -> &'a dyn #crate_path::CompositableDecode<D> {
                if path.remaining() == 0 {
                    return &#crate_path::CompositableWrapper::<()>(core::marker::PhantomData);
                }
                pub use #crate_path::fastbuf::ReadBuf;
                match path.read(1)[0] as usize {
                    #(#field_indexes => <#field_types>::find_decode(path),)*
                    _ => unreachable!(),
                }
            }

            fn calc_offset(mut path: #crate_path::FieldPath) -> usize {
                if path.remaining() == 0 {
                    return 0;
                }
                let value = core::mem::MaybeUninit::<Self>::uninit();
                let value = unsafe { value.assume_init_ref() };
                pub use #crate_path::fastbuf::ReadBuf;
                match path.read(1)[0] as usize {
                    #(#field_indexes => {
                        #crate_path::offset_of(value, &value.#field_idents)
                            + <#field_types as #crate_path::FieldPathFinder>::calc_offset(path)
                    })*
                    _ => unreachable!(),
                }
            }
        }

    }
}

pub fn impl_field_path_drop(
    SerializableInput {
        input:
            DeriveInput {
                ident,
                generics,
                data,
                ..
            },
        crate_path,
    }: &SerializableInput,
) -> TokenStream {
    if matches!(data, Data::Enum(_)) {
        panic!("enum not supported yet");
    }
    let FieldAndIndexes {
        field_indexes,
        field_idents,
    } = match data {
        Data::Struct(data_struct) => field_idents_indexes(&data_struct.fields),
        _ => panic!("non struct type is not supported"),
    };
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #crate_path::FieldPathDrop for #ident #type_generics #where_clause {
            fn drop_fields(value: &mut core::mem::MaybeUninit<Self>, fields: #crate_path::FieldPath) {
                let fields = fields.as_slice();
                #(if fields.contains(&(#field_indexes as #crate_path::FieldIndex)) {
                    unsafe {
                        (&mut *((&mut value.assume_init_mut().#field_idents) as *mut _
                            as *mut core::mem::MaybeUninit<u32>))
                            .assume_init_drop();
                    }
                })*
            }
        }
    }
}
