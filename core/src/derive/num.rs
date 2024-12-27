use core::mem::MaybeUninit;

use concat_idents_bruce0203::concat_idents;

use crate::{macros::input_all_prmitives_of, Decode, Decoder, Encode, Encoder};

macro_rules! serialize_num {
    ($($type:ty),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                concat_idents!(fn_name = encode_, $type, {encoder.fn_name(*self)})
            }
        }

        impl Decode for $type {
            fn decode_in_place<D: Decoder>(decoder: &mut D, out: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                concat_idents!(fn_name = decode_, $type, {decoder.fn_name(out)})
            }
        }
    )*};
}

input_all_prmitives_of!(serialize_num!());
