use std::{hint::black_box, str::FromStr};

use divan::{bench, Bencher};
use serialization::mock::encode;

const SAMPLE: u32 = 100;
const SAMPLE_COUNT: u32 = SAMPLE;
const SAMPLE_SIZE: u32 = SAMPLE;

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bench_log_model_with_1_serialization(b: Bencher) {
    let model = &model();
    black_box(&model);
    let ref mut dst = [0_u8; 5000000];
    b.bench_local(|| {
        black_box(&encode(model, dst));
    });
}

#[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
fn bench_log_model_with_2_bitcode(b: Bencher) {
    let mut buf = bitcode::Buffer::default();
    let model = &model();
    black_box(&model);
    b.bench_local(|| {
        black_box(&buf.encode(model));
    });
    black_box(&buf);
}
fn main() {
    divan::main();
}

#[derive(
    serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode,
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
    serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode,
)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[repr(C)]
#[derive(
    serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, bitcode::Encode,
)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

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
                userid: String::from_str("a").unwrap(),
                date: String::from_str("wijkl").unwrap(),
                request: String::from_str("mnop").unwrap(),
                code: 55,
                size: 66,
            };
            10000
        ],
    }
}
