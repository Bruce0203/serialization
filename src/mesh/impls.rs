use std::marker::PhantomData;

use super::macros::{impl_non_primitives, impl_primitives};

macro_rules! primitives {
    ($($type:tt),*) => {
        $(impl_primitives!(($type));)*
    };
}
type UnitType = ();
primitives!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64, bool, UnitType
);
//impl_non_primitives!((String), {}, impl {} ());
impl_primitives!((PhantomData), {T}, impl {T,} ());
impl_non_primitives!((&'a str), {}, impl {'a,} ('a: 'static));
