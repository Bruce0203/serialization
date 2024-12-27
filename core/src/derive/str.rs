use core::mem::{transmute, MaybeUninit};

use fastbuf::{ReadBuf, WriteBuf, WriteBufferError};

use crate::{
    CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, EncodeError, Encoder,
};

impl Decode for &str {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        decoder.decode_str(out)
    }
}

impl Encode for &str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let bytes = self.as_bytes();
        let col = encoder.encode_seq(bytes.len())?;
        col.try_write(bytes)
            .map_err(|WriteBufferError::BufferFull| {
                EncodeError::not_enough_space_in_the_buffer()
            })?;
        col.end()?;
        Ok(())
    }
}

impl Decode for String {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        let out: &mut MaybeUninit<Vec<u8>> = unsafe { transmute(out) };
        let len = decoder.decode_seq_len()?;
        let seq = decoder.decode_seq()?;
        let mut vec: Vec<u8> = Vec::with_capacity(len);
        let ptr = vec.as_mut_ptr();
        let bytes = seq.read(len);
        if bytes.len() != len {
            return Err(DecodeError::not_enough_bytes_in_the_buffer());
        }
        if !bytes.is_ascii() {
            return Err(DecodeError::invalid_utf8());
        }
        unsafe { core::slice::from_raw_parts_mut(ptr, len).copy_from_slice(bytes) };
        seq.end()?;
        unsafe { vec.set_len(len) };
        *out = MaybeUninit::new(vec);
        Ok(())
    }
}
