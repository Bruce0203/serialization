use concat_idents_bruce0203::concat_idents;

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error>;
}

pub trait Decode<'de>: Sized {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error>;
}

pub trait CompositeEncoder {
    type Error;
    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error>;
    fn end(self) -> Result<(), Self::Error>;
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
            fn fn_name(&mut self) -> Result<$type, Self::Error>;
        });
    )*};
}

pub trait Encoder: Sized {
    type Error;
    type TupleEncoder: CompositeEncoder<Error = Self::Error>;
    type StructEncoder: CompositeEncoder<Error = Self::Error>;
    type SequenceEncoder: CompositeEncoder<Error = Self::Error>;

    encode_value!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

    fn encode_tuple(self) -> Result<Self::TupleEncoder, Self::Error>;
    fn encode_struct(self) -> Result<Self::StructEncoder, Self::Error>;
    fn encode_seq(self, len: usize) -> Result<Self::SequenceEncoder, Self::Error>;

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

pub trait CompositeDecoder<'de> {
    type Error;
    fn decode_element<D: Decode<'de>>(&mut self) -> Result<D, Self::Error>;
    fn end(self) -> Result<(), Self::Error>;
}

pub enum EnumIdentifier {
    Name(&'static str),
    Index(usize),
}

pub trait DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self;
    fn too_large() -> Self;
    fn invalid_enum_variant_name() -> Self;
    fn invalid_enum_variant_index() -> Self;
    fn custom() -> Self;
}

pub trait Decoder<'de>: Sized {
    type Error: DecodeError;
    type TupleDecoder: CompositeDecoder<'de, Error = Self::Error>;
    type StructDecoder: CompositeDecoder<'de, Error = Self::Error>;
    type SequenceDecoder: CompositeDecoder<'de, Error = Self::Error>;

    decode_value!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

    fn decode_str(&mut self) -> Result<&'de str, Self::Error>;
    fn decode_bytes(&mut self) -> Result<&[u8], Self::Error>;
    fn decode_var_i32(&mut self) -> Result<i32, Self::Error>;

    fn decode_tuple(self) -> Result<Self::TupleDecoder, Self::Error>;
    fn decode_struct(self) -> Result<Self::StructDecoder, Self::Error>;
    fn decode_seq(self) -> Result<Self::SequenceDecoder, Self::Error>;

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error>;
    fn decode_enum(&mut self, enum_name: &'static str) -> Result<EnumIdentifier, Self::Error>;

    fn decode_is_some(&mut self) -> Result<bool, Self::Error>;
}
