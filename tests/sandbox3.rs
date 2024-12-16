#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::str::FromStr;

use fastbuf::{Buffer, ReadBuf};
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[repr(C)]
#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestA {
    value5: Vec<Foo>,
    value2: String,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
struct Foo {
    value: u32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo dropped!!");
    }
}

#[test]
fn testA() {
    println!(
        "{:?}",
        TestA::fields::<&mut PacketDecoder<&mut Buffer<1>>>()
    );
    type T = TestA;
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let value: T = TestA {
        value5: vec![Foo { value: 123 }],
        value2: String::from_str("ABCD").unwrap(),
    };
    println!("value ={:?}", unsafe {
        const_transmute::<_, &[u8; size_of::<T>()]>(&value)
    });
    println!("len={}", size_of::<TestA>());
    println!("ptr ={:#08x}", unsafe {
        const_transmute::<_, &usize>(&value.value5.as_ptr())
    });
    serialization::Encode::encode(&value, &mut enc).unwrap();
    println!("{:?}", &buf);
    println!("{:?}", buf.remaining());
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <T as serialization::Decode>::decode(&mut dec).unwrap();
    println!("decoded ptr ={:?}", unsafe {
        const_transmute::<_, &[u8; size_of::<Vec<u32>>()]>(&decoded.value5)
    });
    println!("{:?}", &decoded.value5);
    println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HI");
    drop((decoded, value));
}
