mod actor;
mod edge;
mod impls;
mod macros;
mod order;
// mod size;

pub use actor::*;
pub use edge::*;
pub use macros::*;
pub use order::*;
// pub use size::*;
//
//

#[macro_export]
macro_rules! trim {
    ($code:expr) => {
        $code.replace(
                 "serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Model, ",
                 ""
             ).replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Bar, ", "")

    };
}
