use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::{hint::black_box, mem::transmute, str::FromStr};

use serialization::mock::encode;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[library_benchmark]
#[bench::one(&model())]
fn bench(model: &Logs) -> [u8; 1000000] {
    let mut dst = [0_u8; 1000000];
    black_box(encode(model, &mut dst).unwrap());
    dst
}

library_benchmark_group!(name = bench_group; benchmarks = bench);
main!(library_benchmark_groups = bench_group);

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
            300
        ],
    }
}
