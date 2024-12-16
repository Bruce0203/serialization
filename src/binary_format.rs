use core::slice;
use std::{
    fmt::Debug,
    mem::{transmute, ManuallyDrop, MaybeUninit},
    usize,
};

use constvec::{ConstEq, ConstVec};

use crate::{
    BinaryDecoder, CheckPrimitiveTypeSize, CompositeDecoder, CompositeEncoder, Decode, DecodeError,
    Decoder, Encode, EncodeError, Encoder,
};

pub trait EncodeField: Encode {
    fn encode_field<E: Encoder>(&self, field_indexes: &Fields, encoder: E) -> Result<(), E::Error>;
}

pub trait DecodeField<'de>: Sized {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        fields: &mut Fields,
        field: &mut Self,
        decoder: &mut D,
    ) -> Result<(), D::Error>;
}

#[const_trait]
pub trait SerialDescriptor {
    const N: usize;
    fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]>;
}

pub type Field = u16;
pub type Fields = ConstVec<[Field; 256]>;

#[derive(Eq, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum SerialSize {
    Unsized { fields: Fields },
    Padding(usize),
    Sized { start: usize, len: usize },
}

impl SerialSize {
    pub const fn unsized_field_of<const N: usize>() -> ConstVec<[SerialSize; N]> {
        ConstVec::new(1, [const { Self::unsized_of() }; N])
    }

    pub const fn unsized_of() -> SerialSize {
        SerialSize::Unsized {
            fields: ConstVec::new(0, unsafe { MaybeUninit::zeroed().assume_init() }),
        }
    }

    pub const fn clone(&self) -> Self {
        match self {
            SerialSize::Unsized { fields } => SerialSize::Unsized {
                fields: fields.clone(),
            },
            SerialSize::Padding(padding) => SerialSize::Padding(*padding),
            SerialSize::Sized { start, len } => SerialSize::Sized {
                start: *start,
                len: *len,
            },
        }
    }
}

