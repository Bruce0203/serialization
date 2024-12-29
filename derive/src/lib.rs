//TODO remove warning suppression
#![allow(warnings)]
#![feature(extend_one)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod coder;
mod serial;
mod state;
use coder::*;
use serial::*;
use state::*;

#[proc_macro_derive(Serializable)]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ref input = parse_macro_input!(input as DeriveInput);
    let ref input = SerializableInput::new(input);
    [
        impl_encode(input),
        impl_decode(input),
        impl_serial_descriptor(input),
        impl_field_path_drop(input),
        impl_field_path_finder(input),
    ]
    .into_iter()
    .fold(quote!(), |mut acc, token_stream| {
        acc.extend(token_stream);
        acc
    })
    .into()
}
