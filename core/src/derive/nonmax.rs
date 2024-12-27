#[cfg(feature = "nonmax")]
macro_rules! nonmax {
    ($($type:ty: $inner:ty),*) => {$(
        impl Encode for $type {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                self.get().encode(encoder)
            }
        }

        impl Decode for $type {
            fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
                let place: &mut MaybeUninit<$inner> = unsafe { const_transmute(place) };
                <$inner>::decode(decoder, place)?;
                if unsafe { place.assume_init() } == <$inner>::MAX {
                    return Err(DecodeError::nonmax_but_max());
                }
                Ok(())
            }
        }
    )*};
}

#[cfg(feature = "nonmax")]
nonmax!(
    NonMaxI8: i8,
    NonMaxU8: u8,

    NonMaxU16: u16,
    NonMaxI16: i16,

    NonMaxU32: u32,
    NonMaxI32: i32,

    NonMaxU64: u64,
    NonMaxI64: i64,

    NonMaxU128: u128,
    NonMaxI128: i128,

    NonMaxUsize: usize,
    NonMaxIsize: isize
);

#[cfg(feature = "fastvarint")]
impl Encode for fastvarint::NonMaxI32VarInt {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_var_i32(self.get())
    }
}

#[cfg(feature = "fastvarint")]
impl Decode for fastvarint::NonMaxI32VarInt {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        let place: &mut MaybeUninit<i32> = unsafe { const_transmute(place) };
        decoder.decode_var_i32(place)?;
        if unsafe { place.assume_init() } == i32::MAX {
            return Err(DecodeError::nonmax_but_max());
        }
        Ok(())
    }
}
