#![feature(specialization)]
#![feature(core_intrinsics)]
#![feature(const_trait_impl)]
#![feature(inline_const_pat)]
#![feature(const_type_id)]
#![feature(str_from_raw_parts)]

use core::str;
use std::mem::{transmute, MaybeUninit};

use concat_idents::concat_idents;
use fastbuf::{Buf, Chunk, ReadBuf, WriteBuf, WriteBufferError};
use fastvarint::{DecodeVarInt, EncodeVarInt, VarInt};
use serialization::{
    CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, EncodeError, Encoder,
    EnumIdentifier, PrimitiveTypeSizeChecker,
};

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct PacketEncoder<S> {
    buffer: S,
}

impl<T> PacketEncoder<T> {
    pub fn new(t: T) -> Self {
        Self { buffer: t }
    }
}

#[derive(Debug /*Serializable*/)]
pub enum PacketEncodingError {
    FullOfCapacityInBuffer,
    TooLarge,
    Custom,
}

impl EncodeError for PacketEncodingError {
    fn not_enough_space_in_the_buffer() -> Self {
        Self::FullOfCapacityInBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn custom() -> Self {
        Self::Custom
    }
}

macro_rules! serialize_num {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = encode_, $type, {
            fn fn_name(&mut self, v: $type) -> Result<(), Self::Error> {
                self
                    .try_write(&v.to_be_bytes())
                    .map_err(|WriteBufferError::BufferFull| PacketEncodingError::FullOfCapacityInBuffer)
            }
        });
    )*};
}

macro_rules! deserialize_num {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = decode_, $type, {
            fn fn_name(&mut self, place: &mut MaybeUninit<$type>) -> Result<(), Self::Error> {
                let buf = self.read(size_of::<$type>());
                if buf.len() != size_of::<$type>() {
                    Err(PacketDecodingError::NotEnoughBytesInTheBuffer)?;
                }
                #[allow(invalid_value)]
                let mut slice = [unsafe { MaybeUninit::<u8>::uninit().assume_init() }; size_of::<$type>()];
                slice.copy_from_slice(buf);
                *place = MaybeUninit::new(<$type>::from_be_bytes(slice));
                Ok(())
            }
        });
    )*};
}

impl<'a, S: Buf<u8>> Encoder for PacketEncoder<S> {
    type Error = PacketEncodingError;
    type TupleEncoder = Self;
    type StructEncoder = Self;
    type SequenceEncoder = Self;

    serialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128);

    fn encode_struct(&mut self) -> Result<&mut Self, Self::Error> {
        Ok(self)
    }

    fn encode_enum_variant_key(
        &mut self,
        _name: &'static str,
        _variant_name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error> {
        self.encode_u8(variant_index as u8)?;
        Ok(())
    }

    fn encode_some(&mut self) -> Result<(), Self::Error> {
        self.encode_u8(1)
    }

    fn encode_none(&mut self) -> Result<(), Self::Error> {
        self.encode_u8(0)
    }

    fn encode_seq(&mut self, len: usize) -> Result<&mut Self, Self::Error> {
        self.encode_varint(len as i32)?;
        Ok(self)
    }

    fn encode_tuple(&mut self) -> Result<&mut Self, Self::Error> {
        Ok(self)
    }

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.try_write(&[v as u8])
            .map_err(|WriteBufferError::BufferFull| PacketEncodingError::FullOfCapacityInBuffer)
    }

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.encode_bytes(v.as_bytes())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        self.encode_seq(v.len())?;
        self.try_write(v)
            .map_err(|WriteBufferError::BufferFull| PacketEncodingError::FullOfCapacityInBuffer)
    }

    fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.encode_varint(v)
    }
}

