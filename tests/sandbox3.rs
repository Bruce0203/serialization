#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use fastbuf::{Buffer, ReadBuf};
use serialization::binary_format::SerialDescriptor;
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestA {
    value4: Vec<u8>,
}

fn testA() {
    println!(
        "{:?}",
        TestA::fields::<&mut PacketDecoder<&mut Buffer<1>>>()
    );
    type T = TestA;
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let value: T = TestA {
        value4: vec![1, 2, 3],
    };
    serialization::Encode::encode(&value, &mut enc).unwrap();
    println!("{:?}", &buf);
    println!("{:?}", buf.remaining());
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <T as serialization::Decode>::decode(&mut dec).unwrap();
    println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HIAAI");
}
