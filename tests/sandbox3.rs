#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::str::FromStr;

use fastbuf::{Buffer, ReadBuf};
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestA {
    value4: Vec<u8>,
    value3: (u16, u16),
    value6: String,
    value5: Vec<AAAAAA>,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
struct AAAAAA {
    value: u32,
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
        value3: (11, 22),
        value4: vec![1, 2, 3],
        value5: vec![AAAAAA { value: 123 }],
        value6: String::from_str("123132").unwrap(),
    };
    println!("value ={:?}", unsafe {
        const_transmute::<_, &[u8; size_of::<T>()]>(&value)
    });
    println!("len={}", size_of::<TestA>());
    serialization::Encode::encode(&value, &mut enc).unwrap();
    println!("{:?}", &buf);
    println!("{:?}", buf.remaining());
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <T as serialization::Decode>::decode(&mut dec).unwrap();
    println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HIAAI");
}
