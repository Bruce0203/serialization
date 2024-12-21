#![feature(generic_arg_infer)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{hint::black_box, marker::PhantomData, mem::MaybeUninit, str::FromStr};

use divan::{bench, Bencher};
use fastbuf::{Buf, Buffer, ReadBuf, WriteBuf};
use fastvarint::{EncodeVarInt, VarInt};
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

const SAMPLE_COUNT: u32 = 2000;
const SAMPLE_SIZE: u32 = 2000;

#[derive(
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
    bitcode::Decode,
    bitcode::Encode,
    rkyv::Deserialize,
    rkyv::Serialize,
    rkyv::Archive,
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

#[derive(
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    bitcode::Decode,
    bitcode::Encode,
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
    Copy,
    bitcode::Decode,
    bitcode::Encode,
)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

type Model = Logs;
fn model() -> Logs {
    Logs {
        logs: vec![
            Log {
                address: Address {
                    x0: 11,
                    x1: 22,
                    x2: 33,
                    x3: 44,
                },
                identity: String::from_str("asdf").unwrap(),
                userid: String::from_str("asdf").unwrap(),
                date: String::from_str("asdf").unwrap(),
                request: String::from_str("asdf").unwrap(),
                code: 55,
                size: 66,
            };
            10
        ],
    }
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn encode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let model = &model();
    bencher.bench_local(|| {
        let mut enc = PacketEncoder::new(&mut buf);
        black_box(&model.encode(&mut enc).unwrap());
        unsafe { buf.set_filled_pos(0) };
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn decode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let model = &model();
    black_box(model.encode(&mut enc)).unwrap();
    bencher.bench_local(|| {
        let mut dec = PacketDecoder::new(&mut buf);
        black_box(&Model::decode_placed(&mut dec).unwrap());
        unsafe { buf.set_pos(0) };
    });
}

fn main() {
    divan::main();
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bitcode_encode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = &model();
    bencher.bench_local(|| {
        black_box(&buf.encode(model));
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bitcode_decode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = model();
    let bytes = bitcode::encode(&model);
    let bytes = bytes.as_slice();
    bencher.bench_local(|| {
        black_box(&buf.decode::<Model>(bytes).unwrap());
    });
}

#[derive(serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[repr(C)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct AA {
    value2: Vec<A2>,
}

#[derive(serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[repr(C)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct A2 {
    value: u8,
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn a_test11(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let result = AA {
        value2: vec![A2 { value: 123 }],
    }
    .encode(&mut enc);
    result.unwrap();
    bencher.bench_local(|| {
        unsafe { buf.set_pos(0) };
        let mut dec = PacketDecoder::new(&mut buf);
        let result = AA::decode_placed(&mut dec);
        result.unwrap();
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn asdfwqer(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    bencher.bench_local(|| {
        unsafe { buf.set_filled_pos(0) };
        let mut enc = PacketEncoder::new(&mut buf);
        let result = AA {
            value2: vec![A2 { value: 123 }],
        }
        .encode(&mut enc);
        result.unwrap();
    });
}
