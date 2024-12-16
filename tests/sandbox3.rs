#![feature(new_zeroed_alloc)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::str::FromStr;

use fastbuf::{Buffer, ReadBuf};
use rand::Rng;
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[repr(C)]
#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestA {
    value5: Vec<Foo>,
    value2: String,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Foo {
    value2: String,
    value: u32,
}

impl Drop for Foo {
    fn drop(&mut self) {}
}

#[test]
fn testA() {
    type T = TestA;
    const BUFFER_LEN: usize = 1024 * 1024 * 5000;
    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
    let mut enc = PacketEncoder::new(&mut buf);
    let value: T = TestA {
        value5: vec![
            Foo {
                value: 123,
                value2: unsafe { String::from_str("bruce").unwrap() }
            };
            10000
        ],
        value2: String::from_str("ABCD").unwrap(),
    };
    serialization::Encode::encode(&value, &mut enc).unwrap();
    // println!("{:?}", &buf);
    // println!("{:?}", buf.remaining());
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <T as serialization::Decode>::decode(&mut dec).unwrap();
    // println!("{:?}", &decoded.value5);
    // println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HI");
    drop((decoded, value));
}
