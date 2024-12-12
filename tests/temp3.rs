#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use serialization::binary_format::SerialDescriptor;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bar {
    field1: u8,
    field2: u16,
    field3: u32,
}

impl serialization::Encode for Bar {
    fn encode<E: serialization::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        if <Self as serialization::binary_format::SerialDescriptor>::fields::<E>().as_slice()[0]
            == serialization::binary_format::SerialSize::unsized_of::<Self>()
        {
            let mut struc = encoder.encode_struct()?;
            serialization::CompositeEncoder::encode_element(&mut struc, &self.field1)?;
            serialization::CompositeEncoder::encode_element(&mut struc, &self.field2)?;
            serialization::CompositeEncoder::encode_element(&mut struc, &self.field3)?;
            serialization::CompositeEncoder::end(struc)?;
            Ok(())
        } else {
            serialization::binary_format::encode2(self, encoder)
        }
    }
}

impl serialization::binary_format::EncodeField for Bar {
    fn encode_field<E: serialization::Encoder>(
        &self,
        fields: &serialization::binary_format::Fields,
        encoder: E,
    ) -> Result<(), E::Error> {
        if fields.len() == 0 {
            serialization::Encode::encode(&self, encoder)
        } else {
            let mut fields = fields.clone();
            match fields.pop_last() {
                0 => self.field1.encode_field(&fields, encoder),
                1 => self.field2.encode_field(&fields, encoder),
                2 => self.field3.encode_field(&fields, encoder),
                _ => unreachable!(),
            }
        }
    }
}

impl<'de> serialization::Decode<'de> for Bar {
    fn decode<D: serialization::Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        if <Self as serialization::binary_format::SerialDescriptor>::fields::<D>().as_slice()[0]
            == serialization::binary_format::SerialSize::unsized_of::<Self>()
        {
            let mut struc = decoder.decode_struct()?;
            let result = Self {
                field1: serialization::CompositeDecoder::decode_element(&mut struc)?,
                field2: serialization::CompositeDecoder::decode_element(&mut struc)?,
                field3: serialization::CompositeDecoder::decode_element(&mut struc)?,
            };
            serialization::CompositeDecoder::end(struc)?;
            Ok(result)
        } else {
            serialization::binary_format::decode2(decoder)
        }
    }
}

impl<'de> serialization::binary_format::DecodeField<'de> for Bar {
    unsafe fn decode_field<D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, D::Error> {
        #[allow(invalid_value)]
        let result: Self = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let mut state =
            serialization::binary_format::DecodeFieldState::new(&result, fields.clone());
        match state.start(decoder) {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index {
                0 => state.decode_field(decoder, &result.field1)?,
                1 => state.decode_field(decoder, &result.field2)?,
                2 => state.decode_field(decoder, &result.field3)?,
                _ => unreachable!(),
            }),
        }
    }
}
impl const serialization::binary_format::SerialDescriptor for Bar {
    const N: usize = <u8 as serialization::binary_format::SerialDescriptor>::N
        + <u16 as serialization::binary_format::SerialDescriptor>::N
        + <u32 as serialization::binary_format::SerialDescriptor>::N
        + 3
        + 1;

    fn fields<C: const serialization::CheckPrimitiveTypeSize>(
    ) -> constvec::ConstVec<[serialization::binary_format::SerialSize; <Self as SerialDescriptor>::N]>
    {
        serialization::binary_format::compact_fields({
            #[allow(invalid_value)]
            let value: Self = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            let mut padding_calc = serialization::binary_format::SizeCalcState::<Self>::new(&value);
            serialization::binary_format::SizeCalcState::next_field::<_, C>(
                &mut padding_calc,
                &value.field1,
            );
            serialization::binary_format::SizeCalcState::next_field::<_, C>(
                &mut padding_calc,
                &value.field2,
            );
            serialization::binary_format::SizeCalcState::next_field::<_, C>(
                &mut padding_calc,
                &value.field3,
            );
            serialization::binary_format::SizeCalcState::finish(padding_calc)
        })
    }
}
