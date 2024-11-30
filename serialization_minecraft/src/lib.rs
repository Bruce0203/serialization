#![feature(str_from_raw_parts)]

use core::str;
use std::mem::MaybeUninit;

use concat_idents::concat_idents;
use fastbuf::{ReadBuf, WriteBuf};
use fastvarint::{DecodeVarInt, EncodeVarInt, VarInt};
use serialization::{
    CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, Encoder,
    EnumIdentifier,
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

#[derive(Debug)]
pub enum PacketEncodingError {
    NotEnoughBuffer,
    Custom,
}

macro_rules! serialize_num {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = encode_, $type, {
            fn fn_name(&mut self, v: $type) -> Result<(), Self::Error> {
                self
                    .try_write(&v.to_be_bytes())
                    .map_err(|()| PacketEncodingError::NotEnoughBuffer)
            }
        });
    )*};
}

macro_rules! deserialize_num {
    ($($type:ty),*) => {$(
        concat_idents!(fn_name = decode_, $type, {
            fn fn_name(&mut self) -> Result<$type, Self::Error> {
                let buf = self.read(size_of::<$type>());
                if buf.len() != size_of::<$type>() {
                    Err(PacketDecodingError::NotEnoughBytesInTheBuffer)?;
                }
                #[allow(invalid_value)]
                let mut slice = [unsafe { MaybeUninit::<u8>::uninit().assume_init() }; size_of::<$type>()];
                slice.copy_from_slice(buf);
                Ok(<$type>::from_be_bytes(slice))
            }
        });
    )*};
}

impl<'a, S: WriteBuf> Encoder for &'a mut PacketEncoder<S> {
    type Error = PacketEncodingError;
    type TupleEncoder = Self;
    type StructEncoder = Self;
    type SequenceEncoder = Self;

    serialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128);

    fn encode_struct(self) -> Result<Self, Self::Error> {
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

    fn encode_seq(self, len: usize) -> Result<Self, Self::Error> {
        self.encode_varint(len as i32)?;
        Ok(self)
    }

    fn encode_tuple(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.try_write(&[v as u8])
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)
    }

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.encode_bytes(v.as_bytes())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        self.encode_seq(v.len())?;
        self.try_write(v)
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)
    }

    fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.encode_varint(v)
    }
}

impl<'a, S: WriteBuf> CompositeEncoder for &'a mut PacketEncoder<S> {
    type Error = <Self as Encoder>::Error;

    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
        v.encode(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
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

#[derive(Debug)]
pub enum PacketDecodingError {
    Custom,
    InvalidEnumKeyName(&'static str),
    InvalidEnumKeyIndex(usize),
    NotEnoughBytesInTheBuffer,
    TooLarge,
}

impl DecodeError for PacketDecodingError {
    fn invalid_enum_variant_name(name: &'static str) -> Self {
        Self::InvalidEnumKeyName(name)
    }

    fn invalid_enum_variant_index(index: usize) -> Self {
        Self::InvalidEnumKeyIndex(index)
    }

    fn not_enough_bytes_in_the_buffer() -> Self {
        Self::NotEnoughBytesInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }
}

impl<'de, T: ReadBuf> Decoder<'de> for &mut PacketDecoder<T> {
    type Error = PacketDecodingError;

    type TupleDecoder = Self;
    type StructDecoder = Self;
    type SequenceDecoder = Self;

    deserialize_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128);

    fn decode_tuple(self) -> Result<Self::TupleDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_struct(self) -> Result<Self::StructDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_seq(self) -> Result<Self::SequenceDecoder, Self::Error> {
        Ok(self)
    }

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error> {
        self.decode_varint()
    }

    fn decode_enum(&mut self, _enum_name: &'static str) -> Result<EnumIdentifier, Self::Error> {
        Ok(EnumIdentifier::Index(self.decode_varint()?))
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        Ok(if self.decode_u8()? == 0 { false } else { true })
    }

    fn decode_str(&mut self) -> Result<&'de str, Self::Error> {
        let len = self.decode_seq_len()?;
        let read = self.buffer.read(len);
        if read.len() != len {
            Err(DecodeError::not_enough_bytes_in_the_buffer())?;
        }
        Ok(unsafe { std::str::from_raw_parts(read.as_ptr(), len) })
    }

    fn decode_bytes(&mut self) -> Result<&[u8], Self::Error> {
        let len = self.decode_seq_len()?;
        let read = self.read(len);
        if read.len() != len {
            Err(DecodeError::not_enough_bytes_in_the_buffer())?
        }
        Ok(read)
    }

    fn decode_var_i32(&mut self) -> Result<i32, Self::Error> {
        Ok(self.decode_varint()? as i32)
    }
}

impl<'de, S: ReadBuf> CompositeDecoder<'de> for &mut PacketDecoder<S> {
    type Error = PacketDecodingError;

    fn decode_element<D: Decode<'de>>(&mut self) -> Result<D, Self::Error> {
        D::decode(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<S: ReadBuf> PacketDecoder<S> {
    fn decode_varint(&mut self) -> Result<usize, <&mut Self as Decoder>::Error> {
        let buf = self.buffer.get_continuous(self.buffer.remaining());
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

impl<S: WriteBuf> PacketEncoder<S> {
    fn encode_varint(&mut self, v: i32) -> Result<(), <&mut Self as Encoder>::Error> {
        VarInt::from(v)
            .encode_var_int(|v| self.buffer.try_write(v))
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)?;
        Ok(())
    }
}
