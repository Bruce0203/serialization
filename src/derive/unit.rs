use std::mem::MaybeUninit;

use crate::{Decode, Encode};

impl Encode for () {
    fn encode<E: crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl Decode for () {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}
