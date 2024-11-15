use fastbuf::{Buf, Buffer, WriteBuf};

pub struct SerialDescriptor {
    serial_name: &'static str,
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

pub trait Encoder {
    type Error;

    fn begin_struct(&mut self) -> Result<(), Self::Error>;
    fn end_struct(&mut self) -> Result<(), Self::Error>;

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

    fn encode_enum(&mut self) -> Result<(), Self::Error>;
    fn encode_some(&mut self) -> Result<(), Self::Error>;
    fn encode_none(&mut self) -> Result<(), Self::Error>;
}

pub trait Decoder: Sized {
    type Error;

    fn begin_struct(&mut self) -> Result<Self, Self::Error>;
    fn end_struct(&mut self) -> Result<(), Self::Error>;

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

impl<S: WriteBuf> Encoder for &mut PacketEncoder<S> {
    type Error = PacketEncodingError;

    fn begin_struct(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn end_struct(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.try_write(&[v])
            .map_err(|()| PacketEncodingError::NotEnoughBuffer)
    }

    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_enum(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_some(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_none(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use fastbuf::{Buffer, ReadBuf};

    use crate::{Encode, Encoder, PacketEncoder};

    pub struct TestStruct {
        v1: u8,
    }

    impl Encode for TestStruct {
        fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
        where
            E: Encoder,
        {
            encoder.encode_u8(self.v1)?;
            Ok(())
        }
    }

    #[test]
    fn test_struct() {
        let mut encoder = PacketEncoder {
            buffer: Buffer::<255>::new(),
        };
        TestStruct { v1: 123 }.encode(&mut encoder).unwrap();
        assert_eq!(encoder.get_continuous(1), &[123]);
    }
}
