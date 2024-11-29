pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error>;
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error>;
}

pub trait CompositeEncoder {
    type Error;
    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error>;
    fn end(self) -> Result<(), Self::Error>;
}

pub trait Encoder: Sized {
    type Error;
    type TupleEncoder: CompositeEncoder<Error = Self::Error>;
    type StructEncoder: CompositeEncoder<Error = Self::Error>;
    type SequenceEncoder: CompositeEncoder<Error = Self::Error>;

    fn encode_tuple(self) -> Result<Self::TupleEncoder, Self::Error>;
    fn encode_struct(self) -> Result<Self::StructEncoder, Self::Error>;
    fn encode_seq(self, len: usize) -> Result<Self::SequenceEncoder, Self::Error>;

    fn encode_enum_variant_key(
        &mut self,
        enum_name: &'static str,
        variant_name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error>;

    fn encode_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    fn encode_some(&mut self) -> Result<(), Self::Error>;
    fn encode_none(&mut self) -> Result<(), Self::Error>;

    fn encode_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, v: i8) -> Result<(), Self::Error>;

    fn encode_u16(&mut self, v: u16) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, v: i16) -> Result<(), Self::Error>;

    fn encode_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, v: i32) -> Result<(), Self::Error>;

    fn encode_u64(&mut self, v: u64) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    fn encode_u128(&mut self, v: u128) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, v: i128) -> Result<(), Self::Error>;

    fn encode_usize(&mut self, v: usize) -> Result<(), Self::Error>;
    fn encode_isize(&mut self, v: isize) -> Result<(), Self::Error>;

    fn encode_f32(&mut self, v: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, v: f64) -> Result<(), Self::Error>;
}

pub trait CompositeDecoder {
    type Error;
    fn decode_element<D: Decode>(&mut self) -> Result<D, Self::Error>;
    fn end(self) -> Result<(), Self::Error>;
}

pub enum EnumIdentifier {
    Name(&'static str),
    Index(usize),
}

pub trait DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self;
    fn too_large() -> Self;
    fn invalid_enum_variant_name(name: &'static str) -> Self;
    fn invalid_enum_variant_index(index: usize) -> Self;
}

pub trait Decoder: Sized {
    type Error: DecodeError;
    type TupleDecoder: CompositeDecoder<Error = Self::Error>;
    type StructDecoder: CompositeDecoder<Error = Self::Error>;
    type SequenceDecoder: CompositeDecoder<Error = Self::Error>;

    fn decode_tuple(self) -> Result<Self::TupleDecoder, Self::Error>;
    fn decode_struct(self) -> Result<Self::StructDecoder, Self::Error>;
    fn decode_seq(self) -> Result<Self::SequenceDecoder, Self::Error>;

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error>;
    fn decode_enum(&mut self, enum_name: &'static str) -> Result<EnumIdentifier, Self::Error>;

    fn decode_is_some(&mut self) -> Result<bool, Self::Error>;

    fn decode_bool(&mut self) -> Result<bool, Self::Error>;

    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;

    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;

    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;

    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;

    fn decode_u128(&mut self) -> Result<u128, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error>;

    fn decode_usize(&mut self) -> Result<usize, Self::Error>;
    fn decode_isize(&mut self) -> Result<isize, Self::Error>;

    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;
}
