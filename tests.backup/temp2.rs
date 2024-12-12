//test
#![feature(generic_arg_infer)]
//////
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{
    hint::black_box,
    mem::{transmute, MaybeUninit},
};

use constvec::ConstVec;
use fastbuf::Buffer;
use serialization::{
    binary_format::{
        compact_fields, const_transmute, decode2, encode2, DecodeField, DecodeFieldState,
        EncodeField, Fields, ReadableField, SerialDescriptor, SerialSize, SizeCalcState,
    },
    CheckPrimitiveTypeSize, CompositeDecoder, CompositeEncoder, Decode, Decoder, Encode, Encoder,
};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Foo {
    field1: u32,
    field2: u16,
    field3: u8,
    field4: u32,
}

impl Encode for Foo {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        if Self::fields::<E>().as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = encoder.encode_struct()?;
            struc.encode_element(&self.field1)?;
            struc.encode_element(&self.field2)?;
            struc.encode_element(&self.field3)?;
            struc.encode_element(&self.field4)?;
            struc.end()?;
            return Ok(());
        }
        encode2(self, encoder)
    }
}

impl<'de> Decode<'de> for Foo {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        let fields = Self::fields::<D>();
        if fields.as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = decoder.decode_struct()?;
            let result = Self {
                field1: struc.decode_element()?,
                field2: struc.decode_element()?,
                field3: struc.decode_element()?,
                field4: struc.decode_element()?,
            };
            struc.end()?;
            return Ok(result);
        }
        decode2(decoder)
    }
}

impl EncodeField for Foo {
    fn encode_field<E: Encoder>(&self, fields: &Fields, encoder: E) -> Result<(), E::Error> {
        if fields.len() == 0 {
            self.encode(encoder)
        } else {
            let mut fields = fields.clone();
            match fields.pop_last() {
                0 => self.field1.encode_field(&fields, encoder),
                1 => self.field2.encode_field(&fields, encoder),
                2 => self.field3.encode_field(&fields, encoder),
                3 => self.field4.encode_field(&fields, encoder),
                _ => unreachable!(),
            }
        }
    }
}

impl<'de> DecodeField<'de> for Foo {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        fields: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error> {
        #[allow(invalid_value)]
        let result: Self = unsafe { MaybeUninit::uninit().assume_init() };
        let mut state = DecodeFieldState::new(&result, fields.clone());
        match state.start::<D>() {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index {
                0 => state.decode_field(decoder, &result.field1)?,
                1 => state.decode_field(decoder, &result.field2)?,
                2 => state.decode_field(decoder, &result.field3)?,
                3 => state.decode_field(decoder, &result.field4)?,
                _ => unreachable!(),
            }),
        }
    }
}

impl const SerialDescriptor for Foo {
    const N: usize = <u32 as SerialDescriptor>::N
        + <u16 as SerialDescriptor>::N
        + <u8 as SerialDescriptor>::N
        + <u32 as SerialDescriptor>::N
        + 4
        + 1;

    fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]> {
        compact_fields(
            {
                #[allow(invalid_value)]
                let value: Self = unsafe { MaybeUninit::uninit().assume_init() };
                let mut padding_calc = SizeCalcState::new(&value);
                padding_calc.next_field::<_, C, 0>(&value.field1);
                padding_calc.next_field::<_, C, 1>(&value.field2);
                padding_calc.next_field::<_, C, 2>(&value.field3);
                padding_calc.next_field::<_, C, 3>(&value.field4);
                padding_calc.finish()
            },
            constvec::ConstVec::new(Self::N, unsafe {
                serialization::binary_format::const_transmute(
                    [const {
                        serialization::binary_format::SerialSize::Sized {
                            start: 0,
                            len: size_of::<Self>(),
                        }
                    }; Self::N],
                )
            }),
        )
    }
}

#[test]
fn clone_test() {
    let arr = ConstVec::new(5, [1, 2, 3, 4, 5]);
    assert_eq!(arr.clone(), ConstVec::new(5, [1, 2, 3, 4, 5]));
    let arr = ConstVec::new(0, [0i32; 0]);
    assert_eq!(arr.clone(), ConstVec::new(0, []));
    let arr = ConstVec::new(1, [0i32; 1]);
    assert_eq!(arr.clone(), ConstVec::new(1, [0]));
}

#[test]
fn cutoff_front_at() {
    {
        let arr = ConstVec::new(5, [1, 2, 3, 4, 5]);
        assert!(arr.cutoff_front_at(1) == ConstVec::<[_; 5]>::new(4, [2, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(4) == ConstVec::<[_; 5]>::new(1, [5, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(5) == ConstVec::<[_; 5]>::new(0, [5, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(0) == ConstVec::<[_; 5]>::new(5, [1, 2, 3, 4, 5]));
        let arr = ConstVec::new(0, [0; 5]);
        assert_eq!(arr.cutoff_front_at(0), ConstVec::<[_; 5]>::new(0, [0; 5]));
        let arr = ConstVec::new(1, [0; 5]);
        assert_eq!(arr.cutoff_front_at(0), ConstVec::<[_; 5]>::new(1, [0; 5]));
        let arr = ConstVec::new(1, [0; 5]);
        assert_eq!(arr.cutoff_front_at(1), ConstVec::<[_; 5]>::new(0, [0; 5]));
        let arr = ConstVec::new(2, [0; 5]);
        assert_eq!(arr.cutoff_front_at(1), ConstVec::<[_; 5]>::new(1, [0; 5]));
        let arr = ConstVec::new(4, [0; 5]);
        assert_eq!(arr.cutoff_front_at(4), ConstVec::<[_; 5]>::new(0, [0; 5]));
    }
    assert_eq!([1, 2, 3, 4, 5].split_at(1).1, [2, 3, 4, 5]);
    assert_eq!([1, 2, 3, 4, 5].split_at(5).1, []);
    assert_eq!([0; 0].split_at(0).1, []);
    assert_eq!([0; 1].split_at(0).1, [0]);
}

#[test]
fn test() {
    let foo = Foo {
        field1: 12,
        field2: 23,
        field3: 34,
        field4: 45,
    };
    {
        let temp = unsafe { transmute::<_, &[u8; size_of::<Foo>()]>(&foo) };
        println!("foo = {:?}, len = {}", temp, temp.len());
    }
    let mut buf = Buffer::<1000>::new();
    {
        let slice: &mut [u8; 1000] = unsafe { const_transmute(buf.to_slice_mut()) };
        *slice = [0; _];
    }
    let ref mut encoder = PacketEncoder::new(&mut buf);
    black_box(foo.encode(encoder)).unwrap();
    println!("buf = {buf:?}");
    let ref mut decoder = PacketDecoder::new(&mut buf);
    let decoded = Foo::decode(decoder).unwrap();
    println!("dec = {buf:?}");
    println!("res = {:?}", unsafe {
        transmute::<_, &[u8; size_of::<Foo>()]>(&decoded)
    });
    assert_eq!(decoded, foo);
}

fn asdf2() {
    struct A(i32, i32);
    A { 0: 1, 1: 2 };
}
