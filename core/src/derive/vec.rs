use core::mem::{transmute, MaybeUninit};

use fastbuf::{ReadBuf, WriteBuf, WriteBufferError};

use crate::{
    concatenated_neighboring_sized_of, CompositeDecoder, CompositeEncoder, Decode, DecodeError,
    Decoder, Encode, EncodeError, Encoder, SerialDescriptor, SerialSize,
};

impl<T: Encode + const SerialDescriptor> Encode for Vec<T>
where
    [(); T::SIZES_LEN]:,
{
    default fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let col = encoder.encode_seq(self.len())?;
        let sizes = const { concatenated_neighboring_sized_of::<T, E>() }.as_slice();
        if sizes.len() == 1 {
            match sizes[0] {
                SerialSize::Sized(range) => {
                    debug_assert_eq!(range.start, 0);
                    debug_assert_eq!(range.end, size_of::<T>());
                    col.try_write(unsafe {
                        core::slice::from_raw_parts(
                            self.as_ptr() as *const u8,
                            range.end * self.len(),
                        )
                    })
                    .map_err(|WriteBufferError::BufferFull| {
                        EncodeError::not_enough_space_in_the_buffer()
                    })?;
                    col.end()?;
                    return Ok(());
                }
                _ => {}
            }
        }
        for v in self.iter() {
            col.encode_element(v)?;
        }
        col.end()?;
        Ok(())
    }
}

impl Encode for Vec<u8> {
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let col = encoder.encode_seq(self.len())?;
        col.try_write(&self)
            .map_err(|WriteBufferError::BufferFull| {
                EncodeError::not_enough_space_in_the_buffer()
            })?;
        col.end()?;
        Ok(())
    }
}

impl<T: Decode + const SerialDescriptor> Decode for Vec<T>
where
    [(); T::SIZES_LEN]:,
{
    default fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let sizes = const { concatenated_neighboring_sized_of::<T, D>() }.as_slice();
        let mut vec: Vec<T> = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        if sizes.len() == 1 {
            match sizes[0] {
                SerialSize::Sized(range) => {
                    debug_assert_eq!(range.start, 0);
                    debug_assert_eq!(range.end, size_of::<T>());
                    let read = seq.read(range.end * len);
                    if read.len() != range.end * len {
                        return Err(DecodeError::not_enough_bytes_in_the_buffer());
                    }
                    unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, range.end * len) }
                        .copy_from_slice(read);
                    seq.end()?;
                    unsafe { vec.set_len(len) };
                    *out = MaybeUninit::new(vec);
                    return Ok(());
                }
                _ => {}
            }
        }
        for i in 0..len {
            let value_place: &mut MaybeUninit<T> = unsafe { transmute(ptr.wrapping_add(i)) };
            match seq.decode_element(value_place) {
                Ok(()) => {}
                Err(err) => {
                    unsafe { vec.set_len(i) };
                    return Err(err);
                }
            };
        }
        seq.end()?;
        unsafe { vec.set_len(len) };
        *out = MaybeUninit::new(vec);
        Ok(())
    }
}

impl Decode for Vec<u8> {
    default fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let src = seq.read(len);
        if src.len() != len {
            return Err(DecodeError::not_enough_bytes_in_the_buffer());
        }
        let mut vec = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        unsafe {
            core::slice::from_raw_parts_mut(ptr as *mut _ as *mut u8, len).copy_from_slice(src);
        };
        seq.end()?;
        unsafe { vec.set_len(len) };
        *out = MaybeUninit::new(vec);
        Ok(())
    }
}
