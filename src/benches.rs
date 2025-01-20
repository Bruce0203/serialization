use std::{
    hint::black_box,
    mem::{MaybeUninit, transmute},
    str::FromStr,
};

use rand::Rng;
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
            100
        ],
    }
}

fn random_string<const N: usize>() -> String {
    String::from_str(core::str::from_utf8(&[rand::thread_rng().gen_range(0..10_u8); N]).unwrap())
        .unwrap()
}

fn models() -> Vec<Logs> {
    let mut dst = Vec::new();
    for _ in 0..1000_000 {
        dst.push(model());
    }
    black_box(&model);
    println!("model generated");
    dst
}

#[ignore]
#[bench]
fn bench_log_models(b: &mut Bencher) {
    let models = models();
    let mut models = models.iter();
    let mut dst: Box<[u8]> = Box::new([0_u8; 1000000]);
    black_box(&model);
    b.iter(|| {
        black_box(encode(models.next().unwrap(), &mut dst).unwrap());
    });
    black_box(&dst);
}

#[ignore]
#[bench]
fn bench_log_models_with_bitcode(b: &mut Bencher) {
    let models = models();
    let mut models = models.iter();
    let mut buf = bitcode::Buffer::default();
    b.iter(|| {
        black_box(&buf.encode(models.next().unwrap()));
    });
}

#[bench]
fn bench_log_model(b: &mut Bencher) {
    let models = model();
    let mut dst: Box<[u8]> = Box::new([0_u8; 1000000]);
    black_box(&model);
    b.iter(|| {
        black_box(encode(&models, &mut dst).unwrap());
    });
    println!("{:?}", &dst[0..1000]);
    black_box(&dst);
}

#[bench]
fn bench_log_model_with_bitcode(b: &mut Bencher) {
    let models = model();
    let mut buf = bitcode::Buffer::default();
    b.iter(|| {
        black_box(&buf.encode(&models));
    });
    black_box(&buf);
}
