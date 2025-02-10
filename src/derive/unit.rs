use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{Decode, Decoder, Encode, Encoder};

impl Encode for () {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl Decode for () {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<T> Decode for PhantomData<T> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}
