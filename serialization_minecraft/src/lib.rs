use std::mem::MaybeUninit;

use fastbuf::{ReadBuf, WriteBuf};
use fastvarint::{DecodeVarInt, EncodeVarInt, VarInt};
use serialization::{
    CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, Encoder,
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

macro_rules! serialize_bytes {
    ($buffer:expr, $v:expr) => {
        $buffer
            .try_write($v)
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)
    };
}

macro_rules! serialize_num {
    ($buffer:expr, $v:expr) => {
        serialize_bytes!($buffer, &$v.to_be_bytes())
    };
}

macro_rules! deserialize_num2 {
    ($buffer:expr, $type:ident) => {
        concat_idents!(fn_name = decode_, $type, { decoder.fn_name() })
    };
}

impl<'a, S: WriteBuf> Encoder for &'a mut PacketEncoder<S> {
    type Error = PacketEncodingError;
    type TupleEncoder = Self;
    type StructEncoder = Self;
    type SequenceEncoder = Self;

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
        VarInt::from(len)
            .encode_var_int(|v| self.buffer.try_write(v))
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)?;
        Ok(self)
    }

    fn encode_tuple(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.try_write(&[v as u8])
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)
    }

    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_u128(&mut self, v: u128) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_i128(&mut self, v: i128) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        serialize_num!(self, v)
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

macro_rules! deserialize_num {
    ($buffer:expr, $type:ty) => {{
        let buf = $buffer.read(size_of::<$type>());
        if buf.len() != size_of::<$type>() {
            Err(PacketDecodingError::NotEnoughBytesInTheBuffer)?;
        }
        #[allow(invalid_value)]
        let mut slice = [unsafe { MaybeUninit::<u8>::uninit().assume_init() }; size_of::<$type>()];
        slice.copy_from_slice(buf);
        Ok(<$type>::from_be_bytes(slice))
    }};
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

impl<T: ReadBuf> Decoder for &mut PacketDecoder<T> {
    type Error = PacketDecodingError;

    type TupleDecoder = Self;
    type StructDecoder = Self;
    type SequenceDecoder = Self;

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

    fn decode_enum(
        &mut self,
        _enum_name: &'static str,
    ) -> Result<serialization::EnumIdentifier, Self::Error> {
        Ok(serialization::EnumIdentifier::Index(self.decode_varint()?))
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        Ok(if self.decode_u8()? == 0 { false } else { true })
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        deserialize_num!(self, u8)
    }

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        deserialize_num!(self, i8)
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        deserialize_num!(self, u16)
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        deserialize_num!(self, i16)
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        deserialize_num!(self, u32)
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        deserialize_num!(self, i32)
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        deserialize_num!(self, u64)
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        deserialize_num!(self, i64)
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        deserialize_num!(self, u128)
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        deserialize_num!(self, i128)
    }

    fn decode_usize(&mut self) -> Result<usize, Self::Error> {
        deserialize_num!(self, usize)
    }

    fn decode_isize(&mut self) -> Result<isize, Self::Error> {
        deserialize_num!(self, isize)
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        deserialize_num!(self, f32)
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        deserialize_num!(self, f64)
    }
}

impl<S: ReadBuf> CompositeDecoder for &mut PacketDecoder<S> {
    type Error = <Self as Decoder>::Error;

    fn decode_element<D: Decode>(&mut self) -> Result<D, Self::Error> {
        D::decode(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, S: ReadBuf> PacketDecoder<S> {
    fn decode_varint(&mut self) -> Result<usize, <&'a mut Self as Decoder>::Error> {
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
