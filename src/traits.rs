use std::mem::{ManuallyDrop, MaybeUninit};

use concat_idents_bruce0203::concat_idents;

#[cfg_attr(feature = "fast_binary_format", const_trait)]
pub trait CheckPrimitiveTypeSize {
    fn is_sized<T: 'static>() -> bool {
        false
    }
}

pub trait BinaryEncoder {
    fn skip_bytes(&mut self, len: usize);
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), ()>;
}

pub trait BinaryDecoder {
    fn skip_bytes(&mut self, len: usize);
    fn read_bytes(&mut self, len: usize) -> Result<&[u8], ()>;
}

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error>;
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error>;

    fn decode_placed<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        let mut place: MaybeUninit<Self> = unsafe { MaybeUninit::uninit().assume_init() };
        Self::decode(decoder, &mut place)?;
        Ok(unsafe { place.assume_init() })
    }
}

pub trait CompositeEncoder: BinaryEncoder {
    type Error;
    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

macro_rules! encode_value {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = encode_, $type, {
            fn fn_name(&mut self, v: $type) -> Result<(), Self::Error>;
        });
    )*};
}

macro_rules! decode_value {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = decode_, $type, {
            fn fn_name(&mut self, place: &mut MaybeUninit<$type>) -> Result<(), Self::Error>;
        });
    )*};
}

macro_rules! declare_encoder_supertrait {
    ($($value:ident),*) => {
        #[cfg_attr(feature = "fast_binary_format", const_trait)]
        pub trait Encoder: Sized + BinaryEncoder $(+ const $value)* {
            type Error: EncodeError;
            type TupleEncoder: CompositeEncoder<Error = Self::Error>;
            type StructEncoder: CompositeEncoder<Error = Self::Error>;
            type SequenceEncoder: CompositeEncoder<Error = Self::Error>;

            encode_value!(
                bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64
            );

            fn encode_tuple(&mut self) -> Result<&mut Self::TupleEncoder, Self::Error>;
            fn encode_struct(&mut self) -> Result<&mut Self::StructEncoder, Self::Error>;
            fn encode_seq(&mut self, len: usize) -> Result<&mut Self::SequenceEncoder, Self::Error>;

            fn encode_enum_variant_key(
                &mut self,
                enum_name: &'static str,
                variant_name: &'static str,
                variant_index: usize,
            ) -> Result<(), Self::Error>;

            fn encode_some(&mut self) -> Result<(), Self::Error>;
            fn encode_none(&mut self) -> Result<(), Self::Error>;

            fn encode_str(&mut self, v: &str) -> Result<(), Self::Error>;
            fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error>;
            fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error>;
        }
    };
}
#[cfg(feature = "fast_binary_format")]
declare_encoder_supertrait!(CheckPrimitiveTypeSize);
#[cfg(not(feature = "fast_binary_format"))]
declare_encoder_supertrait!();

pub trait CompositeDecoder: BinaryDecoder + Sized {
    type Error;
    fn decode_element<D: Decode>(&mut self, place: &mut MaybeUninit<D>) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;
}

pub enum EnumIdentifier {
    Name(&'static str),
    Index(usize),
}

pub trait EncodeError {
    fn not_enough_bytes_in_the_buffer() -> Self;
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

macro_rules! declare_decoder_supertrait {
    ($($value:ident),*) => {
        #[cfg_attr(feature = "fast_binary_format", const_trait)]
        pub trait Decoder: Sized + BinaryDecoder $(+ const $value)* {
            type Error: DecodeError;
            type TupleDecoder: CompositeDecoder< Error = Self::Error>;
            type StructDecoder: CompositeDecoder< Error = Self::Error>;
            type SequenceDecoder: CompositeDecoder< Error = Self::Error>;

            decode_value!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

            fn decode_str(&mut self, place: &mut MaybeUninit<& str>) -> Result<(), Self::Error>;
            fn decode_bytes(&mut self) -> Result<&[u8], Self::Error>;
            fn decode_var_i32(&mut self, place: &mut MaybeUninit<i32>) -> Result<(), Self::Error>;

            fn decode_tuple(&mut self) -> Result<&mut Self::TupleDecoder, Self::Error>;
            fn decode_struct(&mut self) -> Result<&mut Self::StructDecoder, Self::Error>;
            fn decode_seq(&mut self) -> Result<&mut Self::SequenceDecoder, Self::Error>;

            fn decode_seq_len(&mut self) -> Result<usize, Self::Error>;
            fn decode_enum(&mut self, enum_name: &'static str) -> Result<EnumIdentifier, Self::Error>;

            fn decode_is_some(&mut self) -> Result<bool, Self::Error>;
        }
    }
}

#[cfg(feature = "fast_binary_format")]
declare_decoder_supertrait!(CheckPrimitiveTypeSize);
#[cfg(not(feature = "fast_binary_format"))]
declare_decoder_supertrait!();
