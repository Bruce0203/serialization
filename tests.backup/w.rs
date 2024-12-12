#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

struct C {}

impl<'de> const serialization::binary_format::SerialDescriptor for C {
    const N: usize = 0usize + 1;
    fn fields<_C: const serialization::CheckPrimitiveTypeSize>() -> constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields({
            #[allow(invalid_value)]
            let value: C = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            let mut padding_calc = serialization::binary_format::SizeCalcState::new(&value);
            serialization::binary_format::SizeCalcState::finish(padding_calc)
        })
    }
}
impl<'de> serialization::Encode for C {
    fn encode<_E: serialization::Encoder>(&self, encoder: _E) -> Result<(), _E::Error> {
        {
            if <C as serialization::binary_format::SerialDescriptor>::fields::<_E>().as_slice()[0]
                == serialization::binary_format::SerialSize::unsized_of::<C>()
            {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::EncodeField for C {
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
                _ => unreachable!(),
            }
        }
    }
}
impl<'de> serialization::Decode<'de> for C {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if <C as serialization::binary_format::SerialDescriptor>::fields::<_D>().as_slice()[0]
                == serialization::binary_format::SerialSize::unsized_of::<C>()
            {
                let mut struc = decoder.decode_struct()?;
                let result = C {};
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de> serialization::binary_format::DecodeField<'de> for C {
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut _D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, _D::Error> {
        #[allow(invalid_value)]
        let result: C = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let mut state =
            serialization::binary_format::DecodeFieldState::new(&result, fields.clone());
        match state.start(decoder) {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index as usize {
                _ => unreachable!(),
            }),
        }
    }
}
