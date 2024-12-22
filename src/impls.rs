use core::slice;
use std::{
    alloc::{Allocator, Global, Layout},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use concat_idents_bruce0203::concat_idents;
use nonmax::*;
use seq_macro::seq;

use crate::{
    const_transmute, BinaryDecoder, BinaryEncoder, CompositeDecoder, CompositeEncoder, Decode,
    DecodeError, Decoder, Encode, EncodeError, Encoder,
};

macro_rules! serialize_num {
    ($($type:ident),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                concat_idents!(fn_name = encode_, $type, {encoder.fn_name(*self)})
            }
        }

        impl Decode for $type {
            fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                concat_idents!(fn_name = decode_, $type, {decoder.fn_name(place)})
            }
        }
    )*};
}

serialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128, bool);

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
            fn decode<D: Decoder >(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                #[allow(unused_mut)]
                let mut tup = decoder.decode_tuple()?;

                #(tup.decode_element(unsafe { const_transmute::<_, &mut MaybeUninit<T~N>>(&mut place.assume_init_mut().N) })?;)*
                tup.end()?;
                Ok(())
            }
        }
    });
)*});

impl<T: Encode> Encode for Option<T> {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
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

const fn get_result_offset_of<T, Error>() -> usize {
    let result: ManuallyDrop<Result<T, Error>> =
        ManuallyDrop::new(Ok(unsafe { MaybeUninit::zeroed().assume_init() }));
    const RESULT_VARIANT_INDEX_SIZE: usize = 1;
    let data = unsafe {
        slice::from_raw_parts(
            &result as *const _ as *const u8,
            size_of::<T>() + RESULT_VARIANT_INDEX_SIZE,
        )
    };
    let mut i = 0;
    while i < data.len() {
        if data[i] == 1 {
            return i;
        }
        i += 1;
    }
    panic!("error while getting offset of result's variant index");
}

const fn get_option_offset_of<T>() -> usize {
    let option: ManuallyDrop<Option<T>> =
        ManuallyDrop::new(Some(unsafe { MaybeUninit::zeroed().assume_init() }));
    const OPTION_VARIANT_INDEX_SIZE: usize = 1;
    let data = unsafe {
        slice::from_raw_parts(
            &option as *const _ as *const u8,
            size_of::<T>() + OPTION_VARIANT_INDEX_SIZE,
        )
    };
    let mut i = 0;
    while i < data.len() {
        if data[i] == 1 {
            return i;
        }
        i += 1;
    }
    panic!("error while getting offset of option's variant index");
}

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        if decoder.decode_is_some()? {
            unsafe {
                *(place.assume_init_mut() as *mut _ as *mut u8)
                    .wrapping_add(get_option_offset_of::<T>()) = 1
            };

            let value_place =
                unsafe { const_transmute(place.assume_init_mut().as_mut().unwrap_unchecked()) };
            T::decode(decoder, value_place)?;
        } else {
            *place = MaybeUninit::new(None);
        }
        Ok(())
    }
}

impl<T: Encode, Error: Encode> Encode for Result<T, Error> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
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

impl<T: Decode, Error: Decode> Decode for Result<T, Error> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        if decoder.decode_is_some()? {
            unsafe {
                *(place.assume_init_mut() as *mut _ as *mut u8)
                    .wrapping_add(get_result_offset_of::<T, Error>()) = 1
            };
            let value_place =
                unsafe { const_transmute(place.assume_init_mut().as_mut().unwrap_unchecked()) };
            T::decode(decoder, value_place)?;
        } else {
            *place = MaybeUninit::new(Err(unsafe { MaybeUninit::uninit().assume_init() }));
            let value_place =
                unsafe { const_transmute(place.assume_init_mut().as_mut().unwrap_err_unchecked()) };
            Error::decode(decoder, value_place)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    default fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let col = encoder.encode_seq(self.len())?;
        for v in self.iter() {
            col.encode_element(v)?;
        }
        col.end()?;
        Ok(())
    }
}

impl Encode for Vec<u8> {
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let col = encoder.encode_seq(self.len())?;
        col.write_bytes(&self)
            .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
        col.end()?;
        Ok(())
    }
}

impl<T: Decode> Decode for Vec<T> {
    default fn decode<D: Decoder>(
        decoder: &mut D,
        place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let mut vec: Vec<T> = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        for i in 0..len {
            let value_place: &mut MaybeUninit<T> = unsafe { const_transmute(ptr.wrapping_add(i)) };
            seq.decode_element(value_place)?;
        }
        seq.end()?;
        unsafe { vec.set_len(len) };
        *place = MaybeUninit::new(vec);
        Ok(())
    }
}

impl Decode for Vec<u8> {
    default fn decode<D: Decoder>(
        decoder: &mut D,
        place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let mut vec = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        unsafe {
            let src = seq
                .read_bytes(len)
                .map_err(|()| DecodeError::not_enough_bytes_in_the_buffer())?;

            slice::from_raw_parts_mut(ptr as *mut _ as *mut u8, len).copy_from_slice(src);
        };
        seq.end()?;
        unsafe { vec.set_len(len) };
        *place = MaybeUninit::new(vec);
        Ok(())
    }
}

