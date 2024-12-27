use core::{
    marker::PhantomData,
    mem::{transmute, ManuallyDrop, MaybeUninit},
};

use crate::{CompositeDecoder, CompositeEncoder, Decode, Decoder, Encode, Encoder};

impl Encode for () {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
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

impl<T: Encode> Encode for Option<T> {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        if let Some(v) = self {
            encoder.encode_some()?;
            v.encode(encoder)?;
        } else {
            encoder.encode_none()?;
        }
        Ok(())
    }
}

const fn get_result_offset_of<T, Error>() -> usize {
    let result: ManuallyDrop<Result<T, Error>> =
        ManuallyDrop::new(Ok(unsafe { MaybeUninit::zeroed().assume_init() }));
    const RESULT_VARIANT_INDEX_SIZE: usize = 1;
    let data = unsafe {
        core::slice::from_raw_parts(
            &result as *const _ as *const u8,
            size_of::<T>() + RESULT_VARIANT_INDEX_SIZE,
        )
    };
    let mut i = 0;
    while i < data.len() {
        if data[i] != 0 {
            return i;
        }
        i += 1;
    }
    panic!("error while getting offset of result's variant index");
}

const fn get_option_offset_of<T>() -> usize {
    let option: ManuallyDrop<Option<T>> =
        ManuallyDrop::new(Some(unsafe { MaybeUninit::zeroed().assume_init() }));
    const OPTION_VARIANT_INDEX_SIZE: usize = 1;
    let data = unsafe {
        core::slice::from_raw_parts(
            &option as *const _ as *const u8,
            size_of::<T>() + OPTION_VARIANT_INDEX_SIZE,
        )
    };
    let mut i = 0;
    while i < data.len() {
        if data[i] != 0 {
            return i;
        }
        i += 1;
    }
    panic!("error while getting offset of option's variant index");
}

impl<T: Decode> Decode for Option<T> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        if decoder.decode_is_some()? {
            unsafe {
                *(out.assume_init_mut() as *mut _ as *mut u8)
                    .wrapping_add(get_option_offset_of::<T>()) = 1
            };

            let value_out = unsafe { transmute(out.assume_init_mut().as_mut().unwrap_unchecked()) };
            T::decode_in_place(decoder, value_out)?;
        } else {
            *out = MaybeUninit::new(None);
        }
        Ok(())
    }
}

impl<T: Encode, Error: Encode> Encode for Result<T, Error> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            Ok(value) => {
                encoder.encode_some()?;
                value.encode(encoder)
            }
            Err(value) => {
                encoder.encode_none()?;
                value.encode(encoder)
            }
        }
    }
}

impl<T: Decode, Error: Decode> Decode for Result<T, Error> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        if decoder.decode_is_some()? {
            unsafe {
                *(out.assume_init_mut() as *mut _ as *mut u8)
                    .wrapping_add(get_result_offset_of::<T, Error>()) = 1
            };
            let value_out = unsafe { transmute(out.assume_init_mut().as_mut().unwrap_unchecked()) };
            T::decode_in_place(decoder, value_out)?;
        } else {
            *out = MaybeUninit::new(Err(unsafe { MaybeUninit::uninit().assume_init() }));
            let value_out =
                unsafe { transmute(out.assume_init_mut().as_mut().unwrap_err_unchecked()) };
            Error::decode_in_place(decoder, value_out)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for &T {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        (*self).encode(encoder)
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<T> Decode for PhantomData<T> {
    fn decode_in_place<D: Decoder>(
        _decoder: &mut D,
        _out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}

impl Encode for &[u8] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_bytes(self)
    }
}

#[cfg(feature = "std")]
impl<'a, T: Encode + Clone> Encode for std::borrow::Cow<'a, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            std::borrow::Cow::Borrowed(value) => value.encode(encoder),
            std::borrow::Cow::Owned(value) => value.encode(encoder),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, T: Decode + Clone> Decode for std::borrow::Cow<'a, T> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        *out = MaybeUninit::new(std::borrow::Cow::Owned(unsafe {
            MaybeUninit::uninit().assume_init()
        }));
        let value_place: &mut MaybeUninit<T> = unsafe { transmute(out.assume_init_mut().to_mut()) };
        T::decode_in_place(decoder, value_place)?;
        Ok(())
    }
}

impl<T: Encode, const CAP: usize> Encode for [T; CAP] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let tup = encoder.encode_tuple()?;
        for v in self.iter() {
            tup.encode_element(v)?;
        }
        tup.end()
    }
}

impl<T: Decode, const CAP: usize> Decode for [T; CAP] {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let tup = decoder.decode_tuple()?;
        for i in 0..CAP {
            let value_place: &mut MaybeUninit<T> =
                unsafe { transmute(out.assume_init_mut().get_unchecked_mut(i)) };
            tup.decode_element(value_place)?;
        }
        tup.end()?;
        Ok(())
    }
}
