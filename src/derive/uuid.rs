#[cfg(feature = "uuid")]
impl Encode for uuid::Uuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_bytes(self.as_bytes())
    }
}

#[cfg(feature = "uuid")]
impl Decode for uuid::Uuid {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        decoder.decode_u128(unsafe { const_transmute(place) })
    }
}
