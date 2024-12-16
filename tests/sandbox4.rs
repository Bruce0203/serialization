#![feature(new_zeroed_alloc)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{
    fmt,
    fs::File,
    hint::black_box,
    io::{Read, Write},
    mem::forget,
    str::FromStr,
};

use fastbuf::{Buf, Buffer, ReadBuf, WriteBuf};
use serialization::binary_format::{const_transmute, SerialDescriptor};
use serialization_minecraft::PacketEncoder;

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Log {
    pub address: Address,
    pub identity: String,
    pub userid: String,
    pub date: String,
    pub request: String,
    pub code: u16,
    pub size: u64,
}

// impl Drop for Log {
//     fn drop(&mut self) {
//         println!("log dropped");
//     }
// }

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(Debug, serialization::Serializable, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

#[test]
fn testing() {
    for i in 0..10 {
        test_log();
    }
}

fn test_log() {
    const BUFFER_LEN: usize = 1024 * 1024 * 5000;
    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
    let value = Logs {
        logs: vec![
            Log {
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
            };
            10
        ],
    };
    let mut enc = PacketEncoder::new(&mut buf);
    serialization::Encode::encode(&value, &mut enc).unwrap();
    // let mut file = File::open("testtemp").unwrap();
    // let mut vec = Vec::new();
    // file.read_to_end(&mut vec).unwrap();
    // println!("veclen={:?}", vec.len());
    // WriteBuf::write(&mut buf, vec.as_slice());
    let buf_clone = buf.get_continuous(buf.remaining()).clone();
    for i in 0..10 {
        let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
        WriteBuf::write(&mut buf, &buf_clone);
        let mut dec = serialization_minecraft::PacketDecoder::new(&mut buf);
        let decoded = <Logs as serialization::Decode>::decode(&mut dec);
        let decoded = decoded.unwrap();
        unsafe { buf.set_pos(0) };

        black_box(&decoded);
    }
    //    println!("{:?}", decoded);
    drop(buf);
    std::mem::forget(value);
}
