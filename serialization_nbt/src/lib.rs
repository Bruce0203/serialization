#![feature(core_intrinsics)]
#![feature(const_trait_impl)]
#![feature(inline_const_pat)]

use fastbuf::Buf;
use serialization::{BinaryEncoder, EncodeError};

pub struct NbtEncoder<S> {
    buffer: S,
}

pub enum NbtEncodeError {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    Custom,
}

pub struct NbtDecoder<S> {
    buffer: S,
}

impl EncodeError for NbtEncodeError {
    fn not_enough_bytes_in_the_buffer() -> Self {
        Self::NotEnoughBytesInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn custom() -> Self {
        Self::Custom
    }
}

impl<S: Buf> BinaryEncoder for &mut NbtEncoder<S> {
    fn skip_bytes(&mut self, len: usize) {
        unsafe { self.buffer.set_filled_pos(self.filled_pos() + len) };
    }
}
