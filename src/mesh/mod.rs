//! TODO try use mir for macro implementation

mod actor;
mod compound;
mod edge;
mod end;
mod field;
mod impls;
mod leaf;
mod macros;
mod order;
mod padding;
mod len;
mod size;

pub mod __private {
    pub mod typenum {
        pub use typenum::*;
    }
    pub use super::actor::*;
    pub use super::size::*;
    pub use super::compound::*;
    pub use super::edge::*;
    pub use super::end::*;
    pub use super::field::*;
    pub use super::leaf::*;
    pub use super::macros::*;
    pub use super::order::*;
    pub use super::padding::*;
    pub use super::len::*;
}

pub fn asdf(input: String) -> String {
    // serialization::mesh::field::PhantomField<Bar, u8, 0>
    //serialization::mesh::field::PhantomField<Foo,
    return input;
    let regex = Regex::new(r"serialization::mesh::field::PhantomField<[^<>]*>, ").unwrap();
    regex.replace_all(input.as_str(), "").into_owned()
}

//TODO REMOVE IT and find alternative
#[cfg(debug_assertions)]
macro_rules! trim {
    ($code:expr) => {
        $crate::asdf($code.replace(
                 "serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Model, ",
                 ""
             ).replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::macros::tests::", "").replace("serialization::mesh::order::", "").replace("serialization::mesh::edge::", "").replace("serialization::mesh::field::PhantomField<Model, ", "").replace("serialization::mesh::node::edge::PhantomEdge<Model, ", "").replace("serialization::mesh::node::end::End", "").replace("serialization::mesh::add::order::Ordering<Model, ", "").replace("serialization::mesh::end::", "").replace("serialization::mesh::padding::Padding", "").replace("typenum::uint::UTerm, ", "").replace("typenum::uint::UInt<", "").replace("typenum::bit::B1>, ", "").replace("typenum::bit::B0>, ", "").replace("serialization::mesh::field::PhantomField<Bar, ", "").replace("serialization::mesh::field::PhantomField<Foo, ", "").replace("<typenum::bit::B1>", "").replace("<typenum::bit::B0>", "").replace("<typenum::uint::UTerm>", "").replace("<Model, typenum::bit::B0>>, ", "").replace("<Foo, typenum::bit::B0>>, ", "").replace("<Bar, typenum::bit::B0>>, ", "").replace("<Model, typenum::bit::B1>>, ", "").replace("<Foo, typenum::bit::B1>>, ", "").replace("<Bar, typenum::bit::B1>>, ", "").replace("<Foo, typenum::uint::UTerm>, ", "").replace("<Bar, typenum::uint::UTerm>, ", "").replace("<Model, typenum::uint::UTerm>, ", "").replace(">)", "").replace("((", "").replace("alloc::vec::", "").replace(">, ", ", ")

        )};
}

use regex::Regex;
#[cfg(debug_assertions)]
pub(super) use trim;
