use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, DeriveInput};

pub struct SerializableInput {
    pub input: DeriveInput,
    pub crate_path: TokenStream,
}

impl Parse for SerializableInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(SerializableInput {
            input: input.parse()?,
            crate_path: quote! { serialization::__private },
        })
    }
}
