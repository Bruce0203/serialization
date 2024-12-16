#![feature(new_zeroed_alloc)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::str::FromStr;

use fastbuf::Buffer;
use serialization_minecraft::PacketEncoder;

#[repr(C)]
#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestA {
    value: Vec<u32>,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}

#[test]
fn testA() {
    type T = TestA;
    const BUFFER_LEN: usize = 1024 * 1024 * 5000;
    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
    let mut enc = PacketEncoder::new(&mut buf);
    let value: T = TestA { value: vec![123] };
    serialization::Encode::encode(&value, &mut enc).unwrap();

    println!("{:?}", &buf);
    // println!("{:?}", buf.remaining());
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <T as serialization::Decode>::decode_placed(&mut dec).unwrap();
    // println!("{:?}", &decoded.value5);
    // println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HI");
    drop((decoded, value));
}
