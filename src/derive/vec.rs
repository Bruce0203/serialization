use crate::{CompositeEncoder, Encode};

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let seq = encoder.encode_seq(self.len())?;
        for elem in self.iter() {
            seq.encode_element(elem)?;
        }
        seq.end()?;
        Ok(())
    }
}
