#![feature(derive_eq)]
#![feature(coverage_attribute)]
#![feature(structural_match)]
#![feature(panic_internals)]
#![feature(rustc_attrs)]
#![feature(fmt_helpers_for_derive)]
#![feature(prelude_import)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

use fastbuf::Buffer;
use serialization::binary_format::const_transmute;
use serialization_minecraft::PacketEncoder;

#[repr(C)]
pub struct Log {
    size: u32,
    v: Vec<u8>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Log {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "Log", "size", &self.size, "v", &&self.v,
        )
    }
}
impl<'de> const serialization::binary_format::SerialDescriptor for Log {
    const N: usize = <u32 as serialization::binary_format::SerialDescriptor>::N
        + <Vec<u8> as serialization::binary_format::SerialDescriptor>::N
        + 2usize
        + 1;
    fn fields<_C: const serialization::CheckPrimitiveTypeSize>(
    ) -> serialization::constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields(
            {
                #[allow(invalid_value)]
                let value: std::mem::MaybeUninit<Log> = std::mem::MaybeUninit::zeroed();
                let value = unsafe { value.assume_init_ref() };
                let mut padding_calc = serialization::binary_format::SizeCalcState::new(value);
                serialization::binary_format::SizeCalcState::next_field::<_, _C, 0_u16>(
                    &mut padding_calc,
                    &value.size,
                );
                serialization::binary_format::SizeCalcState::next_field::<_, _C, 1_u16>(
                    &mut padding_calc,
                    &value.v,
                );
                serialization::binary_format::SizeCalcState::finish(padding_calc)
            },
            serialization::binary_format::SerialSize::unsized_field_of(),
        )
    }
}
impl<'de> serialization::Encode for Log {
    fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Log, _E>() } {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.size)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.v)?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::EncodeField for Log {
    fn encode_field<_E: serialization::Encoder>(
        &self,
        fields: &serialization::binary_format::Fields,
        encoder: _E,
    ) -> Result<(), _E::Error> {
        if fields.len() == 0 {
            serialization::Encode::encode(&self, encoder)
        } else {
            let mut fields = fields.clone();
            match *fields.pop_last() as usize {
                0usize => self.size.encode_field(&fields, encoder),
                1usize => self.v.encode_field(&fields, encoder),
                _ =>
                //    println!("{:?}", decoded);
                //   assert_eq!(decoded, value);
                {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }
    }
}
impl<'de> serialization::Decode<'de> for Log {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Log, _D>() } {
                let mut struc = decoder.decode_struct()?;
                let result = Log {
                    size: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    v: serialization::CompositeDecoder::decode_element(&mut struc)?,
                };
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::DecodeField<'de> for Log {
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut _D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, _D::Error> {
        #[allow(invalid_value)]
        let result: std::mem::MaybeUninit<Log> = std::mem::MaybeUninit::zeroed();
        let result = unsafe { result.assume_init_ref() };
        let mut state = serialization::binary_format::DecodeFieldState::new(result, fields.clone());
        match state.start::<_D>() {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index as usize {
                0usize => state.decode_field(decoder, &result.size)?,
                1usize => state.decode_field(decoder, &result.v)?,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Log {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Log {
    #[inline]
    fn eq(&self, other: &Log) -> bool {
        self.size == other.size && self.v == other.v
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Log {
    #[inline]
    fn partial_cmp(&self, other: &Log) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.size, &other.size) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                ::core::cmp::PartialOrd::partial_cmp(&self.v, &other.v)
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Log {
    #[inline]
    fn cmp(&self, other: &Log) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.size, &other.size) {
            ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.v, &other.v),
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Log {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
    }
}
pub struct Logs {
    pub logs: Vec<Log>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Logs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Logs", "logs", &&self.logs)
    }
}
impl<'de> const serialization::binary_format::SerialDescriptor for Logs {
    const N: usize = <Vec<Log> as serialization::binary_format::SerialDescriptor>::N + 1usize + 1;
    fn fields<_C: const serialization::CheckPrimitiveTypeSize>(
    ) -> serialization::constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields(
            {
                #[allow(invalid_value)]
                let value: std::mem::MaybeUninit<Logs> = std::mem::MaybeUninit::zeroed();
                let value = unsafe { value.assume_init_ref() };
                let mut padding_calc = serialization::binary_format::SizeCalcState::new(value);
                serialization::binary_format::SizeCalcState::next_field::<_, _C, 0_u16>(
                    &mut padding_calc,
                    &value.logs,
                );
                serialization::binary_format::SizeCalcState::finish(padding_calc)
            },
            serialization::binary_format::SerialSize::unsized_field_of(),
        )
    }
}
impl<'de> serialization::Encode for Logs {
    fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Logs, _E>() } {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.logs)?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::EncodeField for Logs {
    fn encode_field<_E: serialization::Encoder>(
        &self,
        fields: &serialization::binary_format::Fields,
        encoder: _E,
    ) -> Result<(), _E::Error> {
        if fields.len() == 0 {
            serialization::Encode::encode(&self, encoder)
        } else {
            let mut fields = fields.clone();
            match *fields.pop_last() as usize {
                0usize => self.logs.encode_field(&fields, encoder),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
}
impl<'de> serialization::Decode<'de> for Logs {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Logs, _D>() } {
                let mut struc = decoder.decode_struct()?;
                let result = Logs {
                    logs: serialization::CompositeDecoder::decode_element(&mut struc)?,
                };
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::DecodeField<'de> for Logs {
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut _D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, _D::Error> {
        #[allow(invalid_value)]
        let result: std::mem::MaybeUninit<Logs> = std::mem::MaybeUninit::zeroed();
        let result = unsafe { result.assume_init_ref() };
        let mut state = serialization::binary_format::DecodeFieldState::new(result, fields.clone());
        match state.start::<_D>() {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index as usize {
                0usize => state.decode_field(decoder, &result.logs)?,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Logs {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Logs {
    #[inline]
    fn eq(&self, other: &Logs) -> bool {
        self.logs == other.logs
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Logs {
    #[inline]
    fn partial_cmp(&self, other: &Logs) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.logs, &other.logs)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Logs {
    #[inline]
    fn cmp(&self, other: &Logs) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.logs, &other.logs)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Logs {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<Log>>;
    }
}
#[test]
fn test_log() {
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
    //   assert_eq!(decoded, value);
    println!("HIAAI");

}
