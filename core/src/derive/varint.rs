#[cfg(feature = "fastvarint")]
impl Decode for fastvarint::VarInt {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        decoder.decode_var_i32(unsafe { std::mem::transmute(place) })
    }
}

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::VarInt {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_var_i32(**self)
    }
}
