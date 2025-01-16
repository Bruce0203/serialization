use crate::Encode;

impl<T> Encode for Vec<T> {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        todo!()
    }
}