impl const ConstEq for SerialSize {
    fn eq(&self, rhs: &Self) -> bool {
        match self {
            SerialSize::Unsized { fields } => {
                if let SerialSize::Unsized { fields: rhs_fields } = rhs {
                    if ConstEq::eq(fields, &rhs_fields) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            SerialSize::Padding(padding) => {
                if let SerialSize::Padding(rhs_padding) = rhs {
                    if *padding == *rhs_padding {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            SerialSize::Sized { start, len } => {
                if let SerialSize::Sized {
                    start: rhs_start,
                    len: rhs_len,
                } = rhs
                {
                    if *start == *rhs_start && *len == *rhs_len {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}

impl<T: Encode> EncodeField for T {
    default fn encode_field<E: Encoder>(
        &self,
        _field_indexes: &Fields,
        encoder: E,
    ) -> Result<(), E::Error> {
        self.encode(encoder)
    }
}

impl<'de, T: Decode<'de>> DecodeField<'de> for T {
    default unsafe fn decode_field<D: CompositeDecoder<'de>>(
        _field_indexes: &mut Fields,
        field: &mut Self,
        decoder: &mut D,
    ) -> Result<(), D::Error> {
        let field: &mut MaybeUninit<T> = const_transmute(field);
        decoder.decode_element(field)
    }
}

macro_rules! impl_serial_descriptor {
    ($($type:ty),*) => {$(
        impl const SerialDescriptor for $type {
            const N: usize = 1;
            fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]> {
                if C::is_sized::<Self>() {
                    sized_field_of::<Self>()
                } else {
                    SerialSize::unsized_field_of()
                }
            }
        }
    )*};
}

impl_serial_descriptor!(
    u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, bool, usize, isize, i128, u128
);

pub struct SizeCalcState<'a, T: const SerialDescriptor>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    temp: ConstVec<[SerialSize; T::N]>,
    value: &'a T,
    cursor: usize,
    counter: usize,
    board: [usize; size_of::<T>()],
}

impl<'a, T: const SerialDescriptor> SizeCalcState<'a, T>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    const PADDING_ID: usize = usize::MAX;
    pub const fn new(value: &'a T) -> Self {
        Self {
            temp: ConstVec::new(0, unsafe { MaybeUninit::zeroed().assume_init() }),
            value,
            cursor: 0,
            counter: 0,
            board: [Self::PADDING_ID; _],
        }
    }

    pub const fn next_field<
        E: const SerialDescriptor,
        C: const CheckPrimitiveTypeSize,
        const FIELD: Field,
    >(
        &mut self,
        field_ptr: &E,
    ) where
        [(); T::N]:,
        [(); <E as SerialDescriptor>::N]:,
    {
        let offset = unsafe { (field_ptr as *const E).byte_sub_ptr(self.value as *const T) };
        let size = size_of::<E>();
        let mut i = offset;
        while i < offset + size {
            self.board[i] = self.counter;
            i += 1;
        }
        self.counter += 1;
        let slice = const { add_to_fields(<E as SerialDescriptor>::fields::<C>(), FIELD) };
        self.temp.push(&SerialSize::Padding(slice.len()));
        self.temp.append(&slice);
    }

    pub const fn finish(mut self) -> ConstVec<[SerialSize; T::N]>
    where
        [(); T::N]:,
    {
        let mut result: ConstVec<[SerialSize; T::N]> =
            ConstVec::new(0, unsafe { MaybeUninit::zeroed().assume_init() });
        let mut i = 0;
        while i < self.board.len() {
            let field_index = self.board[i];
            if field_index == Self::PADDING_ID {
                let mut padding = 0;
                while i < self.board.len() {
                    let field_index = self.board[i];
                    if field_index != Self::PADDING_ID {
                        break;
                    }
                    padding += 1;
                    i += 1;
                }
                i -= 1;
                let v = SerialSize::Padding(padding);
                result.push(&v);
            } else {
                let mut j = 0;
                let mut k = 0;
                while j < self.temp.len() {
                    let v = self.temp.as_slice()[j].clone();
                    let fields_len = match v {
                        SerialSize::Padding(size) => size,
                        _ => unreachable!(),
                    };
                    if field_index == k {
                        j += 1;
                        let repeat = j + fields_len;
                        while j < repeat {
                            //TODO clean up code
                            const fn serial_sized(start: usize, len: usize) -> SerialSize {
                                SerialSize::Sized { start, len }
                            }
                            const fn aa(serial_size: &SerialSize, adder: usize) -> SerialSize {
                                match serial_size {
                                    SerialSize::Sized { start, len } => {
                                        serial_sized(adder + *start, *len)
                                    }
                                    size => size.clone(),
                                }
                            }
                            result.push(&aa(&self.temp.get(j), i));
                            j += 1;
                        }
                        loop {
                            i += 1;
                            if i >= self.board.len() || self.board[i] != field_index {
                                i -= 1;
                                break;
                            }
                        }
                        break;
                    } else {
                        k += 1;
                        j += fields_len + 1;
                    }
                }
            }
            i += 1;
        }
        let last_padding = size_of::<T>() - self.cursor;
        if last_padding > 0 {
            self.temp.push(&SerialSize::Padding(last_padding));
        }
        std::mem::forget(self);
        result
    }
}

pub struct WritableField<'a, T: EncodeField> {
    value: &'a T,
    fields: Fields,
}

impl<'a, T: EncodeField> Encode for WritableField<'a, T> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        self.value.encode_field(&self.fields, encoder)
    }
}

pub struct WritingBytes<'a>(&'a [u8]);

impl Encode for WritingBytes<'_> {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder
            .write_bytes(self.0)
            .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
        Ok(())
    }
}

pub fn encode2<T: const SerialDescriptor + Encode, E: Encoder>(
    value: &T,
    encoder: E,
) -> Result<(), E::Error>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    let mut i = 0;
    let fields = const { T::fields::<E>() };
    let mut tup = encoder.encode_tuple()?;
    while i < fields.len() {
        match fields.get(i) {
            SerialSize::Unsized { fields } => {
                tup.encode_element(&WritableField {
                    value,
                    fields: fields.clone(),
                })?;
            }
            SerialSize::Padding(_) => {}
            SerialSize::Sized { start, len: size } => {
                let slice: *const u8 = unsafe { transmute(value) };
                let ptr = unsafe { slice.byte_add(*start) };
                let value = unsafe { slice::from_raw_parts(ptr, *size) };
                tup.encode_element(&WritingBytes(value))?;
            }
        }
        i += 1;
    }
    tup.end()?;
    Ok(())
}

