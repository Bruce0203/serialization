#![feature(specialization)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

use std::{hint::black_box, str::FromStr};

use bitcode::Buffer;
use fastbuf::ByteBuffer;
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Debug, Serializable)]
pub struct Foo {
    value: Vec<Bar>,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar Dropped");
    }
}

#[derive(Debug, Serializable)]
pub struct Bar {
    value: String,
}
#[test]
fn asdf() {
    let mut buf = ByteBuffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let model = Foo {
        value: vec![
            Bar {
                value: String::from_str("hello").unwrap(),
            },
            Bar {
                value: unsafe { String::from_utf8_unchecked([0x80, 0xC0].to_vec()) },
            },
            Bar {
                value: String::from_str("hi").unwrap(),
            },
        ],
    };
    model.encode(&mut enc).unwrap();
    std::mem::forget(model);
    println!("{:?}", buf);
    let mut dec = PacketDecoder::new(&mut buf);
    let value = Foo::decode(&mut dec).unwrap_err();
    black_box(&value);
}
