use std::{hint::black_box, str::FromStr};

use test::Bencher;

use crate::mock::encode;

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
            1
        ],
    }
}

#[bench]
fn bench_log_model(b: &mut Bencher) {
    let model = model();
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
    let model = model();
    black_box(&model);
    let mut buf = bitcode::Buffer::default();
    b.iter(|| {
        black_box(&buf.encode(&model));
    });
    black_box(&buf);
}
