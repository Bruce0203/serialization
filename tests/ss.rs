#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::marker::PhantomData;

use serialization::binary_format::SerialDescriptor;

struct O<T: Sized> {
    value: T,
}
impl<'de, T: 'static + Sized> const serialization::binary_format::SerialDescriptor for O<T>
where
    [(); <T as SerialDescriptor>::N]:,
{
    const N: usize = <T as serialization::binary_format::SerialDescriptor>::N + 1usize + 1;
    fn fields<C: const serialization::CheckPrimitiveTypeSize>() -> constvec::ConstVec<
        [serialization::binary_format::SerialSize;
            <Self as serialization::binary_format::SerialDescriptor>::N],
    > {
        serialization::binary_format::compact_fields({
            #[allow(invalid_value)]
            let value: O<T> = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            let mut padding_calc = serialization::binary_format::SizeCalcState::new(&value);
            serialization::binary_format::SizeCalcState::next_field::<_, C>(
                &mut padding_calc,
                &value.value,
            );
            serialization::binary_format::SizeCalcState::finish(padding_calc)
        })
    }
}
impl<'de, T: serialization::Encode> serialization::Encode for O<T> {
    fn encode<E: serialization::Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        {
            if <O<T> as serialization::binary_format::SerialDescriptor>::fields::<E>().as_slice()[0]
                == serialization::binary_format::SerialSize::unsized_of::<O<T>>()
            {
                let mut struc = encoder.encode_struct()?;
                serialization::CompositeEncoder::encode_element(&mut struc, &self.value)?;
                serialization::CompositeEncoder::end(struc)?;
                Ok(())
            } else {
                serialization::binary_format::encode2(self, encoder)
            }
        }
    }
}
impl<'de, T: serialization::binary_format::EncodeField + serialization::Encode>
    serialization::binary_format::EncodeField for O<T>
{
    fn encode_field<E: serialization::Encoder>(
        &self,
        fields: &serialization::binary_format::Fields,
        encoder: E,
    ) -> Result<(), E::Error> {
        if fields.len() == 0 {
            serialization::Encode::encode(&self, encoder)
        } else {
            let mut fields = fields.clone();
            match *fields.pop_last() as usize {
                0usize => self.value.encode_field(&fields, encoder),
                _ => unreachable!(),
            }
        }
    }
}
impl<'de, T: serialization::Decode<'de>> serialization::Decode<'de> for O<T> {
    fn decode<_D: serialization::Decoder<'de>>(decoder: _D) -> Result<Self, _D::Error> {
        {
            if <O<T> as serialization::binary_format::SerialDescriptor>::fields::<_D>().as_slice()
                [0]
                == serialization::binary_format::SerialSize::unsized_of::<O<T>>()
            {
                let mut struc = decoder.decode_struct()?;
                let result = O::<T> {
                    value: serialization::CompositeDecoder::decode_element(&mut struc)?,
                };
                serialization::CompositeDecoder::end(struc)?;
                Ok(result)
            } else {
                serialization::binary_format::decode2(decoder)
            }
        }
    }
}
impl<'de, T: serialization::binary_format::DecodeField<'de> + serialization::Decode<'de>>
    serialization::binary_format::DecodeField<'de> for O<T>
{
    unsafe fn decode_field<_D: serialization::CompositeDecoder<'de>>(
        fields: &serialization::binary_format::Fields,
        decoder: &mut _D,
    ) -> Result<serialization::binary_format::ReadableField<Self>, _D::Error> {
        #[allow(invalid_value)]
        let result: O<T> = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let mut state =
            serialization::binary_format::DecodeFieldState::new(&result, fields.clone());
        match state.start(decoder) {
            Ok(value) => {
                return value;
            }
            Err(index) => Ok(match index as usize {
                0usize => state.decode_field(decoder, &result.value)?,
                _ => unreachable!(),
            }),
        }
    }
}
