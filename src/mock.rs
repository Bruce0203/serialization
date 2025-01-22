use std::mem::MaybeUninit;

use crate::{
    prelude::{walk_segment, Mesh, SegmentEncoder, SegmentWalker},
    BufRead, BufWrite, Buffer, CompositeDecoder, CompositeEncoder, Decoder, Encoder,
};

pub struct BinaryCodecMock {
    buffer: Buffer,
}

impl BufWrite for BinaryCodecMock {
    fn write_array<T: Copy, const N: usize>(&mut self, src: &[T; N]) {
        self.buffer.write_array::<T, N>(src)
    }

    fn write_slice<T: Copy>(&mut self, src: &[T]) {
        self.buffer.write_slice::<T>(src)
    }
}

pub fn encode<'a, T>(src: &T, dst: &mut [u8]) -> Result<(), <BinaryCodecMock as Encoder>::Error>
where
    T: Mesh<BinaryCodecMock, Output: SegmentWalker<T, BinaryCodecMock, SegmentEncoder>>,
{
    let buffer = Buffer::from(dst);
    let mut codec = BinaryCodecMock { buffer };
    walk_segment(src, &mut codec)
}

impl Encoder for BinaryCodecMock
where
    Self: BufWrite,
{
    type Error = EncodeError;

    type TupleEncoder = Self;

    type StructEncoder = Self;

    type SequenceEncoder = Self;

    fn encode_u8(&mut self, v: &u8) -> Result<(), Self::Error> {
        self.write_array(&[*v]);
        Ok(())
    }

    fn encode_i8(&mut self, v: &i8) -> Result<(), Self::Error> {
        self.write_array(&[*v]);
        Ok(())
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

    fn encode_tuple<'a>(&mut self) -> Result<&mut Self::TupleEncoder, Self::Error> {
        todo!()
    }

    fn encode_struct<'a>(&mut self) -> Result<&mut Self::StructEncoder, Self::Error> {
        Ok(self)
    }

    fn encode_seq(&mut self, len: usize) -> Result<&mut Self::SequenceEncoder, Self::Error> {
        self.encode_u8(&(len as u8))?;
        Ok(self)
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
        Ok(())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        //TODO remained buffer space check
        self.write_slice(v);
        Ok(())
    }

    fn encode_var_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        todo!()
    }
}

impl CompositeEncoder for BinaryCodecMock
where
    Self: BufWrite,
{
    type Error = EncodeError;

    fn encode_element<E: crate::Encode>(&mut self, v: &E) -> Result<(), Self::Error> {
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

impl Decoder for BinaryCodecMock {
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
    ) -> Result<crate::EnumIdentifier, Self::Error> {
        todo!()
    }

    fn decode_is_some(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl CompositeDecoder for BinaryCodecMock {
    type Error = DecodeError;

    fn decode_element<D: crate::Decode>(
        &mut self,
        place: &mut MaybeUninit<D>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl BufRead for BinaryCodecMock {
    fn read_slice<const N: usize>(&mut self, out: &mut MaybeUninit<[u8; N]>) {
        self.buffer.read_slice::<N>(out)
    }
}

#[derive(Debug)]
pub enum DecodeError {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    InvalidEnumVariantName,
    InvalidEnumVarirantIndex,
    Custom,
    InvalidUtf8,
    NonMaxButMax,
    NonZeroButZero,
}

impl crate::DecodeError for DecodeError {
    fn not_enough_bytes_in_the_buffer() -> Self {
        todo!()
    }

    fn too_large() -> Self {
        todo!()
    }

    fn invalid_enum_variant_name() -> Self {
        todo!()
    }

    fn invalid_enum_variant_index() -> Self {
        todo!()
    }

    fn custom() -> Self {
        todo!()
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
pub mod model {
    use std::hint::black_box;

    use test::Bencher;

    use crate::mock::encode;

    pub mod log {
        use std::{hint::black_box, str::FromStr};

        use test::Bencher;

        use crate::mock::encode;

        #[derive(
            serialization::Serializable,
            Debug,
            PartialEq,
            PartialOrd,
            Ord,
            Eq,
            Clone,
            bitcode::Encode,
        )]
        pub struct Log {
            pub address: Address,

            pub identity: String,
            pub userid: String,
            pub date: String,
            pub request: String,
            pub code: u16,
            pub size: u64,
        }

        #[repr(C)]
        #[derive(
            serialization::Serializable,
            Debug,
            PartialEq,
            PartialOrd,
            Ord,
            Eq,
            Clone,
            bitcode::Encode,
        )]
        pub struct Logs {
            pub logs: Vec<Log>,
        }

        #[repr(C)]
        #[derive(
            serialization::Serializable,
            Debug,
            PartialEq,
            PartialOrd,
            Ord,
            Eq,
            Clone,
            bitcode::Encode,
        )]
        pub struct Address {
            pub x0: u8,
            pub x1: u8,
            pub x2: u8,
            pub x3: u8,
        }

        impl Default for Logs {
            fn default() -> Self {
                Self {
                    logs: vec![
                        Log {
                            address: Address {
                                x0: 11,
                                x1: 22,
                                x2: 33,
                                x3: 44,
                            },

                            identity: String::from_str("abcd").unwrap(),
                            userid: String::from_str("a").unwrap(),
                            date: String::from_str("wijkl").unwrap(),
                            request: String::from_str("mnop").unwrap(),
                            code: 55,
                            size: 66,
                        };
                        1
                    ],
                }
            }
        }

        #[ignore]
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

        #[ignore]
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

    pub mod foo {
        use std::hint::black_box;

        use test::Bencher;

        use crate::mock::encode;

        #[repr(C)]
        #[derive(serialization::Serializable)]
        pub struct Foo {
            field0: u8, // offset 0 size 1
            // padding 3
            field1: Bar, // offset 4 size 12
            // padding 0
            field2: Vec<u8>, // offset 16 size 24
            // padding 0
            field3: u32, // offset 40 size 4
            // padding 0
            field4: Baz, // offset 44 size 2
            // padding 2
            field5: u32, // offset 48 size 4
                         // padding 4
                         // model size 56
        }

        #[repr(C)]
        #[derive(serialization::Serializable)]
        pub struct Bar {
            field0: u32, // offset 0  size 4
            // padding 0
            field1: u32, // offset 4 size 4
            // padding 0
            field2: Baz, //offset 8 size 2
                         // padding 2
                         // size 12
        }

        #[repr(C)]
        #[derive(serialization::Serializable)]
        pub struct Baz {
            field0: u8, // offset 0 size 1
            // padding 0
            field1: u8, // offset 0 size 1
                        // padding 0
        }

        impl Default for Foo {
            fn default() -> Self {
                Foo {
                    field0: 11,
                    field1: Bar {
                        field0: 22,
                        field1: 33,
                        field2: Baz {
                            field0: 44,
                            field1: 55,
                        },
                    },
                    field2: vec![1, 2, 3, 4],
                    field3: 66,
                    field4: Baz {
                        field0: 77,
                        field1: 88,
                    },
                    field5: 99,
                }
            }
        }

        #[test]
        fn test_mock_model_encode() {
            #[allow(invalid_value)]
            let mut dst = [0_u8; 1000000];
            println!("--------");
            encode(&Foo::default(), &mut dst).unwrap();
            println!("{:?}", &dst[..66]);
            black_box(&dst);
            println!("--------");
        }

        #[ignore]
        #[bench]
        fn bench_mock_model(b: &mut Bencher) {
            let model = &Foo::default();
            let mut dst = [0_u8; 1000000];
            b.iter(|| encode(model, &mut dst));
            black_box(&dst);
        }
    }
}
