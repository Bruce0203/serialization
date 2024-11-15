use fastbuf::WriteBuf;
use fastvarint::{EncodeVarInt, VarInt};

pub struct SerialDescriptor {
    serial_name: &'static str,
}

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

pub trait Encode {
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder;
}

pub trait Decode: Sized {
    fn decode<D>(&self, decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder;
}

pub trait Encoder: Sized {
    type Error;

    fn encode_element<E>(&mut self, v: &E) -> Result<(), Self::Error>
    where
        E: Encode;

    fn encode_struct(self) -> Result<Self, Self::Error>;
    fn end_struct(self) -> Result<Self, Self::Error>;

    fn encode_seq(self, len: usize) -> Result<Self, Self::Error>;
    fn end_collection(self) -> Result<Self, Self::Error>;

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error>;

    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error>;

    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error>;

    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    fn encode_enum_variant(
        &mut self,
        name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error>;

    fn encode_some(&mut self) -> Result<(), Self::Error>;
    fn encode_none(&mut self) -> Result<(), Self::Error>;
}

pub trait Decoder: Sized {
    type Error;

    fn begin_struct(self) -> Result<Self, Self::Error>;
    fn end_struct(self) -> Result<(), Self::Error>;

    fn decode_collection_size(&mut self) -> Result<usize, Self::Error>;

    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    fn decode_enum(&mut self) -> Result<usize, Self::Error>;
    fn decode_is_some(&mut self) -> Result<bool, Self::Error>;
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct PacketEncoder<S> {
    buffer: S,
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

    fn encode_struct(self) -> Result<Self, Self::Error> {
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

    fn encode_seq(self, len: usize) -> Result<Self, Self::Error> {
        VarInt::from(len)
            .encode_var_int(|v| self.buffer.try_write(v))
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)?;
        Ok(self)
    }

    fn encode_element<E>(&mut self, v: &E) -> Result<(), Self::Error>
    where
        E: Encode,
    {
        v.encode(&mut **self)
    }

    fn end_collection(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use fastbuf::{Buffer, ReadBuf};

    use crate::{Encode, Encoder, PacketEncoder};

    #[test]
    fn test_struct() {
        pub struct TestStruct {
            v1: u8,
            v2: Option<TestStruct2>,
        }

        pub struct TestStruct2 {
            v1: u16,
            v2: u8,
            vec: Vec<u8>,
        }

        impl Encode for TestStruct {
            fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
            where
                E: Encoder,
            {
                let mut enc = encoder.encode_struct()?;
                enc.encode_element(&self.v1)?;
                enc.encode_element(&self.v2)?;
                enc.end_struct()?;
                Ok(())
            }
        }

        impl Encode for TestStruct2 {
            fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
            where
                E: Encoder,
            {
                let mut enc = encoder.encode_struct()?;
                enc.encode_element(&self.v1)?;
                enc.encode_element(&self.v2)?;
                enc.encode_element(&self.vec)?;
                enc.end_struct()?;
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
                col.end_collection()?;
                Ok(())
            }
        }

        impl Encode for u8 {
            fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
            where
                E: Encoder,
            {
                encoder.encode_u8(*self)
            }
        }

        impl Encode for u16 {
            fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
            where
                E: Encoder,
            {
                encoder.encode_u16(*self)
            }
        }

        let mut encoder = PacketEncoder {
            buffer: Buffer::<255>::new(),
        };
        TestStruct {
            v1: 123,
            v2: Some(TestStruct2 {
                v1: 123,
                v2: 234,
                vec: vec![1, 2, 3],
            }),
        }
        .encode(&mut encoder)
        .unwrap();
        let data = [123, 1, 0, 123, 234, 3, 1, 2, 3];
        assert_eq!(encoder.get_continuous(data.len()), data);
    }
}
