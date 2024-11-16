use seq_macro::seq;

use crate::{Decode, Encode, Encoder};

impl<T: Encode> Encode for Option<T> {
    fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        if let Some(v) = self {
            encoder.encode_some()?;
            v.encode(encoder)?;
        } else {
            encoder.encode_none()?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let mut col = encoder.begin_seq(self.len())?;
        for v in self.iter() {
            col.encode_element(v)?;
        }
        col.end_seq()?;
        Ok(())
    }
}

impl Encode for u8 {
    fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        encoder.encode_u8(*self)
    }
}

impl Encode for u16 {
    fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        encoder.encode_u16(*self)
    }
}

seq!(A in 0..21 {#(
    seq!(N in 0..A {
        impl<#(T~N: Encode, )*> Encode for (#(T~N, )*) {
            fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                #[allow(unused_mut)]
                let mut tup = encoder.begin_tuple()?;
                #(tup.encode_element(&self.N)?;)*
                tup.end_tuple()?;
                Ok(())
            }
        }
        impl<#(T~N: Encode, )*> Decode for (#(T~N, )*) {
            fn decode<D: crate::Decoder>(decoder: D) -> Result<Self, D::Error> {
                #[allow(unused_mut)]
                let mut tup = decoder.begin_tuple()?;
                let v = (#(tup.decode_element()?, )*);
                tup.end_tuple()?;
                Ok(v)
            }
        }
    });
)*});
