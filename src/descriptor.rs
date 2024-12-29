#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]
#![feature(specialization)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(new_range_api)]

use core::{fmt::Debug, marker::PhantomData, mem::MaybeUninit, range::Range, usize};

use const_for::const_for;
use fastbuf::{Buffer, WriteBuf, WriteBufferError};
use nonmax::NonMaxU16;

use crate::{Decode, DecodeError, Decoder, Encode, EncodeError, Encoder, PrimitiveTypeSizeChecker};

const FIELDS_AMOUNT_THRESHOLD: usize = 256;

#[const_trait]
pub trait SerialDescriptor: Sized + 'static {
    const SIZES_LEN: usize;
    fn serial_sizes<S: const PrimitiveTypeSizeChecker>() -> Buffer<[SerialSize; Self::SIZES_LEN]>;
}

impl<T: 'static> const SerialDescriptor for T {
    default const SIZES_LEN: usize = 1;

    default fn serial_sizes<S: const PrimitiveTypeSizeChecker>(
    ) -> Buffer<[SerialSize; Self::SIZES_LEN]> {
        let mut out = Buffer::new_zeroed();
        out.write(&[match S::size_of::<Self>() {
            true => SerialSize::Sized(Range {
                start: 0,
                end: size_of::<Self>(),
            }),
            false => SerialSize::Unsized {
                offset: 0,
                path: FieldPath::new_zeroed(),
            },
        }]);
        out
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SerialSize {
    Unsized { offset: usize, path: FieldPath },
    Sized(Range<usize>),
}

pub type FieldIndex = u16;
pub type FieldPath = Buffer<[FieldIndex; 64]>;

pub trait CompositableDecode<D: Decoder> {
    fn decode_in_place(&self, decoder: &mut D, out: *mut u8) -> Result<(), D::Error>;
}

pub trait CompositableEncode<E: Encoder> {
    fn encode(&self, encoder: &mut E, value: *const u8) -> Result<(), E::Error>;
}

pub struct CompositableWrapper<T>(pub PhantomData<T>);

impl<T: Decode, D: Decoder> CompositableDecode<D> for CompositableWrapper<T> {
    fn decode_in_place(&self, decoder: &mut D, out: *mut u8) -> Result<(), <D as Decoder>::Error> {
        T::decode_in_place(decoder, unsafe { &mut *(out as *mut MaybeUninit<T>) })
    }
}

impl<T: Encode, E: Encoder> CompositableEncode<E> for CompositableWrapper<T> {
    fn encode(&self, encoder: &mut E, value: *const u8) -> Result<(), <E as Encoder>::Error> {
        T::encode(unsafe { &*(value as *const T) }, encoder)
    }
}

#[const_trait]
pub trait FieldPathFinder {
    fn find_encode<'a, E: Encoder>(path: FieldPath) -> &'a dyn CompositableEncode<E>
    where
        Self: 'a;
    fn find_decode<'a, D: Decoder>(path: FieldPath) -> &'a dyn CompositableDecode<D>
    where
        Self: 'a;
    fn calc_offset(path: FieldPath) -> usize;
}

impl<T: Encode + Decode> const FieldPathFinder for T {
    default fn find_encode<'a, E: Encoder>(_path: FieldPath) -> &'a dyn CompositableEncode<E>
    where
        Self: 'a,
    {
        &CompositableWrapper::<T>(PhantomData)
    }

    default fn find_decode<'a, D: Decoder>(_path: FieldPath) -> &'a dyn CompositableDecode<D>
    where
        Self: 'a,
    {
        &CompositableWrapper::<T>(PhantomData)
    }

    default fn calc_offset(_path: FieldPath) -> usize {
        0
    }
}

pub trait FieldPathDrop: Sized {
    fn drop_fields(value: &mut MaybeUninit<Self>, fields: FieldPath);
}

impl<T: Sized> FieldPathDrop for T {
    default fn drop_fields(value: &mut MaybeUninit<Self>, _fields: FieldPath) {
        unsafe { value.assume_init_drop() }
    }
}

pub const fn offset_of<T, F>(value: &T, field: &F) -> usize {
    unsafe { (field as *const _ as *const u8).byte_sub_ptr(value) }
}

pub fn decode_struct<
    T: const SerialDescriptor + const FieldPathFinder + FieldPathDrop + Decode,
    D: Decoder,
>(
    decoder: &mut D,
    out: &mut MaybeUninit<T>,
) -> Result<(), D::Error>
where
    [(); T::SIZES_LEN]:,
{
    let commands = const { commands_of_decode::<T, D>() }.as_slice();
    let mut command_index = 0;
    let mut f = || -> Result<(), D::Error> {
        for _i in 0..commands.len() {
            match commands[command_index] {
                SerialCommand::Unsized {
                    offset,
                    path: _,
                    function,
                } => function.decode_in_place(
                    decoder,
                    (out as *mut _ as *mut u8).wrapping_byte_add(offset),
                )?,
                SerialCommand::Sized(range) => {
                    let dst = unsafe {
                        core::slice::from_raw_parts_mut(
                            (out as *mut _ as *mut u8).wrapping_add(range.start),
                            range.end,
                        )
                    };
                    let src = decoder.read(range.end);
                    if src.len() != range.end {
                        return Err(DecodeError::not_enough_bytes_in_the_buffer())?;
                    }
                    dst.copy_from_slice(src);
                }
            }
            command_index += 1;
        }
        Ok(())
    };
    match f() {
        Ok(()) => Ok(()),
        Err(err) => {
            let mut fields = FieldPath::new_zeroed();
            for i in 0..command_index {
                match commands[i] {
                    SerialCommand::Unsized {
                        offset: _,
                        path,
                        function: _,
                    } => {
                        fields.write(&[path.as_slice()[0]]);
                    }
                    _ => {}
                }
                fields.write(&[]);
            }
            T::drop_fields(out, fields);
            return Err(err);
        }
    }
}

pub fn encode_struct<
    'a,
    T: const SerialDescriptor + const FieldPathFinder + FieldPathDrop + Encode,
    E: Encoder,
>(
    value: &T,
    encoder: &mut E,
) -> Result<(), E::Error>
where
    [(); T::SIZES_LEN]:,
{
    let commands = const { commands_of_encode::<T, E>() }.as_slice();
    let mut command_index = 0;
    const_for!(_i in 0..commands.len() => {
        match commands[command_index] {
            SerialCommand::Unsized {
                offset,
                path: _,
                function,
            } => function.encode(
                encoder,
                (value as *const _ as *const u8).wrapping_add(offset),
            )?,
            SerialCommand::Sized(range) => {
                let len = range.end;
                let src = unsafe {
                    core::slice::from_raw_parts(
                        (value as *const _ as *const u8).wrapping_add(range.start),
                        len,
                    )
                };
                encoder.try_write(src)
                    .map_err(|WriteBufferError::BufferFull|
                        EncodeError::not_enough_space_in_the_buffer())?;

            }
        }
        command_index += 1;
    });
    Ok(())
}

pub const fn order_sizes_by_repr_and_calc_offset<
    T: const FieldPathFinder + const SerialDescriptor,
    C: const PrimitiveTypeSizeChecker,
    const N: usize,
>(
    sizes: &[&[SerialSize]],
) -> Buffer<[SerialSize; N]>
where
    [(); size_of::<T>()]:,
{
    let mut board = [Option::<NonMaxU16>::None; size_of::<T>()];
    const_for!(i in 0..sizes.len() as FieldIndex => {
        let mut path = FieldPath::new_zeroed();
        path.write(&[i]);
        board[T::calc_offset(path)] = NonMaxU16::new(i);
    });
    let mut ordered_result = Buffer::new_zeroed();
    const_for!(offset in 0..board.len() => {
        if let Some(index) = board[offset] {
            let sizes = sizes[index.get() as usize];
            const_for!(i in 0..sizes.len() => {
                let size = sizes[i];
                ordered_result.write(&[match size {
                    SerialSize::Unsized { offset: origin_offset, path: origin_path } => {
                        let mut path = FieldPath::new_zeroed();
                        path.write(&[index.get()]);
                        path.write(origin_path.as_slice());
                        SerialSize::Unsized { offset: origin_offset + offset, path }
                    },
                    SerialSize::Sized(Range { start: origin_start, end: origin_end }) => {
                        SerialSize::Sized(Range { start: origin_start + offset, end: origin_end } )
                    },
                }]);
            });
        }
    });
    ordered_result
}

pub const fn concatenated_neighboring_sized_of<
    T: const SerialDescriptor,
    S: const PrimitiveTypeSizeChecker,
>() -> Buffer<[SerialSize; FIELDS_AMOUNT_THRESHOLD]>
where
    [(); T::SIZES_LEN]:,
{
    let sizes = const { T::serial_sizes::<S>() };
    concat_neighboring_sized(sizes.as_slice())
}

pub const fn concat_neighboring_sized<const N: usize>(
    sizes: &[SerialSize],
) -> Buffer<[SerialSize; N]> {
    let mut buffer = Buffer::new_zeroed();
    let mut sized_acc = Range { start: 0, end: 0 };
    macro_rules! flush {
        () => {
            if sized_acc.end != 0 {
                buffer.write(&[SerialSize::Sized(sized_acc)]);
                sized_acc.end = 0;
            }
        };
    }
    const_for!(i in 0..sizes.len() => {
        let size = sizes[i];
        match size {
            SerialSize::Unsized { offset, path } => {
                flush!();
                buffer.write(&[SerialSize::Unsized { offset, path }]);
            }
            SerialSize::Sized(Range { start, end }) => {
                if sized_acc.start == 0 && sized_acc.end == 0 {
                    sized_acc.start = start;
                }
                sized_acc.end += end;
            }
        }
    });
    flush!();
    buffer
}

#[derive(Copy, Clone)]
pub enum SerialCommand<F> {
    Unsized {
        offset: usize,
        path: FieldPath,
        function: F,
    },
    Sized(Range<usize>),
}

impl<F> Debug for SerialCommand<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unsized {
                offset,
                path,
                function: _,
            } => f
                .debug_struct("Unsized")
                .field("offset", offset)
                .field("path", path)
                .finish(),
            Self::Sized(arg0) => f.debug_tuple("Sized").field(arg0).finish(),
        }
    }
}