impl<T: Encode> Encode for &T {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        (*self).encode(encoder)
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<T> Decode for PhantomData<T> {
    fn decode<D: Decoder>(
        _decoder: &mut D,
        _place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}

impl Decode for &str {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        decoder.decode_str(place)
    }
}

impl Encode for &str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

impl Encode for &[u8] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_bytes(self)
    }
}

#[cfg(feature = "uuid")]
impl Encode for uuid::Uuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_bytes(self.as_bytes())
    }
}

#[cfg(feature = "uuid")]
impl Decode for uuid::Uuid {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        decoder.decode_u128(unsafe { const_transmute(place) })
    }
}

#[cfg(feature = "arrayvec")]
impl<T: Encode, const CAP: usize> Encode for arrayvec::ArrayVec<T, CAP> {
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let seq = encoder.encode_seq(self.len())?;
        for ele in self.iter() {
            seq.encode_element(ele)?;
        }
        seq.end()?;
        Ok(())
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayVec<u8, CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        self.as_slice().encode(encoder)
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Decode for arrayvec::ArrayVec<u8, CAP> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<T: Decode, const CAP: usize> Decode for arrayvec::ArrayVec<T, CAP> {
    default fn decode<D: Decoder>(
        decoder: &mut D,
        place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Decode for arrayvec::ArrayString<CAP> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayString<CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let vec = arrayvec::ArrayVec::<u8, CAP>::try_from(self.as_bytes()).unwrap();
        vec.encode(encoder)
    }
}

#[cfg(feature = "std")]
impl<'a, T: Encode + Clone> Encode for std::borrow::Cow<'a, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            std::borrow::Cow::Borrowed(value) => value.encode(encoder),
            std::borrow::Cow::Owned(value) => value.encode(encoder),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, T: Decode + Clone> Decode for std::borrow::Cow<'a, T> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        *place = MaybeUninit::new(std::borrow::Cow::Owned(unsafe {
            MaybeUninit::uninit().assume_init()
        }));
        let value_place: &mut MaybeUninit<T> =
            unsafe { const_transmute(place.assume_init_mut().to_mut()) };
        T::decode(decoder, value_place)?;
        Ok(())
    }
}

#[cfg(feature = "fastvarint")]
impl Decode for fastvarint::VarInt {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        decoder.decode_var_i32(unsafe { std::mem::transmute(place) })
    }
}

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::VarInt {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_var_i32(**self)
    }
}

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::NonMaxI32VarInt {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_var_i32(self.get())
    }
}

#[cfg(feature = "fastvarint")]
impl Decode for fastvarint::NonMaxI32VarInt {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        let place: &mut MaybeUninit<i32> = unsafe { const_transmute(place) };
        decoder.decode_var_i32(place)?;
        if unsafe { place.assume_init() } == i32::MAX {
            return Err(DecodeError::nonmax_but_max());
        }
        Ok(())
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let bytes = self.as_bytes();
        let col = encoder.encode_seq(bytes.len())?;
        col.write_bytes(bytes)
            .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
        col.end()?;
        Ok(())
    }
}

impl Decode for String {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        let place: &mut MaybeUninit<Vec<u8>> = unsafe { const_transmute(place) };
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let mut vec: Vec<u8> = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        let bytes = seq
            .read_bytes(len)
            .map_err(|()| DecodeError::not_enough_bytes_in_the_buffer())?;
        simdutf8::compat::from_utf8(bytes).map_err(|_err| DecodeError::invalid_utf8())?;
        unsafe { slice::from_raw_parts_mut(ptr, len).copy_from_slice(bytes) };
        seq.end()?;
        unsafe { vec.set_len(len) };
        *place = MaybeUninit::new(vec);
        Ok(())
    }
}

impl<T: Encode, const CAP: usize> Encode for [T; CAP] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let tup = encoder.encode_tuple()?;
        for v in self.iter() {
            tup.encode_element(v)?;
        }
        tup.end()
    }
}

impl<T: Decode, const CAP: usize> Decode for [T; CAP] {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        let tup = decoder.decode_tuple()?;
        for i in 0..CAP {
            let value_place: &mut MaybeUninit<T> =
                unsafe { const_transmute(place.assume_init_mut().get_unchecked_mut(i)) };
            tup.decode_element(value_place)?;
        }
        tup.end()?;
        Ok(())
    }
}

#[cfg(feature = "nonmax")]
macro_rules! nonmax {
    ($($type:ty: $inner:ty),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                self.get().encode(encoder)
            }
        }

        impl Decode for $type {
            fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                let place: &mut MaybeUninit<$inner> = unsafe { const_transmute(place) };
                <$inner>::decode(decoder, place)?;
                if unsafe { place.assume_init() } == <$inner>::MAX {
                    return Err(DecodeError::nonmax_but_max());
                }
                Ok(())
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
