use std::mem::MaybeUninit;

use serialization::{CompositeDecoder, CompositeEncoder, Decoder, Encoder};

pub struct Coder<T>(pub T);

/// Copies `N` or `n` bytes from `src` to `dst` depending on if `src` lies within a memory page.
/// https://stackoverflow.com/questions/37800739/is-it-safe-to-read-past-the-end-of-a-buffer-within-the-same-page-on-x86-and-x64
/// # Safety
/// Same as [`std::ptr::copy_nonoverlapping`] but with the additional requirements that
/// `n != 0 && n <= N` and `dst` has room for a `[T; N]`.
/// Is a macro instead of an `#[inline(always)] fn` because it optimizes better.
macro_rules! unsafe_wild_copy {
    // pub unsafe fn wild_copy<T, const N: usize>(src: *const T, dst: *mut T, n: usize) {
    ([$T:ident; $N:expr], $src:ident, $dst:ident, $n:expr) => {
        debug_assert!($n != 0 && $n <= $N);

        let page_size = 4096;
        let read_size = core::mem::size_of::<[$T; $N]>();
        let within_page = $src as usize & (page_size - 1) < (page_size - read_size) && cfg!(all(
            // Miri doesn't like this.
            not(miri),
            // cargo fuzz's memory sanitizer complains about buffer overrun.
            // Without nightly we can't detect memory sanitizers, so we check debug_assertions.
            not(debug_assertions),
            // x86/x86_64/aarch64 all have min page size of 4096, so reading past the end of a non-empty
            // buffer won't page fault.
            any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
        ));

        if within_page {
            *($dst as *mut core::mem::MaybeUninit<[$T; $N]>) = core::ptr::read($src as *const core::mem::MaybeUninit<[$T; $N]>);
        } else {
            $src.copy_to_nonoverlapping($dst, $n);
        }
    }
}

impl serialization::BinaryEncoder for Coder<*mut u8> {
    fn encode_slice<const N: usize>(&mut self, src: &[u8; N]) {
        let dst = self.0;
        let src = src.as_ptr();
        self.0 = unsafe { self.0.byte_add(N) };
        unsafe {
            unsafe_wild_copy!([u8; N], src, dst, N);
        }
    }
}

impl<T> Encoder for Coder<T>
where
    Self: serialization::BinaryEncoder,
{
    type Error = EncodeError;

    type TupleEncoder = Self;

    type StructEncoder = Self;

    type SequenceEncoder = Self;

    fn encode_u8(&mut self, v: &u8) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i8(&mut self, v: &i8) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u16(&mut self, v: &u16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i16(&mut self, v: &i16) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u32(&mut self, v: &u32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i32(&mut self, v: &i32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u64(&mut self, v: &u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i64(&mut self, v: &i64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_u128(&mut self, v: &u128) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_i128(&mut self, v: &i128) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_usize(&mut self, v: &usize) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_isize(&mut self, v: &isize) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f32(&mut self, v: &f32) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_f64(&mut self, v: &f64) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, v: &bool) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_tuple(&mut self) -> Result<&mut Self::TupleEncoder, Self::Error> {
        todo!()
    }

    fn encode_struct(&mut self) -> Result<&mut Self::StructEncoder, Self::Error> {
        todo!()
    }

    fn encode_seq(&mut self, len: usize) -> Result<&mut Self::SequenceEncoder, Self::Error> {
        todo!()
    }

    fn encode_enum_variant_key(
        &mut self,
        enum_name: &'static str,
        variant_name: &'static str,
        variant_index: usize,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_some(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_none(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_str(&mut self, v: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<T> CompositeEncoder for Coder<T>
where
    Self: serialization::BinaryEncoder,
{
    type Error = EncodeError;

    fn encode_element<E: serialization::Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

pub enum EncodeError {
    NotEnoughSpaceInTheBuffer,
    TooLarge,
    Custom,
}

impl serialization::EncodeError for EncodeError {
    fn not_enough_space_in_the_buffer() -> Self {
        Self::NotEnoughSpaceInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn custom() -> Self {
        Self::Custom
    }
}

impl<T> Decoder for Coder<T>
where
    Self: serialization::BinaryDecoder,
{
    type Error = DecodeError;

    type TupleDecoder = Self;

    type StructDecoder = Self;

    type SequenceDecoder = Self;

    fn decode_u8(&mut self, place: &mut MaybeUninit<u8>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i8(&mut self, place: &mut MaybeUninit<i8>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u16(&mut self, place: &mut MaybeUninit<u16>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i16(&mut self, place: &mut MaybeUninit<i16>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u32(&mut self, place: &mut MaybeUninit<u32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i32(&mut self, place: &mut MaybeUninit<i32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u64(&mut self, place: &mut MaybeUninit<u64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i64(&mut self, place: &mut MaybeUninit<i64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_u128(&mut self, place: &mut MaybeUninit<u128>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_i128(&mut self, place: &mut MaybeUninit<i128>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_usize(&mut self, place: &mut MaybeUninit<usize>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_isize(&mut self, place: &mut MaybeUninit<isize>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_f32(&mut self, place: &mut MaybeUninit<f32>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_f64(&mut self, place: &mut MaybeUninit<f64>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_bool(&mut self, place: &mut MaybeUninit<bool>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_str(&mut self, place: &mut std::mem::MaybeUninit<&str>) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_bytes<'a>(&mut self) -> Result<&'a [u8], Self::Error> {
        todo!()
    }

    fn decode_var_i32(
        &mut self,
        place: &mut std::mem::MaybeUninit<i32>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_tuple(&mut self) -> Result<&mut Self::TupleDecoder, Self::Error> {
        todo!()
    }

    fn decode_struct(&mut self) -> Result<&mut Self::StructDecoder, Self::Error> {
        todo!()
    }

    fn decode_seq(&mut self) -> Result<&mut Self::SequenceDecoder, Self::Error> {
        todo!()
    }

    fn decode_seq_len(&mut self) -> Result<usize, Self::Error> {
        todo!()
    }

    fn decode_enum(
        &mut self,
        enum_name: &'static str,
    ) -> Result<serialization::EnumIdentifier, Self::Error> {
        todo!()
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl<T> CompositeDecoder for Coder<T>
where
    Self: serialization::BinaryDecoder,
{
    type Error = DecodeError;

    fn decode_element<D: serialization::Decode>(
        &mut self,
        place: &mut MaybeUninit<D>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub enum DecodeError {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    InvalidEnumVariantName,
    InvalidEnumVarirantIndex,
    Custom,
}

impl serialization::DecodeError for DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self {
        Self::NotEnoughBytesInTheBuffer
    }

    fn too_large() -> Self {
        Self::TooLarge
    }

    fn invalid_enum_variant_name() -> Self {
        Self::InvalidEnumVariantName
    }

    fn invalid_enum_variant_index() -> Self {
        Self::InvalidEnumVarirantIndex
    }

    fn custom() -> Self {
        Self::Custom
    }

    fn invalid_utf8() -> Self {
        todo!()
    }

    fn nonmax_but_max() -> Self {
        todo!()
    }

    fn nonzero_but_zero() -> Self {
        todo!()
    }
}
