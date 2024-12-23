use core::slice;
use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{size_of, transmute, MaybeUninit},
    usize,
};

use constvec::{ConstEq, ConstVec};
use seq_macro::seq;

use crate::{
    const_transmute, CheckPrimitiveTypeSize, Decode, DecodeError, Decoder, Encode, EncodeError,
    Encoder,
};

#[const_trait]
pub trait OffsetAccumlator: Decode + Encode {
    fn current_decode_fn<'a, D: Decoder>() -> &'a dyn UnsafeDecode<D>
    where
        Self: 'a,
    {
        &DecodeWrapper::<Self>(PhantomData) as &'a dyn UnsafeDecode<D>
    }

    fn current_encode_fn<'a, E: Encoder>() -> &'a dyn UnsafeEncode<E>
    where
        Self: 'a,
    {
        &EncodeWrapper::<Self>(PhantomData) as &dyn UnsafeEncode<E>
    }

    fn acc_offset(fields: &Fields, field_index: Field) -> usize;

    fn acc_decode_fn<'a, D: Decoder>(
        _fields: &Fields,
        _field_index: Field,
    ) -> &'a dyn UnsafeDecode<D>
    where
        Self: 'a,
    {
        Self::current_decode_fn()
    }

    fn acc_encode_fn<'a, E: Encoder>(
        _fields: &Fields,
        _field_index: Field,
    ) -> &'a dyn UnsafeEncode<E>
    where
        Self: 'a;
}

impl<T: Decode + Encode> const OffsetAccumlator for T {
    default fn current_encode_fn<'a, E: Encoder>() -> &'a dyn UnsafeEncode<E>
    where
        Self: 'a,
    {
        &EncodeWrapper::<Self>(PhantomData) as &'a dyn UnsafeEncode<E>
    }

    default fn current_decode_fn<'a, D: Decoder>() -> &'a dyn UnsafeDecode<D>
    where
        Self: 'a,
    {
        &DecodeWrapper::<Self>(PhantomData) as &'a dyn UnsafeDecode<D>
    }

    default fn acc_offset(_fields: &Fields, _field_index: Field) -> usize {
        0
    }

    default fn acc_decode_fn<'a, D: Decoder>(
        _fields: &Fields,
        _field_index: Field,
    ) -> &'a dyn UnsafeDecode<D>
    where
        Self: 'a,
    {
        Self::current_decode_fn()
    }

    default fn acc_encode_fn<'a, E: Encoder>(
        _fields: &Fields,
        _field_index: Field,
    ) -> &'a dyn UnsafeEncode<E>
    where
        Self: 'a,
    {
        Self::current_encode_fn()
    }
}

pub trait UnsafeDecode<D: Decoder> {
    fn decode_unsafe(&self, decoder: &mut D, place: *mut u8) -> Result<(), D::Error>;
}

pub trait UnsafeEncode<E: Encoder> {
    fn encode_unsafe(&self, encoder: &mut E, value: *const u8) -> Result<(), E::Error>;
}

pub struct DecodeWrapper<T>(pub PhantomData<T>);

impl<T: Decode, D: Decoder> UnsafeDecode<D> for T {
    fn decode_unsafe(&self, decoder: &mut D, place: *mut u8) -> Result<(), D::Error> {
        T::decode(decoder, unsafe { &mut *(place as *mut MaybeUninit<T>) })
    }
}

impl<T: Decode> Decode for DecodeWrapper<T> {
    fn decode<D: Decoder>(decoder: &mut D, place: &mut MaybeUninit<Self>) -> Result<(), D::Error> {
        T::decode(decoder, unsafe {
            &mut *(place as *mut _ as *mut MaybeUninit<T>)
        })
    }
}

pub struct EncodeWrapper<T>(pub PhantomData<T>);
impl<T: Encode, E: Encoder> UnsafeEncode<E> for T {
    fn encode_unsafe(
        &self,
        encoder: &mut E,
        value: *const u8,
    ) -> Result<(), <E as Encoder>::Error> {
        T::encode(unsafe { &*(value as *const T) }, encoder)
    }
}

