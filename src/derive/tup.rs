use core::mem::{transmute, MaybeUninit};

use seq_macro::seq;

use crate::{CompositeDecoder, CompositeEncoder, Decode, Decoder, Encode, Encoder};

seq!(A in 2..21 {#(
    seq!(N in 0..A {

        impl<#(T~N: Encode, )*> Encode for (#(T~N, )*) {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                #[allow(unused_mut)]
                let mut tup = encoder.encode_tuple()?;
                #(tup.encode_element(&self.N)?;)*
                tup.end()?;
                Ok(())
            }
        }
        impl<#(T~N: Decode, )*> Decode for (#(T~N, )*) {
            fn decode_in_place<D: Decoder>(decoder: &mut D, out: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                #[allow(unused_mut)]
                let mut tup = decoder.decode_tuple()?;
                #(tup.decode_element(unsafe { transmute::<_, &mut MaybeUninit<T~N>>(&mut out.assume_init_mut().N) })?;)*
                tup.end()?;
                Ok(())
            }
        }
    });
)*});
