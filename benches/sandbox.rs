#![feature(specialization)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

use std::{hint::black_box, str::FromStr};

use divan::{bench, Bencher};
use fastbuf::{Buffer, ReadBuf, WriteBuf};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

const SAMPLE_COUNT: u32 = 1000;
const SAMPLE_SIZE: u32 = 1000;

fn main() {
    divan::main();
}

#[derive(
    Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode, bitcode::Decode,
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
    Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode, bitcode::Decode,
)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(
    Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode, bitcode::Decode,
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

                identity: String::from_str("abcd").unwrap(),
                userid: String::from_str("efgh").unwrap(),
                date: String::from_str("ijkl").unwrap(),
                request: String::from_str("abcd").unwrap(),
                code: 55,
                size: 66,
            };
            10
        ],
    }
}

#[bench(sample_size = 1000, sample_count = 1000)]
fn bench_encode(bencher: Bencher) {
    let mut buf = Buffer::<[u8; 10000]>::new();
    let ref model = model();
    bencher.bench_local(|| {
        let mut enc = PacketEncoder::new(&mut buf);
        let _ = black_box(black_box(model).encode(&mut enc).unwrap());
        unsafe { buf.set_filled_pos(0) };
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bench_decode(bencher: Bencher) {
    let mut buf = Buffer::<[u8; 10000]>::new();
    let ref model = model();
    let mut enc = PacketEncoder::new(&mut buf);
    let _ = black_box(model.encode(&mut enc));
    bencher.bench_local(|| {
        let mut dec = PacketDecoder::new(&mut buf);
        black_box(Model::decode(&mut dec).unwrap());
        unsafe { buf.set_pos(0) };
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bench_encode_bitcode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = &model();
    bencher.bench_local(|| {
        black_box(&buf.encode(black_box(model)));
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bench_decode_bitcode(bencher: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = model();
    let bytes = bitcode::encode(&model);
    let bytes = bytes.as_slice();
    bencher.bench_local(|| {
        black_box(&buf.decode::<Model>(bytes).unwrap());
    });
}
