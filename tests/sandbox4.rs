#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use fastbuf::Buffer;
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::PacketEncoder;

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
#[repr(C)]
pub struct Log {
    v: Vec<u8>,
    size: u64,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct Logs {
    pub logs: Vec<Log>,
}

// #[test]
fn test_log() {
    println!(
        "{:?}",
        Log::fields::<&mut PacketEncoder<&mut Buffer<100>>>()
    );
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let value = Logs {
        logs: vec![Log {
            size: 11,
            v: vec![1, 2, 3],
        }],
    };
    serialization::Encode::encode(&value, &mut enc).unwrap();
    println!("{:?}", &buf);
    let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
    let decoded = <Logs as serialization::Decode>::decode(&mut dec).unwrap();
    println!("{:?}", unsafe {
        const_transmute::<_, &[u8; size_of::<Logs>()]>(&decoded)
    });
    //    println!("{:?}", decoded);
    assert_eq!(decoded, value);
    println!("HIAAI");
}
