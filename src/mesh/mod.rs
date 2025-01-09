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
mod size;

pub mod __private {
    pub use super::actor::*;
    pub use super::compound::*;
    pub use super::edge::*;
    pub use super::end::*;
    pub use super::field::*;
    pub use super::leaf::*;
    pub use super::macros::*;
    pub use super::order::*;
    pub use super::size::*;
}

//TODO REMOVE IT and find alternative
#[cfg(debug_assertions)]
macro_rules! trim {
    ($code:expr) => {
        $code.replace(
                 "serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Model, ",
                 ""
             ).replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::macros::tests::", "").replace("serialization::mesh::order::", "").replace("serialization::mesh::edge::", "").replace("serialization::mesh::field::PhantomField<Model, ", "").replace("serialization::mesh::node::edge::PhantomEdge<Model, ", "").replace("serialization::mesh::node::end::End", "").replace("serialization::mesh::add::order::Ordering<Model, ", "").replace("serialization::mesh::end::", "")

    };
}

#[cfg(debug_assertions)]
pub(super) use trim;
