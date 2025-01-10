use super::macros::{impl_non_primitives, impl_primitives};

impl_primitives!(u8, u32, ());
impl_non_primitives!(Vec<u8>);
