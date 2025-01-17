use crate::Encode;

impl Encode for String {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u8(&(self.len() as u8))?;
        encoder.encode_bytes(self.as_bytes())?;
        Ok(())
    }
}

impl Encode for &'static str {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}
