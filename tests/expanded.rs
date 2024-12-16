#![feature(derive_clone_copy)]
#![feature(derive_eq)]
#![feature(fmt_helpers_for_derive)]
#![feature(coverage_attribute)]
#![feature(core_intrinsics)]
#![feature(structural_match)]
#![feature(panic_internals)]
//////
#![feature(prelude_import)]
#![feature(new_zeroed_alloc)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

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

pub struct Log {
    pub address: Address,
    pub identity: String,
    pub userid: String,
    pub date: String,
    pub request: String,
    pub code: u16,
    pub size: u64,
}
#[automatically_derived]
impl ::core::fmt::Debug for Log {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "address", "identity", "userid", "date", "request", "code", "size",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.address,
            &self.identity,
            &self.userid,
            &self.date,
            &self.request,
            &self.code,
            &&self.size,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Log", names, values)
    }
}
impl<'de> const serialization::binary_format::SerialDescriptor for Log {
    const N: usize = <Address as serialization::binary_format::SerialDescriptor>::N
        + <String as serialization::binary_format::SerialDescriptor>::N
        + <String as serialization::binary_format::SerialDescriptor>::N
        + <String as serialization::binary_format::SerialDescriptor>::N
        + <String as serialization::binary_format::SerialDescriptor>::N
        + <u16 as serialization::binary_format::SerialDescriptor>::N
        + <u64 as serialization::binary_format::SerialDescriptor>::N
        + 7usize
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
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 0 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.address);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 1 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.identity);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 2 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.userid);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 3 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.date);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 4 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.request);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 5 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.code);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 6 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.size);
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
                serialization::CompositeEncoder::encode_element(&mut struc, &self.address)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.identity)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.userid)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.date)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.request)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.code)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.size)?;
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
                0usize => self.address.encode_field(&fields, encoder),
                1usize => self.identity.encode_field(&fields, encoder),
                2usize => self.userid.encode_field(&fields, encoder),
                3usize => self.date.encode_field(&fields, encoder),
                4usize => self.request.encode_field(&fields, encoder),
                5usize => self.code.encode_field(&fields, encoder),
                6usize => self.size.encode_field(&fields, encoder),
                _ =>
                // impl Drop for Log {
                //     fn drop(&mut self) {
                //         println!("log dropped");
                //     }
                // }

                // let mut file = File::open("testtemp").unwrap();
                // let mut vec = Vec::new();
                // file.read_to_end(&mut vec).unwrap();
                // println!("veclen={:?}", vec.len());
                // WriteBuf::write(&mut buf, vec.as_slice());

                //    println!("{:?}", decoded);
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
                    address: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    identity: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    userid: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    date: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    request: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    code: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    size: serialization::CompositeDecoder::decode_element(&mut struc)?,
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
        fields: &mut serialization::binary_format::Fields,
        field: &mut Log,
        decoder: &mut _D,
    ) -> Result<(), _D::Error> {
        let mut fields = fields.clone();
        if fields.len() == 0 {
            serialization::binary_format::DecodeField::decode_field(&mut fields, field, decoder)
        } else {
            match *fields.pop_last() as usize {
                0usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.address,
                    decoder,
                ),
                1usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.identity,
                    decoder,
                ),
                2usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.userid,
                    decoder,
                ),
                3usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.date,
                    decoder,
                ),
                4usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.request,
                    decoder,
                ),
                5usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.code,
                    decoder,
                ),
                6usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.size,
                    decoder,
                ),
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code"
                    ));
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Log {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Log {
    #[inline]
    fn eq(&self, other: &Log) -> bool {
        self.address == other.address
            && self.identity == other.identity
            && self.userid == other.userid
            && self.date == other.date
            && self.request == other.request
            && self.code == other.code
            && self.size == other.size
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Log {
    #[inline]
    fn partial_cmp(&self, other: &Log) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.address, &other.address) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(&self.identity, &other.identity) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.userid, &other.userid) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                match ::core::cmp::PartialOrd::partial_cmp(&self.date, &other.date)
                                {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                        match ::core::cmp::PartialOrd::partial_cmp(
                                            &self.request,
                                            &other.request,
                                        ) {
                                            ::core::option::Option::Some(
                                                ::core::cmp::Ordering::Equal,
                                            ) => match ::core::cmp::PartialOrd::partial_cmp(
                                                &self.code,
                                                &other.code,
                                            ) {
                                                ::core::option::Option::Some(
                                                    ::core::cmp::Ordering::Equal,
                                                ) => ::core::cmp::PartialOrd::partial_cmp(
                                                    &self.size,
                                                    &other.size,
                                                ),
                                                cmp => cmp,
                                            },
                                            cmp => cmp,
                                        }
                                    }
                                    cmp => cmp,
                                }
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Log {
    #[inline]
    fn cmp(&self, other: &Log) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.address, &other.address) {
            ::core::cmp::Ordering::Equal => {
                match ::core::cmp::Ord::cmp(&self.identity, &other.identity) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.userid, &other.userid) {
                            ::core::cmp::Ordering::Equal => {
                                match ::core::cmp::Ord::cmp(&self.date, &other.date) {
                                    ::core::cmp::Ordering::Equal => {
                                        match ::core::cmp::Ord::cmp(&self.request, &other.request) {
                                            ::core::cmp::Ordering::Equal => {
                                                match ::core::cmp::Ord::cmp(&self.code, &other.code)
                                                {
                                                    ::core::cmp::Ordering::Equal => {
                                                        ::core::cmp::Ord::cmp(
                                                            &self.size,
                                                            &other.size,
                                                        )
                                                    }
                                                    cmp => cmp,
                                                }
                                            }
                                            cmp => cmp,
                                        }
                                    }
                                    cmp => cmp,
                                }
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
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
        let _: ::core::cmp::AssertParamIsEq<Address>;
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
        let _: ::core::cmp::AssertParamIsEq<u64>;
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Log {
    #[inline]
    fn clone(&self) -> Log {
        Log {
            address: ::core::clone::Clone::clone(&self.address),
            identity: ::core::clone::Clone::clone(&self.identity),
            userid: ::core::clone::Clone::clone(&self.userid),
            date: ::core::clone::Clone::clone(&self.date),
            request: ::core::clone::Clone::clone(&self.request),
            code: ::core::clone::Clone::clone(&self.code),
            size: ::core::clone::Clone::clone(&self.size),
        }
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
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 0 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.logs);
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
        fields: &mut serialization::binary_format::Fields,
        field: &mut Logs,
        decoder: &mut _D,
    ) -> Result<(), _D::Error> {
        let mut fields = fields.clone();
        if fields.len() == 0 {
            serialization::binary_format::DecodeField::decode_field(&mut fields, field, decoder)
        } else {
            match *fields.pop_last() as usize {
                0usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.logs,
                    decoder,
                ),
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code"
                    ));
                }
            }
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
#[automatically_derived]
impl ::core::clone::Clone for Logs {
    #[inline]
    fn clone(&self) -> Logs {
        Logs {
            logs: ::core::clone::Clone::clone(&self.logs),
        }
    }
}
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for Address {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f, "Address", "x0", &self.x0, "x1", &self.x1, "x2", &self.x2, "x3", &&self.x3,
        )
    }
}
impl<'de> const serialization::binary_format::SerialDescriptor for Address {
    const N: usize = <u8 as serialization::binary_format::SerialDescriptor>::N
        + <u8 as serialization::binary_format::SerialDescriptor>::N
        + <u8 as serialization::binary_format::SerialDescriptor>::N
        + <u8 as serialization::binary_format::SerialDescriptor>::N
        + 4usize
        + 1;
    fn fields<_C: const serialization::CheckPrimitiveTypeSize>(
    ) -> serialization::constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields(
            {
                #[allow(invalid_value)]
                let value: std::mem::MaybeUninit<Address> = std::mem::MaybeUninit::zeroed();
                let value = unsafe { value.assume_init_ref() };
                let mut padding_calc = serialization::binary_format::SizeCalcState::new(value);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 0 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.x0);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 1 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.x1);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 2 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.x2);
                serialization::binary_format::SizeCalcState::next_field::<
                    _,
                    _C,
                    { 3 as serialization::binary_format::Field },
                >(&mut padding_calc, &value.x3);
                serialization::binary_format::SizeCalcState::finish(padding_calc)
            },
            serialization::binary_format::SerialSize::unsized_field_of(),
        )
    }
}
impl<'de> serialization::Encode for Address {
    fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Address, _E>() } {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.x0)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.x1)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.x2)?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.x3)?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::EncodeField for Address {
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
                0usize => self.x0.encode_field(&fields, encoder),
                1usize => self.x1.encode_field(&fields, encoder),
                2usize => self.x2.encode_field(&fields, encoder),
                3usize => self.x3.encode_field(&fields, encoder),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
}
impl<'de> serialization::Decode<'de> for Address {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<Address, _D>() } {
                let mut struc = decoder.decode_struct()?;
                let result = Address {
                    x0: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    x1: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    x2: serialization::CompositeDecoder::decode_element(&mut struc)?,
                    x3: serialization::CompositeDecoder::decode_element(&mut struc)?,
                };
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::DecodeField<'de> for Address {
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &mut serialization::binary_format::Fields,
        field: &mut Address,
        decoder: &mut _D,
    ) -> Result<(), _D::Error> {
        let mut fields = fields.clone();
        if fields.len() == 0 {
            serialization::binary_format::DecodeField::decode_field(&mut fields, field, decoder)
        } else {
            match *fields.pop_last() as usize {
                0usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.x0,
                    decoder,
                ),
                1usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.x1,
                    decoder,
                ),
                2usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.x2,
                    decoder,
                ),
                3usize => serialization::binary_format::DecodeField::decode_field(
                    &mut fields,
                    &mut field.x3,
                    decoder,
                ),
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code"
                    ));
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Address {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Address {
    #[inline]
    fn eq(&self, other: &Address) -> bool {
        self.x0 == other.x0 && self.x1 == other.x1 && self.x2 == other.x2 && self.x3 == other.x3
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Address {
    #[inline]
    fn partial_cmp(&self, other: &Address) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.x0, &other.x0) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(&self.x1, &other.x1) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.x2, &other.x2) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.x3, &other.x3)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Address {
    #[inline]
    fn cmp(&self, other: &Address) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.x0, &other.x0) {
            ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.x1, &other.x1) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.x2, &other.x2) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.x3, &other.x3),
                    cmp => cmp,
                },
                cmp => cmp,
            },
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Address {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u8>;
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Address {
    #[inline]
    fn clone(&self) -> Address {
        let _: ::core::clone::AssertParamIsClone<u8>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Address {}