const fn commands_of_encode<
    'a,
    T: const SerialDescriptor + const FieldPathFinder + Encode,
    E: Encoder,
>() -> Buffer<[SerialCommand<&'a dyn CompositableEncode<E>>; FIELDS_AMOUNT_THRESHOLD]>
where
    [(); T::SIZES_LEN]:,
{
    let mut buf = Buffer::new_zeroed();
    let sizes = &const { concatenated_neighboring_sized_of::<T, E>() }.as_slice();
    const_for!(i in 0..sizes.len() => {
        buf.write(&[match sizes[i] {
            SerialSize::Unsized { offset, path } => SerialCommand::Unsized { offset, path, function: T::find_encode(path) },
            SerialSize::Sized(range) => SerialCommand::Sized(range)
        }]);
    });
    buf
}

const fn commands_of_decode<
    'a,
    T: const SerialDescriptor + const FieldPathFinder + Decode,
    E: Decoder,
>() -> Buffer<[SerialCommand<&'a dyn CompositableDecode<E>>; T::SIZES_LEN]> {
    let sizes = &const { concatenated_neighboring_sized_of::<T, E>() }.as_slice();
    let mut buf = Buffer::new_zeroed();
    const_for!(i in 0..sizes.len() => {
        buf.write(&[match sizes[i] {
            SerialSize::Unsized { offset, path } => SerialCommand::Unsized {
                offset,
                path,
                function: T::find_decode(path),
            },
            SerialSize::Sized(range) => SerialCommand::Sized(range),
        }]);

    });
    buf
}

#[cfg(test)]
mod fuzz {
    // TODO
}
