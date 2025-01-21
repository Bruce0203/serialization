use std::mem::MaybeUninit;

use crate::{
    prelude::{encode_with_encoder, EncodeActor, Mesh},
    unsafe_wild_copy, BinaryDecoder, BinaryEncoder, CompositeDecoder, CompositeEncoder, Decoder,
    Encoder,
};

pub struct Codec<T>(pub(crate) T);

pub fn encode<'a, T>(src: &T, dst: &mut [u8]) -> Result<(), <Codec<*mut u8> as Encoder>::Error>
where
    T: Mesh<Codec<*mut u8>, Output: EncodeActor<T, Codec<*mut u8>>>,
{
    let mut coder = Codec(dst.as_mut_ptr());
    encode_with_encoder(src, &mut coder)
}

impl Encoder for Codec<*mut u8>
where
    Self: BinaryEncoder,
{
    type Error = EncodeError;

    type TupleEncoder = Self;

    type StructEncoder = Self;

    type SequenceEncoder = Self;

    fn encode_u8(&mut self, v: &u8) -> Result<(), Self::Error> {
        self.encode_array(&[*v]);
        Ok(())
    }

    fn encode_i8(&mut self, v: &i8) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u16(&mut self, v: &u16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i16(&mut self, v: &i16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u32(&mut self, v: &u32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i32(&mut self, v: &i32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u64(&mut self, v: &u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i64(&mut self, v: &i64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u128(&mut self, v: &u128) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i128(&mut self, v: &i128) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_usize(&mut self, v: &usize) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_isize(&mut self, v: &isize) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f32(&mut self, v: &f32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f64(&mut self, v: &f64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, v: &bool) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_tuple<'a>(&mut self) -> Result<&mut Self::TupleEncoder, Self::Error> {
        todo!()
    }

    fn encode_struct<'a>(&mut self) -> Result<&mut Self::StructEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_seq(&mut self, len: usize) -> Result<&mut Self::SequenceEncoder, Self::Error> {
        self.encode_u8(&(len as u8))?;
        Ok(self)
    }

    fn encode_enum_variant_key(
        &mut self,
        enum_name: &'static str,
        variant_name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_some(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_none(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        //TODO remained buffer space check
        self.encode_slice(v);
        Ok(())
    }

    fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        todo!()
    }
}

impl CompositeEncoder for Codec<*mut u8>
where
    Self: BinaryEncoder,
{
    type Error = EncodeError;

    fn encode_element<E: crate::Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
        v.encode(self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl BinaryEncoder for Codec<*mut u8> {
    fn encode_array<T: Copy, const N: usize>(&mut self, src: &[T; N]) {
        let dst = self.0 as *mut T;
        self.0 = dst.wrapping_add(N) as *mut u8;
        let src = src.as_ptr();
        unsafe {
            unsafe_wild_copy!([T; N], src, dst, N);
        }
    }

    fn encode_slice<T: Copy>(&mut self, src: &[T]) {
        for chunk in src.chunks(size_of::<T>() * 16) {
            let dst = self.0 as *mut T;
            unsafe {
                let src = chunk.as_ptr() as *const T;
                unsafe_wild_copy!([T; 16], src, dst, 16);
            }
            self.0 = dst.wrapping_add(chunk.len()) as *mut u8;
        }
    }
}

#[derive(Debug)]
pub enum EncodeError {
    NotEnoughSpaceInTheBuffer,
    TooLarge,
    Custom,
}

impl crate::EncodeError for EncodeError {
    fn not_enough_space_in_the_buffer() -> Self {
        Self::NotEnoughSpaceInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn custom() -> Self {
        Self::Custom
    }
}

impl<T> Decoder for Codec<T> {
    type Error = DecodeError;

    type TupleDecoder = Self;

    type StructDecoder = Self;

    type SequenceDecoder = Self;

    fn decode_u8(&mut self, place: &mut MaybeUninit<u8>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i8(&mut self, place: &mut MaybeUninit<i8>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u16(&mut self, place: &mut MaybeUninit<u16>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i16(&mut self, place: &mut MaybeUninit<i16>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u32(&mut self, place: &mut MaybeUninit<u32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i32(&mut self, place: &mut MaybeUninit<i32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u64(&mut self, place: &mut MaybeUninit<u64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i64(&mut self, place: &mut MaybeUninit<i64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u128(&mut self, place: &mut MaybeUninit<u128>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i128(&mut self, place: &mut MaybeUninit<i128>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_usize(&mut self, place: &mut MaybeUninit<usize>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_isize(&mut self, place: &mut MaybeUninit<isize>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_f32(&mut self, place: &mut MaybeUninit<f32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_f64(&mut self, place: &mut MaybeUninit<f64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_bool(&mut self, place: &mut MaybeUninit<bool>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_str(&mut self, place: &mut std::mem::MaybeUninit<&str>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_bytes<'a>(&mut self) -> Result<&'a [u8], Self::Error> {
        todo!()
    }

    fn decode_var_i32(
        &mut self,
        place: &mut std::mem::MaybeUninit<i32>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_tuple(&mut self) -> Result<&mut Self::TupleDecoder, Self::Error> {
        todo!()
    }

    fn decode_struct(&mut self) -> Result<&mut Self::StructDecoder, Self::Error> {
        todo!()
    }

    fn decode_seq(&mut self) -> Result<&mut Self::SequenceDecoder, Self::Error> {
        todo!()
    }

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error> {
        todo!()
    }

    fn decode_enum(
        &mut self,
        enum_name: &'static str,
    ) -> Result<crate::EnumIdentifier, Self::Error> {
        todo!()
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl<T> CompositeDecoder for Codec<T> {
    type Error = DecodeError;

    fn decode_element<D: crate::Decode>(
        &mut self,
        place: &mut MaybeUninit<D>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<T> BinaryDecoder for Codec<T> {
    fn decode_slice<const N: usize>(self, out: &mut MaybeUninit<[u8; N]>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum DecodeError {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    InvalidEnumVariantName,
    InvalidEnumVarirantIndex,
    Custom,
    InvalidUtf8,
    NonMaxButMax,
    NonZeroButZero,
}

impl crate::DecodeError for DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self {
        todo!()
    }

    fn too_large() -> Self {
        todo!()
    }

    fn invalid_enum_variant_name() -> Self {
        todo!()
    }

    fn invalid_enum_variant_index() -> Self {
        todo!()
    }

    fn custom() -> Self {
        todo!()
    }

    fn invalid_utf8() -> Self {
        todo!()
    }

    fn nonmax_but_max() -> Self {
        todo!()
    }

    fn nonzero_but_zero() -> Self {
        todo!()
    }
}
