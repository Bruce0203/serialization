#![feature(generic_arg_infer)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{hint::black_box, str::FromStr};

use divan::{bench, Bencher};
use fastbuf::{Buf, Buffer};
use rkyv::rancor;
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

const SAMPLE_COUNT: u32 = 1000;
const SAMPLE_SIZE: u32 = 1000;

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
    rkyv::Serialize,
    rkyv::Deserialize,
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
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
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
    pub logs: Log,
}

#[derive(
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
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
        logs: Log {
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
        },
    }
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn encode_serialization(bencher: Bencher) {
    let mut buf = Buffer::<1000000>::new();
    let model = &model();
    bencher.bench_local(|| {
        let mut enc = PacketEncoder::new(&mut buf);
        black_box(&model.encode(&mut enc).unwrap());
        unsafe { buf.set_filled_pos(0) };
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn decode_serialization(bencher: Bencher) {
    let mut buf = Buffer::<1000000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let model = &model();
    black_box(model.encode(&mut enc)).unwrap();
    bencher.bench_local(|| {
        let mut dec = PacketDecoder::new(&mut buf);
        black_box(&Model::decode_placed(&mut dec).unwrap());
        unsafe { buf.set_pos(0) };
    });
}
#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn encode_bitcode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = &model();
    bencher.bench_local(|| {
        black_box(&buf.encode(model));
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn decode_bitcode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = model();
    let bytes = bitcode::encode(&model);
    let bytes = bytes.as_slice();
    bencher.bench_local(|| {
        black_box(&buf.decode::<Model>(bytes).unwrap());
    });
}

fn main() {
    divan::main();
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
fn encode_rkyv(bencher: Bencher) {
    {
        println!(
            "{:?}",
            rkyv::to_bytes::<rkyv::rancor::Error>(&123_u32).unwrap()
        )
    }
    let model = &model();
    bencher.bench_local(|| {
        black_box(&rkyv::to_bytes::<rancor::Error>(black_box(model)).unwrap());
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn decode_rkyv(bencher: Bencher) {
    let bytes = black_box(rkyv::to_bytes::<rancor::Error>(black_box(&model())).unwrap());
    let bytes = bytes.as_slice();
    bencher.bench_local(|| {
        black_box(&rkyv::from_bytes::<Model, rancor::Error>(black_box(bytes)));
    });
}
