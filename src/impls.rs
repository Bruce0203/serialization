use core::slice;
use std::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use concat_idents_bruce0203::concat_idents;
use nonmax::*;
use seq_macro::seq;

use crate::{
    BinaryEncoder, CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode,
    EncodeError, Encoder,
};

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

impl<'de, T: Decode<'de>> Decode<'de> for Option<T> {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        if decoder.decode_is_some()? {
            Ok(Some(Decode::decode(decoder)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Encode, Error: Encode> Encode for Result<T, Error> {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        match self {
            Ok(value) => {
                encoder.encode_some()?;
                value.encode(encoder)
            }
            Err(value) => {
                encoder.encode_none()?;
                value.encode(encoder)
            }
        }
    }
}

impl<'de, T: Decode<'de>, Error: Decode<'de>> Decode<'de> for Result<T, Error> {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        if decoder.decode_is_some()? {
            Ok(Ok(T::decode(decoder)?))
        } else {
            Ok(Err(Error::decode(decoder)?))
        }
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

impl<'de, T: Decode<'de>> Decode<'de> for Vec<T> {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        let len = decoder.decode_seq_len()?;
        let mut seq = decoder.decode_seq()?;
        let mut result: ManuallyDrop<Vec<T>> = ManuallyDrop::new(Vec::with_capacity(len));
        unsafe { result.set_len(len) };
        for i in 0..len {
            let value: ManuallyDrop<T> = ManuallyDrop::new(seq.decode_element()?);
            unsafe {
                slice::from_raw_parts_mut(
                    &mut *result.as_mut_ptr().offset(i as isize) as *mut _ as *mut u8,
                    size_of::<T>(),
                )
                .copy_from_slice(slice::from_raw_parts(
                    &value as *const _ as *const u8,
                    size_of::<T>(),
                ));
            }
        }
        seq.end()?;
        Ok(ManuallyDrop::<Vec<T>>::into_inner(result))
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
        self.as_slice().encode(encoder)
    }
}

#[cfg(feature = "arrayvec")]
impl<'de, const CAP: usize> Decode<'de> for arrayvec::ArrayVec<u8, CAP> {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        let bytes: &[u8] = decoder.decode_bytes()?;
        if bytes.len() > CAP {
            Err(DecodeError::too_large())?
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
            *unsafe { result.get_unchecked_mut(i) } = seq.decode_element()?;
        }
        seq.end()?;
        Ok(result)
    }
}

#[cfg(feature = "arrayvec")]
impl<'de, const CAP: usize> Decode<'de> for arrayvec::ArrayString<CAP> {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        let vec = arrayvec::ArrayVec::<u8, CAP>::decode(decoder)?;
        let s = unsafe { std::str::from_utf8_unchecked(vec.as_slice()) };
        Ok(arrayvec::ArrayString::from(s).unwrap())
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayString<CAP> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let vec = arrayvec::ArrayVec::<u8, CAP>::try_from(self.as_bytes()).unwrap();
        vec.encode(encoder)
    }
}

#[cfg(feature = "std")]
impl<'a, T: Encode + Clone> Encode for std::borrow::Cow<'a, T> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        match self {
            std::borrow::Cow::Borrowed(value) => value.encode(encoder),
            std::borrow::Cow::Owned(value) => value.encode(encoder),
        }
    }
}

#[cfg(feature = "std")]
impl<'de, 'a, T: Decode<'de> + Clone> Decode<'de> for std::borrow::Cow<'a, T> {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        Ok(std::borrow::Cow::Owned(T::decode(decoder)?))
    }
}

#[cfg(feature = "fastvarint")]
impl<'de> Decode<'de> for fastvarint::VarInt {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        Ok(decoder.decode_var_i32()?.into())
    }
}

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::VarInt {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.encode_var_i32(**self)
    }
}

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::NonMaxI32VarInt {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.encode_var_i32(self.get())
    }
}

#[cfg(feature = "fastvarint")]
impl<'de> Decode<'de> for fastvarint::NonMaxI32VarInt {
    fn decode<D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        Ok(fastvarint::NonMaxI32VarInt::new(decoder.decode_var_i32()?))
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let bytes = self.as_bytes();
        let mut col = encoder.encode_seq(bytes.len())?;
        col.write_bytes(bytes)
            .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
        col.end()?;
        Ok(())
    }
}

impl<'de> Decode<'de> for String {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        Ok(unsafe { String::from_utf8_unchecked(Vec::<u8>::decode(decoder)?) })
    }
}

impl<T: Encode, const CAP: usize> Encode for [T; CAP] {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let mut tup = encoder.encode_tuple()?;
        for v in self.iter() {
            tup.encode_element(v)?;
        }
        tup.end()
    }
}

impl<'de, T: Decode<'de>, const CAP: usize> Decode<'de> for [T; CAP] {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        let mut tup = decoder.decode_tuple()?;
        let mut result: [T; CAP] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..CAP {
            result[i] = tup.decode_element()?;
        }
        tup.end()?;
        Ok(result)
    }
}

#[cfg(feature = "nonmax")]
macro_rules! nonmax {
    ($($type:ty: $inner:ty),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                self.get().encode(encoder)
            }
        }

        impl<'de> Decode<'de> for $type {
            fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
                Ok(unsafe { Self::new_unchecked(<$inner>::decode(decoder)?) })
            }
        }
    )*};
}

#[cfg(feature = "nonmax")]
nonmax!(
    NonMaxI8: i8,
    NonMaxU8: u8,

    NonMaxU16: u16,
    NonMaxI16: i16,

    NonMaxU32: u32,
    NonMaxI32: i32,

    NonMaxU64: u64,
    NonMaxI64: i64,

    NonMaxU128: u128,
    NonMaxI128: i128,

    NonMaxUsize: usize,
    NonMaxIsize: isize
);
