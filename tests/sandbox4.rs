#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{fs::File, io::Read, str::FromStr};

use fastbuf::{Buf, Buffer, ReadToBuf};
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::PacketEncoder;

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
#[repr(C)]

pub struct Log {
    pub address: Address,
    pub identity: String,
    pub userid: String,
    pub date: String,
    pub request: String,
    pub code: u16,
    pub size: u64,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}
#[test]
fn test_log() {
    println!(
        "{:?}",
        Log::fields::<&mut PacketEncoder<&mut Buffer<100>>>()
    );
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let value = Logs {
        logs: vec![Log {
            address: Address {
                x0: 11,
                x1: 22,
                x2: 33,
                x3: 44,
            },
            identity: String::from_str("ABCD").unwrap(),
            userid: String::from_str("ABCD").unwrap(),
            date: String::from_str("ABCD").unwrap(),
            request: String::from_str("ABCD").unwrap(),
            code: 55,
            size: 66,
        }],
    };
    let mut file = File::open("testtemp").unwrap();
    file.read_to_buf(&mut buf).unwrap();

    // serialization::Encode::encode(&value, &mut enc).unwrap();
    for i in 0..1000 {
        println!("{i}");
        let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
        let decoded = <Logs as serialization::Decode>::decode(&mut dec).unwrap();
        unsafe { buf.set_pos(0) };
        assert_eq!(decoded, value);
    }
    //    println!("{:?}", decoded);
    println!("HIAAI");
}
