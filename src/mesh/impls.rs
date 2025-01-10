use super::macros::{impl_non_primitives, impl_primitives};

impl_primitives!(u8);
impl_primitives!(u32);
impl_primitives!(());
impl_non_primitives!(Vec<u8>);