impl<T: Encode, E: Encoder> UnsafeEncode<E> for EncodeWrapper<T> {
    fn encode_unsafe(
        &self,
        encoder: &mut E,
        value: *const u8,
    ) -> Result<(), <E as Encoder>::Error> {
        T::encode(unsafe { &*(value as *const T) }, encoder)
    }
}

pub struct EmptyDecodeFn;
impl Decode for EmptyDecodeFn {
    fn decode<D: Decoder>(
        _decoder: &mut D,
        _place: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}

pub struct EmptyEncodeFn;
impl Encode for EmptyEncodeFn {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

#[const_trait]
pub trait SerialDescriptor {
    const N: usize;
    fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]>;
}

impl<T> const SerialDescriptor for T {
    default const N: usize = 1;

    default fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]> {
        SerialSize::unsized_field_of()
    }
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
            fields: Fields::EMPTY,
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

impl_serial_descriptor!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128);

pub fn encode2<T: const OffsetAccumlator + const SerialDescriptor + Encode, E: Encoder>(
    value: &T,
    encoder: &mut E,
) -> Result<(), E::Error>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    let commands = &const { collect_encode_functions::<T, E>() };
    let mut i = 0;
    while i < commands.len() {
        match commands.get(i) {
            SerialCommand::Unsized { offset, function } => {
                function.encode_unsafe(encoder, unsafe {
                    (value as *const _ as *const u8).wrapping_add(*offset)
                })?;
            }
            SerialCommand::Sized { start, len } => {
                let slice: *const u8 = unsafe { transmute(value) };
                let ptr = unsafe { slice.wrapping_add(*start) };
                let value = unsafe { slice::from_raw_parts(ptr, *len) };
                encoder
                    .write_bytes(value)
                    .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
            }
            SerialCommand::Padding => {}
        }
        i += 1;
    }
    Ok(())
}

pub enum SerialCommand<F> {
    Unsized { offset: usize, function: F },
    Sized { start: usize, len: usize },
    Padding,
}

impl<F> Debug for SerialCommand<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsized {
                offset,
                function: _,
            } => f.debug_struct("Unsized").field("offset", offset).finish(),
            Self::Sized { start, len } => f
                .debug_struct("Sized")
                .field("start", start)
                .field("len", len)
                .finish(),
            Self::Padding => write!(f, "Padding"),
        }
    }
}

impl<F> SerialCommand<F> {
    pub const fn padding() -> Self {
        Self::Padding
    }
}

const fn into_encode_command<'a, T: 'a + const OffsetAccumlator, E: Encoder>(
    serial_size: &SerialSize,
) -> SerialCommand<&'a dyn UnsafeEncode<E>>
where
    [(); T::N]:,
{
    match serial_size {
        SerialSize::Unsized { fields } => SerialCommand::Unsized {
            offset: T::acc_offset(fields, fields.len() as Field - 1),
            function: T::acc_encode_fn::<E>(fields, fields.len() as Field - 1),
        },
        SerialSize::Padding(_) => SerialCommand::Padding,
        SerialSize::Sized { start, len } => SerialCommand::Sized {
            start: *start,
            len: *len,
        },
    }
}

const fn into_decode_command<'a, T: 'a + const OffsetAccumlator, D: Decoder>(
    serial_size: &SerialSize,
) -> SerialCommand<&'a dyn UnsafeDecode<D>>
where
    [(); T::N]:,
{
    match serial_size {
        SerialSize::Unsized { fields } => SerialCommand::Unsized {
            offset: T::acc_offset(fields, fields.len() as Field - 1),
            function: T::acc_decode_fn::<D>(fields, fields.len() as Field - 1),
        },
        SerialSize::Padding(_) => SerialCommand::Padding,
        SerialSize::Sized { start, len } => SerialCommand::Sized {
            start: *start,
            len: *len,
        },
    }
}

