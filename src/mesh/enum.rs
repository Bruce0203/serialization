use crate::Encode;

pub struct Enum<T>(T);

impl<T> Encode for Enum<T> {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        todo!()
    }
}
