use crate::{Decode, Decoder, Encode, Encoder};

macro_rules! impl_num {
    ($($encode_fn_name:ident $decode_fn_name:ident: $type:ty),*) => {
        $(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                encoder.$encode_fn_name(self)
            }
        }

        impl Decode for $type {
            fn decode_in_place<D: Decoder>(
                decoder: &mut D,
                out: &mut std::mem::MaybeUninit<Self>,
            ) -> Result<(), D::Error> {
                decoder.$decode_fn_name(out)
            }
        }
        )*
    };
}

impl_num!(
    encode_u8 decode_u8: u8, encode_i8 decode_i8: i8,
    encode_u16 decode_u16: u16, encode_i16 decode_i16: i16,
    encode_u32 decode_u32: u32, encode_i32 decode_i32: i32,
    encode_u64 decode_u64: u64, encode_i64 decode_i64: i64,
    encode_u128 decode_u128: u128, encode_i128 decode_i128: i128,
    encode_usize decode_usize: usize, encode_isize decode_isize: isize,
    encode_f32 decode_f32: f32, encode_f64 decode_f64: f64,
    encode_bool decode_bool: bool
);
