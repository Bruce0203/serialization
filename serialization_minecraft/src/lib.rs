use fastbuf::WriteBuf;
use fastvarint::{EncodeVarInt, VarInt};
use serialization::{Encode, Encoder};

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

#[derive(Debug)]
pub enum PacketDecodingError {
    Custom,
}

macro_rules! deserialize_num {
    ($buffer:expr, $visitor:expr, $type:ty, $fn_name:ident) => {{
        const BYTES: usize = std::mem::size_of::<$type>() as usize;
        let read = $buffer.read(BYTES);
        #[allow(invalid_value)]
        let mut buf = [unsafe { MaybeUninit::<u8>::uninit().assume_init() }; BYTES];
        buf.copy_from_slice(read);
        if read.len() != BYTES {
            Err(PacketDeserializeError::NotEnoughBytes)
        } else {
            $visitor.$fn_name(<$type>::from_be_bytes(buf))
        }
    }};
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

impl<'a, S: WriteBuf> Encoder for &'a mut PacketEncoder<S> {
    type Error = PacketEncodingError;

    fn begin_struct(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn end_struct(self) -> Result<Self, Self::Error> {
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

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        serialize_num!(self, v)
    }

    fn encode_enum_variant(
        &mut self,
        _name: &'static str,
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

    fn begin_seq(self, len: usize) -> Result<Self, Self::Error> {
        VarInt::from(len)
            .encode_var_int(|v| self.buffer.try_write(v))
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)?;
        Ok(self)
    }

    fn end_seq(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn encode_element<E>(&mut self, v: &E) -> Result<(), Self::Error>
    where
        E: Encode,
    {
        v.encode(&mut **self)
    }

    fn begin_tuple(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn end_tuple(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
}
