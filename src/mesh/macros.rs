#[macro_export]
macro_rules! impl_meshup {
    ($type:ty; $($field_ident:tt => $field:ty),*) => {
        impl $crate::__private::Edge for $type {
            type First = $crate::__private::End<$type>;
            type Second = <$crate::meshup!(0, $type; $($field,)*) as $crate::__private::Flatten>::Output;
        }
        impl<S, const I: usize> $crate::__private::CompoundWrapper<S> for $crate::__private::PhantomField<S, $type, I> {
            type Compound = $crate::__private::Compound<S, <$type as $crate::__private::Edge>::Second>;
        }

        impl<S, const I: usize> $crate::__private::Len for $crate::__private::PhantomField<S, $type, I> {
            const SIZE: usize = core::mem::size_of::<$type>();
        }
        $crate::impl_field_offset!(0, $type; $($field_ident => $field),*);
    };
}

#[macro_export]
macro_rules! meshup {
    ($index:expr, $type:ty;) => { $crate::__private::End<$type> };
    ($index:expr, $type:ty; $first:ty, $($field:ty,)*) => {
        <$crate::__private::PhantomOrder<$type, <$crate::__private::PhantomOrder<$type, $crate::meshup!({ ($index) + 1 }, $type; $($field,)*)>
            as core::ops::Add<$crate::__private::Padding<$type, $crate::__private::PhantomField<$type, $first, $index>>>>::Output>
            as core::ops::Add<$crate::__private::PhantomField<$type, $first, $index>>>::Output
    };
}

#[macro_export]
macro_rules! field_of {
    ($index:expr, $type:ty; $first:ty, $($field:ty,)*) => {};
}

#[macro_export]
macro_rules! impl_field_offset {
    ($index:expr, $type:ty; ) => {};
    ($index:expr, $type:ty; $first_field_ident:tt => $first_field:ty) => {
        impl $crate::__private::FieldOffset for $crate::__private::PhantomField<$type, $first_field, $index> {
            type Offset = $crate::__private::typenum::U< { $crate::offset_of!($type, $first_field_ident) } >;
        }
    };
    ($index:expr, $type:ty; $first_field_ident:tt => $first_field:ty, $($field_ident:tt => $field:ty),*) => {
        $crate::impl_field_offset!($index, $type; $first_field_ident => $first_field);
        $crate::impl_field_offset!({ ($index) + 1 }, $type; $($field_ident => $field),*);
    };
}

#[macro_export]
macro_rules! count_items {
    () => { 0 };
    ($head:expr, $($tail:expr,)*) => {
        1 + count_items!($($tail,)*)
    };
}

#[macro_export]
macro_rules! offset_of {
    ($T:ty, $field:tt) => {{
        use core::mem::MaybeUninit;
        let origin: MaybeUninit<$T> = MaybeUninit::uninit();
        unsafe {
            $crate::__private::sub_ptr(
                &origin.assume_init_ref().$field as *const _ as *const u8,
                origin.assume_init_ref() as *const _ as *const u8,
            )
        }
    }};
}

macro_rules! impl_serializable {
    ($type:ty) => {
        impl $crate::__private::Edge for $type {
            type First = ();
            type Second = ();
        }

        impl<S, const I: usize> $crate::__private::CompoundWrapper<S>
            for $crate::__private::PhantomField<S, $type, I>
        {
            type Compound = $crate::__private::PhantomLeaf<S, Self>;
        }
    };
}

macro_rules! impl_primitives {
    ($type:ty) => {
        $crate::__private::impl_serializable!($type);

        impl<S, const I: usize> $crate::__private::Len
            for $crate::__private::PhantomField<S, $type, I>
        {
            const SIZE: usize = core::mem::size_of::<$type>();
        }
    };
}

macro_rules! impl_non_primitives {
    ($type:ty) => {
        $crate::__private::impl_serializable!($type);

        impl<S, const I: usize> $crate::__private::Len
            for $crate::__private::PhantomField<S, $type, I>
        {
            const SIZE: usize = $crate::__private::UNSIZED;
        }
    };
}

pub(crate) use {impl_non_primitives, impl_primitives, impl_serializable};

/// Replacement for `feature = "ptr_sub_ptr"` which isn't yet stable.
pub const unsafe fn sub_ptr<T>(field: *const T, origin: *const T) -> usize {
    unsafe { field.offset_from(origin) as usize }
}

#[cfg(test)]
mod tests {
    use serialization_derive::Serializable;

    extern crate test;

    #[repr(C)]
    #[derive(Serializable)]
    struct Model {
        field0: u8,
        // padding 3
        field1: Foo,
        // padding 8
        field2: Vec<u8>,
        // padding 6
        field3: u32,
        // padding 0
        field4: Bar,
        // padding 2
        field5: u32,
        // padding 4
    }

    #[derive(Serializable)]
    struct Foo {
        field0: u32,
        // padding 0
        field1: u32,
        // padding 0
        field2: Bar,
        // padding 2
    }

    #[derive(Serializable)]
    struct Bar {
        field0: u8,
        // padding 0
        field1: u8,
        // padding 0
    }

    #[derive(Serializable)]
    struct Bar2(u8, u32);

    #[test]
    fn actor() {
        for i in 0..100 {
            <<Model as crate::__private::Edge>::Second as crate::__private::Actor>::run_at(i);
        }
    }

    #[cfg(not(debug_assertions))]
    #[bench]
    fn bench_must_0ns(b: &mut test::Bencher) {
        <<Model as crate::__private::Edge>::Second as crate::__private::Actor>::run_at(10);
    }
}