seq!(N in 0..256 {
    pub struct FlatVec<T> {
        len: u16,
        #(v~N: MaybeUninit<T>,)*
    }

    impl<T> FlatVec<T> {
        pub const fn new() -> Self {
            Self {
                len: 0,
                #(v~N: unsafe { MaybeUninit::zeroed().assume_init() },)*
            }
        }

        pub const fn get(&self, index: usize) -> &T {
            match index {
                #(N => unsafe { self.v~N.assume_init_ref() },)*
                _ => panic!("Out of index")
            }
        }

        pub const fn push(&mut self, new_value: T) {
            match self.len {
                #(N => {
                    self.v~N = MaybeUninit::new(new_value);
                })*
                _ => unreachable!()
            };
            self.len += 1;
        }

        pub const fn len(&self) -> usize {
            self.len as usize
        }
    }
});

const fn collect_encode_functions<
    'a,
    T: 'a + const OffsetAccumlator + Sized + const SerialDescriptor,
    E: 'a + Encoder,
>() -> FlatVec<SerialCommand<&'a dyn UnsafeEncode<E>>>
where
    [(); T::N]:,
{
    let fields = const { T::fields::<E>() };
    let mut vec = FlatVec::<SerialCommand<&dyn UnsafeEncode<E>>>::new();
    let mut i = 0;
    while i < fields.len() {
        vec.push(into_encode_command::<T, E>(fields.get(i)));
        i += 1;
    }
    vec
}
const fn collect_decode_functions<
    'a,
    T: 'a + const OffsetAccumlator + Sized + const SerialDescriptor,
    D: 'a + Decoder,
>() -> FlatVec<SerialCommand<&'a dyn UnsafeDecode<D>>>
where
    [(); T::N]:,
{
    let fields = const { T::fields::<D>() };
    let mut vec = FlatVec::<SerialCommand<&dyn UnsafeDecode<D>>>::new();
    let mut i = 0;
    while i < fields.len() {
        vec.push(into_decode_command::<T, D>(fields.get(i)));
        i += 1;
    }
    vec
}

pub fn decode2<
    'a,
    T: const OffsetAccumlator + Sized + const SerialDescriptor + Decode,
    D: 'a + Decoder,
>(
    decoder: &mut D,
    place: &mut MaybeUninit<T>,
) -> Result<(), D::Error>
where
    [(); T::N]:,
{
    let commands = &const { collect_decode_functions::<T, D>() };
    let mut i = 0;
    while i < commands.len() {
        match commands.get(i) {
            SerialCommand::Unsized { offset, function } => {
                function.decode_unsafe(decoder, {
                    (&raw mut *place as *mut u8).wrapping_add(*offset)
                })?;
            }
            SerialCommand::Sized { start, len } => {
                unsafe {
                    let dst = slice::from_raw_parts_mut(
                        (&raw mut *place as *mut u8).wrapping_add(*start),
                        *len,
                    );
                    let src = decoder
                        .read_bytes(*len)
                        .map_err(|()| DecodeError::not_enough_bytes_in_the_buffer())?;
                    dst.copy_from_slice(src);
                };
            }
            SerialCommand::Padding => {}
        }
        i += 1;
    }
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
    false
}

pub const fn sized_field_of<T: SerialDescriptor>() -> ConstVec<[SerialSize; T::N]> {
    let value: [[u8; size_of::<SerialSize>()]; T::N] = [[0_u8; size_of::<SerialSize>()]; T::N];
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
        F: const SerialDescriptor,
        C: const CheckPrimitiveTypeSize,
        const FIELD: Field,
    >(
        &mut self,
        field_ptr: &F,
    ) where
        [(); T::N]:,
        [(); <F as SerialDescriptor>::N]:,
    {
        let offset = unsafe { (field_ptr as *const F).byte_sub_ptr(self.value as *const T) };
        let size = size_of::<F>();
        let mut i = offset;
        while i < offset + size {
            self.board[i] = self.counter;
            i += 1;
        }
        self.counter += 1;
        let slice = const { add_to_fields(<F as SerialDescriptor>::fields::<C>(), FIELD) };
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
