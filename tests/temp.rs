#![feature(coverage_attribute)]
#![feature(rustc_attrs)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(panic_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(prelude_import)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

use fastbuf::{Buffer, ReadBuf};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[repr(transparent)]
pub struct TestA {
    value4: Vec<u8>,
}
#[automatically_derived]
impl ::core::fmt::Debug for TestA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "TestA", "value4", &&self.value4)
    }
}
impl<'de> const serialization::binary_format::SerialDescriptor for TestA {
    const N: usize = <Vec<u8> as serialization::binary_format::SerialDescriptor>::N + 1usize + 1;
    fn fields<_C: const serialization::CheckPrimitiveTypeSize>(
    ) -> serialization::constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields(
            {
                #[allow(invalid_value)]
                let value: std::mem::MaybeUninit<TestA> = std::mem::MaybeUninit::zeroed();
                let value = unsafe { value.assume_init_ref() };
                let mut padding_calc = serialization::binary_format::SizeCalcState::new(value);
                serialization::binary_format::SizeCalcState::next_field::<_, _C, 0_u16>(
                    &mut padding_calc,
                    &value.value4,
                );
                serialization::binary_format::SizeCalcState::finish(padding_calc)
            },
            serialization::binary_format::SerialSize::unsized_field_of(),
        )
    }
}
impl<'de> serialization::Encode for TestA {
    fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<TestA, _E>() } {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.value4)?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::EncodeField for TestA {
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
                0usize => self.value4.encode_field(&fields, encoder),
                _ =>
                // [123, 0, 0, 0, 234, 45, 48, 0, !!0, 12, 32, 0, 0]
                {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }
    }
}
impl<'de> serialization::Decode<'de> for TestA {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if const { serialization::binary_format::is_not_fast_binary::<TestA, _D>() } {
                let mut struc = decoder.decode_struct()?;
                let result = TestA {
                    value4: serialization::CompositeDecoder::decode_element(&mut struc)?,
                };
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::DecodeField<'de> for TestA {
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut _D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, _D::Error> {
        #[allow(invalid_value)]
        let result: std::mem::MaybeUninit<TestA> = std::mem::MaybeUninit::zeroed();
        let result = unsafe { result.assume_init_ref() };
        let mut state = serialization::binary_format::DecodeFieldState::new(result, fields.clone());
        match state.start::<_D>() {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index as usize {
                0usize => state.decode_field(decoder, &result.value4)?,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TestA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TestA {
    #[inline]
    fn eq(&self, other: &TestA) -> bool {
        self.value4 == other.value4
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for TestA {
    #[inline]
    fn partial_cmp(&self, other: &TestA) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.value4, &other.value4)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for TestA {
    #[inline]
    fn cmp(&self, other: &TestA) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.value4, &other.value4)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TestA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
    }
}
