pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error>;
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error>;
}

pub trait Encoder: Sized {
    type Error;

    fn begin_tuple(self) -> Result<Self, Self::Error>;
    fn end_tuple(self) -> Result<Self, Self::Error>;

    fn begin_struct(self) -> Result<Self, Self::Error>;
    fn end_struct(self) -> Result<Self, Self::Error>;

    fn begin_seq(self, len: usize) -> Result<Self, Self::Error>;
    fn end_seq(self) -> Result<Self, Self::Error>;

    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error>;

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
        variant_name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error>;

    fn encode_some(&mut self) -> Result<(), Self::Error>;
    fn encode_none(&mut self) -> Result<(), Self::Error>;
}

pub trait Decoder: Sized {
    type Error;

    fn begin_tuple(self) -> Result<Self, Self::Error>;
    fn end_tuple(self) -> Result<Self, Self::Error>;

    fn begin_struct(self) -> Result<Self, Self::Error>;
    fn end_struct(self) -> Result<(), Self::Error>;

    fn begin_seq(self) -> Result<usize, Self::Error>;
    fn end_seq(self) -> Result<Self, Self::Error>;

    fn decode_element<E>(&mut self) -> Result<E, Self::Error>;

    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;

    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;

    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;

    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;

    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    fn decode_enum_index(&mut self) -> Result<usize, Self::Error>;
    fn decode_is_some(&mut self) -> Result<bool, Self::Error>;
}
