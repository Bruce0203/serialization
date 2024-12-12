#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::hint::black_box;

use divan::{bench, Bencher};
use fastbuf::{Buf, Buffer};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Serializable, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Foo {
    v1: Bar,
    v2: u8,
}

#[derive(Serializable, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bar {
    v1: i32,
}

pub const MODEL: Foo = Foo {
    v1: Bar { v1: 123 },
    v2: 234,
};

#[bench(sample_count = 1000, sample_size = 1000)]
fn encode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    bencher.bench_local(|| {
        let mut enc = PacketEncoder::new(&mut buf);
        black_box(MODEL.encode(&mut enc)).unwrap();
        unsafe { buf.set_filled_pos(0) };
    });
}

#[bench(sample_count = 1000, sample_size = 1000)]
fn decode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    black_box(MODEL.encode(&mut enc)).unwrap();
    bencher.bench_local(|| {
        {
            let mut dec = PacketDecoder::new(&mut buf);
            let decoded_bar = black_box(Foo::decode(&mut dec)).unwrap();
            unsafe { buf.set_pos(0) };
            black_box(&decoded_bar);
            // assert_eq!(decoded_bar, bar);
        }
    });
}

fn main() {
    divan::main();
}
