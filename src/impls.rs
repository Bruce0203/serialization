use std::marker::PhantomData;

use concat_idents_bruce0203::concat_idents;
use seq_macro::seq;

use crate::{CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, Encoder};

macro_rules! serialize_num {
    ($($type:ident),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
                concat_idents!(fn_name = encode_, $type, {encoder.fn_name(*self)})
            }
        }

        impl<'de> Decode<'de> for $type {
            fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
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
        impl<'de, #(T~N: Decode<'de>, )*> Decode<'de> for (#(T~N, )*) {
            fn decode<D: Decoder<'de> >(decoder: D) -> Result<Self, D::Error> {
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

impl<'de, T> Decode<'de> for PhantomData<T> {
    fn decode<D: Decoder<'de>>(_decoder: D) -> Result<Self, D::Error> {
        Ok(Self)
    }
}

impl<'de> Decode<'de> for &'de str {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_str()
    }
}

impl Encode for &str {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

impl Encode for &[u8] {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.encode_bytes(self)
    }
}

#[cfg(feature = "uuid")]
impl Encode for uuid::Uuid {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.encode_bytes(self.as_bytes())
    }
}

#[cfg(feature = "uuid")]
impl<'de> Decode<'de> for uuid::Uuid {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        Ok(uuid::Uuid::from_u128(decoder.decode_u128()?))
    }
}

#[cfg(feature = "arrayvec")]
impl<T: Encode, const CAP: usize> Encode for arrayvec::ArrayVec<T, CAP> {
    default fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let mut seq = encoder.encode_seq(self.len())?;
        for ele in self.iter() {
            seq.encode_element(ele)?;
        }
        seq.end()?;
        Ok(())
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayVec<u8, CAP> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let mut seq = encoder.encode_seq(self.len())?;
        seq.encode_element(&self.as_slice())?;
        Ok(())
    }
}

#[cfg(feature = "arrayvec")]
impl<'de, const CAP: usize> Decode<'de> for arrayvec::ArrayVec<u8, CAP> {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        let bytes: &[u8] = decoder.decode_bytes()?;
        if bytes.len() != CAP {
            if bytes.len() > CAP {
                Err(DecodeError::too_large())?
            } else {
                Err(DecodeError::not_enough_bytes_in_the_buffer())?
            }
        }
        Ok(arrayvec::ArrayVec::try_from(bytes).unwrap())
    }
}

#[cfg(feature = "arrayvec")]
impl<'de, T: Decode<'de>, const CAP: usize> Decode<'de> for arrayvec::ArrayVec<T, CAP> {
    default fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        let len = decoder.decode_seq_len()?;
        let mut seq = decoder.decode_seq()?;
        let mut result = arrayvec::ArrayVec::new();
        unsafe { result.set_len(len) };
        for i in 0..len {
            result.insert(i, seq.decode_element()?);
        }
        seq.end()?;
        Ok(result)
    }
}