pub fn decode2<'de, T: Sized + const SerialDescriptor + Decode<'de>, D: Decoder<'de>>(
    decoder: D,
    place: &mut MaybeUninit<T>,
) -> Result<(), D::Error>
where
    [(); T::N]:,
{
    let mut i = 0;
    let mut tup = decoder.decode_tuple()?;
    let fields = const { T::fields::<D>() };
    let place: &mut T = unsafe { const_transmute(place) };
    while i < fields.len() {
        match fields.get(i) {
            SerialSize::Unsized { fields } => {
                unsafe { T::decode_field(&mut fields.clone(), place, &mut tup) }?
            }
            SerialSize::Padding(_size) => {}
            SerialSize::Sized { start, len } => {
                unsafe {
                    (place as *mut _ as *mut u8)
                        .byte_add(*start)
                        .copy_from_nonoverlapping(
                            tup.read_bytes(*len)
                                .map_err(|()| DecodeError::not_enough_bytes_in_the_buffer())?
                                as *const _ as *const u8,
                            *len,
                        );
                };
            }
        }
        i += 1;
    }
    tup.end()?;
    Ok(())
}

pub const fn calc_field_offset<T, F>(base_ptr: &T, field_ptr: &F) -> usize {
    let base_ptr = base_ptr as *const _ as *const u8;
    unsafe { (field_ptr as *const _ as *const u8).byte_sub_ptr(base_ptr) }
}

pub const fn compact_fields<const N: usize>(
    fields: ConstVec<[SerialSize; N]>,
    or_else: ConstVec<[SerialSize; N]>,
) -> ConstVec<[SerialSize; N]> {
    if fields.len() == 0 {
        return or_else;
    }
    if fields.len() <= 1 {
        return fields.clone();
    }
    let mut result: [SerialSize; N] = [const { SerialSize::Sized { start: 0, len: 0 } }; N];
    let mut result_i = 0;
    let mut slice = fields.clone();
    loop {
        let mut j = 0;
        let mut size: Option<SerialSize> = None;
        while j < slice.as_slice().len() {
            let size_clone = if let Some(ref size) = &size {
                Some(size.clone())
            } else {
                None
            };
            match (size_clone, slice.as_slice()[j].clone()) {
                (
                    Some(SerialSize::Sized {
                        start,
                        len: origin_size,
                    }),
                    SerialSize::Sized {
                        start: _,
                        len: adder,
                    },
                ) => {
                    size = Some(SerialSize::Sized {
                        start,
                        len: origin_size + adder,
                    });
                }
                (Some(SerialSize::Padding(origin_size)), SerialSize::Padding(adder)) => {
                    size = Some(SerialSize::Padding(origin_size + adder));
                }
                (Some(SerialSize::Sized { .. }), SerialSize::Padding(new_size)) if j == 0 => {
                    size = Some(SerialSize::Padding(new_size));
                }
                (Some(SerialSize::Sized { .. }), SerialSize::Padding(_))
                | (Some(SerialSize::Padding(_)), SerialSize::Sized { .. })
                | (Some(_), SerialSize::Unsized { .. }) => {
                    break;
                }
                (None, SerialSize::Unsized { fields }) => {
                    size = Some(SerialSize::Unsized { fields });
                    j += 1;
                    break;
                }
                (_origin_size, new_size) => {
                    size = Some(new_size);
                }
            };
            j += 1;
        }
        result[result_i] = size.unwrap();
        result_i += 1;
        if j == slice.as_slice().len() {
            break;
        }
        slice = slice.cutoff_front_at(j);
    }
    ConstVec::new(result_i, result)
}

pub const fn is_not_fast_binary<
    T: 'static + const SerialDescriptor,
    E: const CheckPrimitiveTypeSize,
>() -> bool
where
    [(); T::N]:,
{
    let fields = T::fields::<E>();
    fields.len() == 1 && ConstEq::eq(fields.get(0), &SerialSize::unsized_of())
}

pub const fn sized_field_of<T: SerialDescriptor>() -> ConstVec<[SerialSize; T::N]> {
    let value: [[u8; std::mem::size_of::<SerialSize>()]; T::N] =
        [[0_u8; std::mem::size_of::<SerialSize>()]; T::N];
    let value: [SerialSize; T::N] = unsafe { const_transmute(value) };
    let mut value = ConstVec::new(1, value);
    *value.get_mut(0) = SerialSize::Sized {
        start: 0,
        len: size_of::<T>(),
    };
    value
}

pub const fn add_to_fields<T: const SerialDescriptor>(
    fields: ConstVec<[SerialSize; T::N]>,
    field: Field,
) -> ConstVec<[SerialSize; T::N]> {
    let mut fields = fields.clone();
    let mut i = 0;
    while i < fields.len() {
        match fields.get_mut(i) {
            SerialSize::Unsized { fields } => {
                fields.push(&field);
            }
            _ => {}
        }
        i += 1;
    }
    fields
}

impl<T> const SerialDescriptor for T {
    default const N: usize = 1;

    default fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]> {
        SerialSize::unsized_field_of()
    }
}
