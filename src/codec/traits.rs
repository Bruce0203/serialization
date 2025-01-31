use std::mem::{Discriminant, MaybeUninit};

use crate::{BufRead, BufWrite, Endian};

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error>;
}

pub trait Codec {
    fn endian(&self) -> Endian;
}

macro_rules! encode_value {
    ($($fn_name:ident: $type:ty),*) => {$(
        fn $fn_name(&mut self, v: &$type) -> Result<(), Self::Error>;
    )*};
}

pub trait Encoder: Codec + Sized + BufWrite {
    type Error: EncodeError;
    type TupleEncoder: CompositeEncoder<Error = Self::Error>;
    type StructEncoder: CompositeEncoder<Error = Self::Error>;
    type SequenceEncoder: CompositeEncoder<Error = Self::Error>;

    encode_value!(
        encode_u8: u8, encode_i8: i8,
        encode_u16: u16, encode_i16: i16,
        encode_u32: u32, encode_i32: i32,
        encode_u64: u64, encode_i64: i64,
        encode_u128: u128, encode_i128: i128,
        encode_usize: usize, encode_isize: isize,
        encode_f32: f32, encode_f64: f64,
        encode_bool: bool
    );

    fn encode_tuple<'a>(&mut self) -> Result<&mut Self::TupleEncoder, Self::Error>;
    fn encode_struct<'a>(&mut self) -> Result<&mut Self::StructEncoder, Self::Error>;
    fn encode_seq(&mut self, len: usize) -> Result<&mut Self::SequenceEncoder, Self::Error>;

    fn encode_enum_variant<T>(
        &mut self,
        enum_name: &'static str,
        variant_name: &'static str,
        variant_discriminant: Discriminant<T>,
    ) -> Result<(), Self::Error>;

    fn encode_some(&mut self) -> Result<(), Self::Error>;
    fn encode_none(&mut self) -> Result<(), Self::Error>;

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error>;
    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error>;
    fn encode_seq_len(&mut self, v: usize) -> Result<(), Self::Error>;
}

pub trait Decode: Sized {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error>;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        let mut place: MaybeUninit<Self> = MaybeUninit::uninit();
        Self::decode_in_place(decoder, &mut place)?;
        Ok(unsafe { place.assume_init() })
    }

    fn populate<D: Decoder>(_decoder: &mut D) -> Result<(), D::Error> {
        Ok(())
    }
}

macro_rules! decode_value {
     ($($fn_name:ident: $type:ty),*) => {$(
        fn $fn_name(&mut self, place: &mut MaybeUninit<$type>) -> Result<(), Self::Error>;
     )*};
}

pub trait Decoder: Codec + Sized + BufRead {
    type Error: DecodeError;
    type TupleDecoder: CompositeDecoder<Error = Self::Error>;
    type StructDecoder: CompositeDecoder<Error = Self::Error>;
    type SequenceDecoder: CompositeDecoder<Error = Self::Error>;

    decode_value!(
        decode_u8: u8, decode_i8: i8,
        decode_u16: u16, decode_i16: i16,
        decode_u32: u32, decode_i32: i32,
        decode_u64: u64, decode_i64: i64,
        decode_u128: u128, decode_i128: i128,
        decode_usize: usize, decode_isize: isize,
        decode_f32: f32, decode_f64: f64,
        decode_bool: bool
    );

    fn decode_str(&mut self, place: &mut MaybeUninit<&str>) -> Result<(), Self::Error>;
    fn decode_bytes<'a>(&mut self) -> Result<&'a [u8], Self::Error>;
    fn decode_var_i32(&mut self, place: &mut MaybeUninit<i32>) -> Result<(), Self::Error>;

    fn decode_tuple(&mut self) -> Result<&mut Self::TupleDecoder, Self::Error>;
    fn decode_struct(&mut self) -> Result<&mut Self::StructDecoder, Self::Error>;
    fn decode_seq(&mut self) -> Result<&mut Self::SequenceDecoder, Self::Error>;

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error>;
    fn decode_enum_variant<T>(
        &mut self,
        enum_name: &'static str,
    ) -> Result<EnumIdentifier<T>, Self::Error>;

    fn decode_is_some(&mut self) -> Result<bool, Self::Error>;
}

pub enum EnumIdentifier<T> {
    VariantName(&'static str),
    Discriminant(Discriminant<T>),
}

pub trait EncodeError {
    fn not_enough_space_in_the_buffer() -> Self;
    fn too_large() -> Self;
    fn custom() -> Self;
}

pub trait DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self;
    fn too_large() -> Self;
    fn invalid_enum_variant_name() -> Self;
    fn invalid_enum_variant_index() -> Self;
    fn custom() -> Self;
    fn invalid_utf8() -> Self;
    fn nonmax_but_max() -> Self;
    fn nonzero_but_zero() -> Self;
}

pub trait CompositeEncoder: Encoder {
    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub trait CompositeDecoder: Decoder {
    fn decode_element<D: Decode>(&mut self, place: &mut MaybeUninit<D>) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}
