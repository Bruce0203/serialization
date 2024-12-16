#![feature(generic_arg_infer)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::mem::MaybeUninit;

use constvec::ConstVec;
use fastbuf::{Buffer, WriteBuf};
use serialization::{binary_format::DecodeField, Decoder, Serializable};
use serialization_minecraft::PacketDecoder;

#[derive(Serializable)]
pub struct AA {
    value: Vec<u8>,
}

#[test]
fn test11() {
    let mut fields = ConstVec::new(1, [0; _]);
    let mut field = unsafe { MaybeUninit::uninit().assume_init() };
    let mut buf = Buffer::<1000>::new();
    buf.write(&[3, 1, 2, 3]);
    let mut decoder = PacketDecoder::new(&mut buf);
    let mut tup = decoder.decode_tuple().unwrap();
    unsafe { AA::decode_field(&mut fields, &mut field, &mut tup) }.unwrap();
}
