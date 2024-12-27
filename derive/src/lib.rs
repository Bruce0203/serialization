use coder::{impl_decode, impl_encode};
use quote::quote;
use serial::{impl_field_path_drop, impl_field_path_finder, impl_serial_descriptor};
use state::SerializableInput;
use syn::parse_macro_input;

mod coder;
mod serial;
mod shared;
mod state;

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ref state = parse_macro_input!(input as SerializableInput);
    [
        impl_encode(state),
        impl_decode(state),
        impl_serial_descriptor(state),
        impl_field_path_finder(state),
        impl_field_path_drop(state),
    ]
    .into_iter()
    .fold(quote!(), |mut acc, v| {
        acc.extend(v);
        acc
    })
    .into()
}
