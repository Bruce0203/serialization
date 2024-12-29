use core::{mem::MaybeUninit, str::FromStr};

use arrayvec::ArrayString;

use crate::{Decode, Decoder, Encode, Encoder};

#[cfg(feature = "arrayvec")]
impl<T: Encode, const CAP: usize> Encode for arrayvec::ArrayVec<T, CAP> {
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let seq = encoder.encode_seq(self.len())?;
        for ele in self.iter() {
            seq.encode_element(ele)?;
        }
        seq.end()?;
        Ok(())
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayVec<u8, CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        self.as_slice().encode(encoder)
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Decode for arrayvec::ArrayVec<u8, CAP> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<T: Decode, const CAP: usize> Decode for arrayvec::ArrayVec<T, CAP> {
    default fn decode<D: Decoder>(
        decoder: &mut D,
        place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Decode for arrayvec::ArrayString<CAP> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Encode for arrayvec::ArrayString<CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let vec = arrayvec::ArrayVec::<u8, CAP>::try_from(self.as_bytes()).unwrap();
        vec.encode(encoder)
    }
}

impl<const CAP: usize> Encode for ArrayString<CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<const CAP: usize> Decode for ArrayString<CAP> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}
