use std::marker::PhantomData;

use concat_idents::concat_idents;
use seq_macro::seq;

use crate::{CompositeDecoder, CompositeEncoder, Decode, Encode, Encoder};

macro_rules! serialize_num {
    ($($type:ident),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
                concat_idents!(fn_name = encode_, $type, {encoder.fn_name(*self)})
            }
        }

        impl Decode for $type {
            fn decode<D: crate::Decoder>(mut decoder: D) -> Result<Self, D::Error> {
                concat_idents!(fn_name = decode_, $type, {decoder.fn_name()})
            }
        }
    )*};
}

serialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, bool, usize, isize, i128, u128);

seq!(A in 2..21 {#(
    seq!(N in 0..A {
        impl<#(T~N: Encode, )*> Encode for (#(T~N, )*) {
            fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                #[allow(unused_mut)]
                let mut tup = encoder.encode_tuple()?;
                #(tup.encode_element(&self.N)?;)*
                tup.end()?;
                Ok(())
            }
        }
        impl<#(T~N: Decode, )*> Decode for (#(T~N, )*) {
            fn decode<D: crate::Decoder>(decoder: D) -> Result<Self, D::Error> {
                #[allow(unused_mut)]
                let mut tup = decoder.decode_tuple()?;
                let v = (#(tup.decode_element()?, )*);
                tup.end()?;
                Ok(v)
            }
        }
    });
)*});

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
        let mut col = encoder.encode_seq(self.len())?;
        for v in self.iter() {
            col.encode_element(v)?;
        }
        col.end()?;
        Ok(())
    }
}

impl<T: Encode> Encode for &T {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        (*self).encode(encoder)
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _encoder: E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<T> Decode for PhantomData<T> {
    fn decode<D: crate::Decoder>(_decoder: D) -> Result<Self, D::Error> {
        Ok(Self)
    }
}
