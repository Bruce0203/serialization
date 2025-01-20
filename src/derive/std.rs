use crate::Encode;

impl Encode for () {
    fn encode<E: crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}
