use std::mem::MaybeUninit;

use crate::{
    prelude::{walk_segment, Mesh, SegmentEncoder, SegmentWalker},
    BufRead, BufWrite, Buffer, CompositeDecoder, CompositeEncoder, Decode, Decoder, Encode,
    Encoder, EnumIdentifier,
};

pub struct BinaryCodec {
    buffer: Buffer,
}

pub fn encode<'a, T>(src: &T, dst: &mut [u8]) -> Result<(), <BinaryCodec as Encoder>::Error>
where
    T: Mesh<BinaryCodec, Output: SegmentWalker<T, BinaryCodec, SegmentEncoder>>,
{
    let buffer = Buffer::from(dst);
    let mut codec = BinaryCodec { buffer };
    walk_segment(src, &mut codec)?;
    Ok(())
}

impl BufWrite for BinaryCodec {
    fn write_array<T: Copy, const N: usize>(&mut self, src: &[T; N]) {
        self.buffer.write_array::<T, N>(src);
    }

    fn write_slice<T: Copy>(&mut self, src: &[T]) {
        self.buffer.write_slice::<T>(src);
    }
}

impl BufRead for BinaryCodec {
    fn read_slice<const N: usize>(&mut self, out: &mut MaybeUninit<[u8; N]>) {
        self.buffer.read_slice(out)
    }
}

impl Encoder for BinaryCodec
where
    Self: BufWrite,
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

impl CompositeEncoder for BinaryCodec
where
    Self: BufWrite,
{
    type Error = EncodeError;

    fn encode_element<E: Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
        v.encode(self)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum EncodeError {
    NotEnoughSpaceInTheBuffer,
    TooLarge,
    Custom,
}

impl crate::EncodeError for EncodeError {
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

impl Decoder for BinaryCodec
where
    Self: BufRead,
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

    fn decode_enum(&mut self, enum_name: &'static str) -> Result<EnumIdentifier, Self::Error> {
        todo!()
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl CompositeDecoder for BinaryCodec
where
    Self: BufRead,
{
    type Error = DecodeError;

    fn decode_element<D: Decode>(&mut self, place: &mut MaybeUninit<D>) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum DecodeError {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    InvalidEnumVariantName,
    InvalidEnumVarirantIndex,
    Custom,
}

impl crate::DecodeError for DecodeError {
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

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::{codec::bin::encode, mock::model::foo::Foo};

    extern crate test;

    #[test]
    fn actor() {
        println!();
        #[allow(invalid_value)]
        println!("--------");
        let mut dst: Box<[u8]> = Box::new([0_u8; 1000000]);
        encode(&Foo::default(), &mut dst).unwrap();
        println!("{:?}", &dst[..66]);
        println!("--------");
    }
}

#[cfg(test)]
mod benches {
    use std::hint::black_box;

    use test::Bencher;

    use crate::{bin::encode, mock::model::log::Logs};

    #[bench]
    fn bench_log_model(b: &mut Bencher) {
        let model = Logs::default();
        let mut dst = unsafe { Box::<[u8; 1000000]>::new_uninit().assume_init() } as Box<[u8]>;
        black_box(&model);
        b.iter(|| {
            black_box(encode(&model, &mut dst).unwrap());
        });
        println!("{:?}", &dst[..66]);
        black_box(&dst);
    }

    #[bench]
    fn bench_log_model_with_bitcode(b: &mut Bencher) {
        let model = Logs::default();
        black_box(&model);
        let mut buf = bitcode::Buffer::default();
        b.iter(|| {
            black_box(&buf.encode(&model));
        });
        black_box(&buf);
    }
}
