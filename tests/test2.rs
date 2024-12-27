#![feature(specialization)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

use fastbuf::{Buffer, ByteBuffer};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Serializable, Debug)]
#[repr(C)]
pub struct A {
    value: u32,
    value2: Foo,
    value3: u8,
}

#[derive(Serializable, Debug)]
#[repr(C)]
pub struct Foo {
    value: Vec<u8>,
}

#[test]
fn asdf() {
    let a = A {
        value: 11,
        value2: Foo {
            value: vec![1, 2, 3],
        },
        value3: 33,
    };
    let mut buf = ByteBuffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    a.encode(&mut enc).unwrap();
    println!(
        "{:?}",
        <A as serialization::__private::SerialDescriptor>::serial_sizes::<
            PacketEncoder<&mut Buffer<u8, 10>>,
        >()
    );
    println!("{buf:?}");

    let mut dec = PacketDecoder::new(&mut buf);

    let a = A::decode(&mut dec).unwrap();
    println!("{:?}", a);
}