impl<'a, S: Buf<u8>> CompositeEncoder for PacketEncoder<S> {
    type Error = <Self as Encoder>::Error;

    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
        v.encode(self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct PacketDecoder<S> {
    buffer: S,
}

impl<T> PacketDecoder<T> {
    pub fn new(t: T) -> Self {
        Self { buffer: t }
    }
}

#[derive(Debug /*Serializable*/)]
pub enum PacketDecodingError {
    InvalidEnumKeyName,
    InvalidEnumKeyIndex,
    NotEnoughBytesInTheBuffer,
    TooLarge,
    InvalidUtf8,
    Custom,
    NonMaxButMax,
    NonZeroButZero,
}

impl DecodeError for PacketDecodingError {
    fn invalid_enum_variant_name() -> Self {
        Self::InvalidEnumKeyName
    }

    fn invalid_enum_variant_index() -> Self {
        Self::InvalidEnumKeyIndex
    }

    fn not_enough_bytes_in_the_buffer() -> Self {
        Self::NotEnoughBytesInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn custom() -> Self {
        Self::Custom
    }

    fn invalid_utf8() -> Self {
        Self::InvalidUtf8
    }

    fn nonmax_but_max() -> Self {
        Self::NonMaxButMax
    }

    fn nonzero_but_zero() -> Self {
        Self::NonZeroButZero
    }
}

impl<T: Buf<u8>> Decoder for PacketDecoder<T> {
    type Error = PacketDecodingError;

    type TupleDecoder = Self;
    type StructDecoder = Self;
    type SequenceDecoder = Self;

    deserialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128);

    fn decode_tuple(&mut self) -> Result<&mut PacketDecoder<T>, PacketDecodingError> {
        Ok(self)
    }

    fn decode_struct(&mut self) -> Result<&mut PacketDecoder<T>, PacketDecodingError> {
        Ok(self)
    }

    fn decode_seq(&mut self) -> Result<&mut PacketDecoder<T>, PacketDecodingError> {
        Ok(self)
    }

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error> {
        self.decode_varint()
    }

    fn decode_enum(&mut self, _enum_name: &'static str) -> Result<EnumIdentifier, Self::Error> {
        Ok(EnumIdentifier::Index(self.decode_varint()?))
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        let mut place = MaybeUninit::uninit();
        self.decode_bool(&mut place)?;
        Ok(unsafe { place.assume_init() })
    }

    fn decode_bool(&mut self, place: &mut MaybeUninit<bool>) -> Result<(), Self::Error> {
        let place: &mut MaybeUninit<u8> = unsafe { transmute(place) };
        self.decode_u8(place)?;
        if unsafe { transmute::<_, u8>(*place) } > 1 {
            *place = MaybeUninit::new(1);
        } else {
        }
        Ok(())
    }

    fn decode_str(&mut self, place: &mut MaybeUninit<&str>) -> Result<(), Self::Error> {
        let len = self.decode_seq_len()?;
        let read = self.buffer.read(len);
        if read.len() != len {
            Err(DecodeError::not_enough_bytes_in_the_buffer())?;
        }
        *place = MaybeUninit::new(unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(read.as_ptr(), len))
                .map_err(|_err| DecodeError::invalid_utf8())?
        });
        Ok(())
    }

    fn decode_bytes(&mut self) -> Result<&[u8], Self::Error> {
        let len = self.decode_seq_len()?;
        let read = self.read(len);
        if read.len() != len {
            Err(DecodeError::not_enough_bytes_in_the_buffer())?
        }
        Ok(read)
    }

    fn decode_var_i32(&mut self, place: &mut MaybeUninit<i32>) -> Result<(), Self::Error> {
        *place = MaybeUninit::new(self.decode_varint()? as i32);
        Ok(())
    }
}

impl<S: Buf<u8>> CompositeDecoder for PacketDecoder<S> {
    type Error = PacketDecodingError;

    fn decode_element<D: Decode>(&mut self, place: &mut MaybeUninit<D>) -> Result<(), Self::Error> {
        D::decode_in_place(&mut *self, place)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<S: Buf<u8>> PacketDecoder<S> {
    fn decode_varint(&mut self) -> Result<usize, <Self as Decoder>::Error> {
        let buf = unsafe { self.buffer.get_continuous(self.buffer.remaining()) };
        let (len, read_len) = fastvarint::VarInt::decode_var_int::<_, &'static str>(|index| {
            Ok(buf.get(index).map(|v| *v))
        })
        .map_err(|err| match err {
            fastvarint::DecodeVarIntError::NotEnoughBytesInTheBuffer => {
                DecodeError::not_enough_bytes_in_the_buffer()
            }
            fastvarint::DecodeVarIntError::TooLarge => DecodeError::too_large(),
            fastvarint::DecodeVarIntError::Custom(_) => {
                DecodeError::not_enough_bytes_in_the_buffer()
            }
        })?;
        self.buffer.advance(read_len + 1);
        Ok(*len as usize)
    }
}

impl<S: Buf<u8>> PacketEncoder<S> {
    fn encode_varint(&mut self, v: i32) -> Result<(), <Self as Encoder>::Error> {
        if v < 255 {
            self.try_write(&[v as u8])
                .map_err(|WriteBufferError::BufferFull| {
                    EncodeError::not_enough_space_in_the_buffer()
                })?;
        } else {
            VarInt::from(v)
                .encode_var_int(|v| self.buffer.try_write(v))
                .map_err(|WriteBufferError::BufferFull| {
                    PacketEncodingError::FullOfCapacityInBuffer
                })?;
        }
        Ok(())
    }
}

impl<S> const PrimitiveTypeSizeChecker for PacketEncoder<S> {}

impl<S> const PrimitiveTypeSizeChecker for PacketDecoder<S> {}

impl<S: Buf<u8>> WriteBuf<u8> for PacketEncoder<S> {
    fn write(&mut self, data: &[u8]) {
        self.buffer.write(data)
    }

    fn try_write(&mut self, data: &[u8]) -> Result<(), WriteBufferError> {
        self.buffer.try_write(data)
    }

    fn remaining_space(&self) -> usize {
        self.buffer.remaining_space()
    }

    fn filled_pos(&self) -> usize {
        self.buffer.filled_pos()
    }

    unsafe fn set_filled_pos(&mut self, filled_pos: usize) {
        self.buffer.set_filled_pos(filled_pos);
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl<S: Buf<u8>> ReadBuf<u8> for PacketDecoder<S> {
    fn read(&mut self, len: usize) -> &[u8] {
        self.buffer.read(len)
    }

    unsafe fn get_continuous(&self, len: usize) -> &[u8] {
        self.buffer.get_continuous(len)
    }

    unsafe fn get_continuous_mut(&mut self, len: usize) -> &mut [u8] {
        self.buffer.get_continuous_mut(len)
    }

    fn remaining(&self) -> usize {
        self.buffer.remaining()
    }

    fn advance(&mut self, len: usize) {
        self.buffer.advance(len)
    }

    fn pos(&self) -> usize {
        self.buffer.pos()
    }

    unsafe fn set_pos(&mut self, pos: usize) {
        self.buffer.set_pos(pos)
    }
}

impl<S: Chunk<u8>> Chunk<u8> for PacketEncoder<S> {
    fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }
}

impl<S: Chunk<u8>> Chunk<u8> for PacketDecoder<S> {
    fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }
}
