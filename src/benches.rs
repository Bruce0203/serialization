use std::{hint::black_box, str::FromStr};

use test::Bencher;

use crate::{
    __private::{EncodeActor, Mesh, encode_with_encoder},
    mock::{self, Codec, encode},
};

#[derive(serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
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
#[derive(serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[repr(C)]
#[derive(serialization::Serializable, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
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
    let model = &model();
    let model = model.logs.first().unwrap();
    let mut dst = [0_u8; 1000];
    black_box(&model);
    b.iter(|| encode(model, &mut dst));
    println!("{:?}", &dst[..66]);
    black_box(&dst);
}
