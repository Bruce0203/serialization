use std::{
    hint::black_box,
    mem::{transmute, transmute_copy},
    str::FromStr,
};

use test::Bencher;

use crate::mock::encode;

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

#[bench]
fn bench_log_model(b: &mut Bencher) {
    let model = &model();
    let mut dst = [0_u8; 100000];
    black_box(&model);
    b.iter(|| encode(model, &mut dst));
    println!("{:?}", &dst[..66]);
    // assert_eq!(unsafe { transmute_copy::<_, &Model>(&dst) }, model);
    black_box(&dst);
}
