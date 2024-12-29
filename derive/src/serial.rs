use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

use crate::{state::SerializableInput, SerializableFields, SerializableItem, SerializableStruct};

pub fn impl_serial_descriptor(
    SerializableInput {
        attrs,
        ident,
        item,
        generics,
        crate_path,
        has_type_generic,
        where_clause,
        impl_generics,
        type_generics,
    }: &SerializableInput,
) -> TokenStream {
    if *has_type_generic {
        return quote! {};
    }
    let SerializableFields {
        field_idents,
        field_indexes,
        field_types,
        ..
    } = match item {
        SerializableItem::Struct(serializable_struct) => &serializable_struct.fields,
        SerializableItem::Enum(serializable_enum) => return quote! {},
    };
    quote! {
        impl #impl_generics const #crate_path::SerialDescriptor for #ident #type_generics #where_clause {
            const SIZES_LEN: usize = 0
                #(+ <#field_types as #crate_path::SerialDescriptor>::SIZES_LEN)*;
            fn serial_sizes<__S: const #crate_path::PrimitiveTypeSizeChecker>(
            ) ->#crate_path::fastbuf::Buffer<[#crate_path::SerialSize; { Self::SIZES_LEN }]> {
                pub use #crate_path::fastbuf::WriteBuf;
                #crate_path::order_sizes_by_repr_and_calc_offset::<Self, __S, { Self::SIZES_LEN }>(
                    &[#(<#field_types as #crate_path::SerialDescriptor>::serial_sizes::<__S>().as_slice(),)*]
                )
            }
        }
    }
}

pub fn impl_field_path_finder(
    SerializableInput {
        attrs,
        ident,
        item,
        generics,
        crate_path,
        has_type_generic,
        where_clause,
        impl_generics,
        type_generics,
    }: &SerializableInput,
) -> TokenStream {
    if *has_type_generic {
        return quote! {};
    }

    let SerializableFields {
        field_idents,
        field_indexes,
        field_types,
        destructing_part,
        ..
    } = match item {
        SerializableItem::Struct(serializable_struct) => &serializable_struct.fields,
        SerializableItem::Enum(serializable_enum) => return quote!(),
    };
    quote! {
        impl #impl_generics const #crate_path::FieldPathFinder for #ident #type_generics #where_clause {
            fn find_encode<'__a, E: #crate_path::Encoder>(
                mut path: #crate_path::FieldPath,
            ) -> &'__a dyn #crate_path::CompositableEncode<E>
                where
                    Self: '__a
            {
                if path.remaining() == 0 {
                    return &#crate_path::CompositableWrapper::<()>(core::marker::PhantomData);
                }
                pub use #crate_path::fastbuf::ReadBuf;
                match path.read(1)[0] as usize {
                    #(#field_indexes => <#field_types>::find_encode(path),)*
                    _ => unreachable!(),
                }
            }

            fn find_decode<'__a, D: #crate_path::Decoder>(
                mut path: #crate_path::FieldPath,
            ) -> &'__a dyn #crate_path::CompositableDecode<D>
                where
                    Self: '__a

            {
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
                let #ident #destructing_part = value;
                pub use #crate_path::fastbuf::ReadBuf;
                match path.read(1)[0] as usize {
                    #(#field_indexes => {
                        #crate_path::offset_of(value, #field_idents)
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
        attrs,
        ident,
        item,
        generics,
        crate_path,
        has_type_generic,
        where_clause,
        impl_generics,
        type_generics,
    }: &SerializableInput,
) -> TokenStream {
    if *has_type_generic {
        return quote! {};
    }

    let SerializableFields {
        field_idents,
        field_indexes,
        field_types,
        body_type,
        destructing_part,
    } = match item {
        SerializableItem::Struct(serializable_struct) => &serializable_struct.fields,
        SerializableItem::Enum(serializable_enum) => return quote! {},
    };
    quote! {
        impl #impl_generics #crate_path::FieldPathDrop for #ident #type_generics #where_clause {
            fn drop_fields(value: &mut core::mem::MaybeUninit<Self>, fields: #crate_path::FieldPath) {
                let fields = fields.as_slice();
                let #ident #destructing_part = unsafe { value.assume_init_mut() };
                #(if fields.contains(&(#field_indexes as #crate_path::FieldIndex)) {
                    unsafe {
                        (&mut *((#field_idents) as *mut _
                            as *mut core::mem::MaybeUninit<u32>))
                            .assume_init_drop();
                    }
                })*
            }
        }
    }
}
