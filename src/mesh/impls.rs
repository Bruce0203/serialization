use super::macros::{impl_non_primitives, impl_primitives};

impl_primitives!((u8));
impl_primitives!((u32));
impl_primitives!((i32));
impl_primitives!((i16));
impl_primitives!((u16));
impl_primitives!((()));
impl_non_primitives!((Vec), {<T>}, impl {T} ());
impl_non_primitives!((String));
impl_non_primitives!((&'a str), {}, impl {'a} (where 'a: 'static));
