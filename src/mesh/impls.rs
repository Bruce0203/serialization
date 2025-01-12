use std::marker::PhantomData;

use super::macros::{impl_non_primitives, impl_primitives};

impl_primitives!((u8));
impl_primitives!((u32));
impl_primitives!((i32));
impl_primitives!((i16));
impl_primitives!((u16));
type UnitType = ();
impl_primitives!((UnitType));
impl_primitives!((PhantomData), {T}, impl {T,} ());
impl_non_primitives!((Vec), {T}, impl {T,} ());
impl_non_primitives!((String));
impl_non_primitives!((&'a str), {}, impl {'a,} ('a: 'static));
