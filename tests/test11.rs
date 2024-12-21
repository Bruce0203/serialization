#![feature(generic_arg_infer)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::mem::MaybeUninit;

use constvec::ConstVec;
use fastbuf::{Buffer, WriteBuf};
use serialization::{
    binary_format::{Field, Fields, OffsetAccumlator},
    Decoder, Serializable,
};
use serialization_minecraft::PacketDecoder;

#[derive(Serializable)]
pub struct AA {
    value: Vec<u8>,
}

#[test]
fn test11() {
    let mut fields = Fields::EMPTY;
    fields.push(&0);
    let mut buf = Buffer::<1000>::new();
    buf.write(&[3, 1, 2, 3]);
    let mut decoder = PacketDecoder::new(&mut buf);
    let mut tup = decoder.decode_tuple().unwrap();
    let mut acc = 0;
    unsafe { AA::acc_offset(&fields, fields.len() as Field - 1) };
}
