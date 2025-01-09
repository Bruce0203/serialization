pub const unsafe fn sub_ptr<T>(field: *const T, origin: *const T) -> usize {
    unsafe { field.byte_sub_ptr(origin) }
}

#[macro_export]
macro_rules! offset_of {
    ($T:ty, $field:ident) => {{
        use core::mem::MaybeUninit;
        let origin: MaybeUninit<$T> = MaybeUninit::uninit();
        unsafe {
            $crate::sub_ptr(
                &origin.assume_init_ref().$field as *const _ as *const u8,
                origin.assume_init_ref() as *const _ as *const u8,
            )
        }
    }};
}
#[macro_export]
macro_rules! impl_meshup {
    ($type:ty; $($field_ident:ident: $field:ty),*) => {
        impl $crate::Edge for $type {
            type Second = <$crate::Compound<$type, $crate::meshup!(0, $type; $($field,)*)> as core::ops::Add<!>>::Output;
            // type Second = $crate::Compound<$type, $crate::meshup!(0, $type; $($field,)*)>;
        }
        impl<S, const I: usize> $crate::CompoundWrapper<S> for $crate::PhantomField<S, $type, I> {
            type Compound = $crate::Compound<S, $crate::PhantomEdge<S, (<Self as $crate::Edge>::First, <Self as $crate::Edge>::Second)>>;
            // type Compound = $crate::PhantomLeaf<S, Self>;
        }
        impl<S> $crate::FieldOffset<S> for $type {
            type Offset = typenum::U0;
        }
        $crate::impl_field_offset!(0, $type; $($field_ident: $field),*);
    };
}

#[macro_export]
macro_rules! meshup {
    ($index:expr, $type:ty;) => { ! };
    ($index:expr, $type:ty; $first:ty, $($field:ty,)*) => {
        <$crate::PhantomOrder<$type, $crate::meshup!({ ($index) + 1 }, $type; $($field,)*)> as core::ops::Add<
            $crate::PhantomField<$type, $first, $index>
        >>::Output
    };
}

#[macro_export]
macro_rules! impl_field_offset {
    ($index:expr, $type:ty; ) => {
    };
    ($index:expr, $type:ty; $first_field_ident:ident: $first_field:ty) => {
        impl $crate::FieldOffset<$type> for $crate::PhantomField<$type, $first_field, $index> {
            type Offset = typenum::U< { $crate::offset_of!($type, $first_field_ident)} >;
        }
    };
    ($index:expr, $type:ty; $first_field_ident:ident: $first_field:ty, $($field_ident:ident: $field:ty),*) => {
        $crate::impl_field_offset!($index, $type; $first_field_ident: $first_field);
        $crate::impl_field_offset!({ ($index) + 1 }, $type; $($field_ident: $field),*);
    };
}

#[macro_export]
macro_rules! count_items {
    () => { 0 };
    ($head:expr, $($tail:expr,)*) => {
        1 + count_items!($($tail,)*)
    };
}

#[cfg(feature = "non")]
#[cfg(test)]
mod tests2 {
    use std::{convert::Infallible, ops::Add};

    use crate::{Compound, Edge, End, PhantomEdge};

    extern crate test;
    #[test]
    fn test() {
        type T = <PhantomEdge<Infallible, u8> as Add<PhantomEdge<Infallible, End>>>::Output;
        type T2 = <PhantomEdge<Infallible, u32> as Add<T>>::Output;
        type T3 = <PhantomEdge<Infallible, u32> as Add<T2>>::Output;
        pub struct A;
        impl Edge for A {
            type First = ();

            type Second = T3;
        }

        type T4 = <Compound<
            Infallible,
            PhantomEdge<Infallible, (<A as Edge>::First, <A as Edge>::Second)>,
        > as Add<PhantomEdge<Infallible, u8>>>::Output;
        println!(
            "{}",
            std::any::type_name::<T4>().replace(
                "serialization::mesh::edge::PhantomEdge<core::convert::Infallible, ",
                ""
            )
        );
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use std::any::type_name;

    use test::Bencher;
    use typenum::{ToUInt, Unsigned};

    use crate::{Edge, FieldOffset, mesh::actor::Actor, trim};

    struct Model {
        field0: u8,
        field1: Foo,
        field2: Vec<u8>,
        field3: u32,
        field4: Bar,
        field5: u32,
    }
    impl_meshup!(Model; field0: u8, field1: Foo, field2: Vec<u8>, field3: u32, field4: Bar, field5: u32);

    struct Foo {
        field0: u32,
        field1: u32,
        field2: Bar,
    }
    impl_meshup!(Foo; field0: u32, field1: u32, field2: Bar);

    struct Bar {
        field0: u8,
        field1: u8,
    }
    impl_meshup!(Bar; field0: u8, field1: u8);

    #[test]
    fn actor() {
        <Model as Actor>::run_at(20);
    }

    #[test]
    fn sandbox() {
        //2 1 3 4 0
        //Vec Foo u32 Bar u8
        println!(
            "{}",
            <<crate::PhantomField::<Model, u8, 0> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        println!(
            "{}",
            <<crate::PhantomField::<Model, Foo, 1> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        println!(
            "{}",
            <<crate::PhantomField::<Model, Vec<u8>, 2> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        println!(
            "{}",
            <<crate::PhantomField::<Model, u32, 3> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        println!(
            "{}",
            <<crate::PhantomField::<Model, Bar, 4> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        println!(
            "{}",
            <<crate::PhantomField::<Model, u32, 5> as FieldOffset<Model>>::Offset as Unsigned>::to_usize()
        );
        // 0  2
        // 24 1
        // 36 3
        // 40 5
        // 44 4
        // 46 0

        // 0  2
        // 24 1
        // 36 3
        // 40 5
        // 44 4
        // 46 0

        println!("{}", trim!(type_name::<<Model as Edge>::Second>()));
        // println!(
        //     "{}",
        //     trim!(type_name::<<Model as CompoundWrapper<Model>>::Compound>())
        // );
    }

    #[bench]
    #[ignore]
    fn bench_actor(b: &mut Bencher) {
        // <Model as Finder>::run_at(10);
    }

    // (u8, serialization::mesh::macros::tests::Foo>)>)>, alloc::vec::Vec<u8>, u32>)>)>, u8, u8>)>)>, u32>)>)>)>)
}

// test mesh::macros::tests::sandbox ... ((((alloc::vec::Vec<u8>, 2>, (u32, 3>, u32, 5>>)>)>, Foo, 1>)>, u8, 0>)>, Bar, 4>)>
