mod actor;
mod add;
mod field;
mod impls;
mod macros;
mod node;
// mod size;

pub use actor::*;
pub use add::*;
pub use field::*;
pub use macros::*;
pub use node::*;
// pub use size::*;
//
//

#[macro_export]
macro_rules! trim {
    ($code:expr) => {
        $code.replace(
                 "serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Model, ",
                 ""
             ).replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::edge::Compound<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::edge::PhantomEdge<serialization::mesh::macros::tests::Bar, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Model, ", "").replace("serialization::mesh::order::PhantomField<serialization::mesh::macros::tests::Foo, ", "").replace("serialization::mesh::macros::tests::", "").replace("serialization::mesh::order::", "").replace("serialization::mesh::edge::", "").replace("serialization::mesh::field::PhantomField<Model, ", "").replace("serialization::mesh::node::edge::PhantomEdge<Model, ", "").replace("serialization::mesh::node::end::End", "")

    };
}
