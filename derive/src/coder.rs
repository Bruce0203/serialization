use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::state::SerializableInput;

pub fn impl_encode(
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
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #crate_path::Encode for #ident #type_generics #where_clause {
            fn encode<__E: #crate_path::Encoder>(&self, encoder: &mut __E) -> core::result::Result<(), __E::Error> {
                #crate_path::encode_struct(self, encoder)
            }
        }
    }
}

pub fn impl_decode(
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
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #crate_path::Decode for #ident #type_generics #where_clause {
            fn decode_in_place<__D: #crate_path::Decoder>(
                decoder: &mut __D,
                out: &mut core::mem::MaybeUninit<Self>,
            ) -> Result<(), __D::Error> {
                #crate_path::decode_struct(decoder, out)
            }
        }
    }
}
