#[macro_export]
macro_rules! impl_meshup {
    (($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
    const _: () = {
        struct __FieldToken<T, const I: usize>(core::marker::PhantomData<T>);
        impl<T, const I: usize> $crate::__private::Edge for __FieldToken<T, I> where T: $crate::__private::Edge {
            type First = T::First;
            type Second = T::Second;
        }
        impl<T, const I: usize> $crate::__private::Len for __FieldToken<T, I> where T: $crate::__private::Len {
            const SIZE: usize = T::SIZE;
        }
        impl<S, T, const I: usize> $crate::__private::FieldUnwrapper for __FieldToken<$crate::__private::Compound<S, T>, I> {
            type Output = $crate::__private::Compound<S, T>;
        }
        impl<T, const I: usize> $crate::__private::FieldUnwrapper for __FieldToken<$crate::__private::PhantomLeaf<T>, I> {
            type Output = $crate::__private::PhantomLeaf<__FieldToken<T, I>>;
        }

        impl<$($impl_generics,)*> $crate::__private::Edge for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type First = $crate::__private::End<$($type)+ <$($type_generics)*>>;
            type Second = $crate::meshup!(0, ($($type)+), {$($type_generics)*}; $({$($field)*})*);
        }
        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics)*>>();
        }
        impl<$($impl_generics,)* __S> $crate::__private::CompoundWrapper<__S> for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type Compound = $crate::__private::Compound<__S, <$($type)+ <$($type_generics)*> as $crate::__private::Edge>::Second>;
        }

        $crate::impl_field_offset!(0, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); $($field_ident => {$($field)*}),*);
    };
    };
}

#[macro_export]
macro_rules! meshup {
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*};) => { $crate::__private::End<$($type)+ <$($type_generics)*>> };
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*}; {$($first:tt)*} $({$($field:tt)*})*) => {
        <<$crate::meshup!({ ($index) + 1 }, ($($type)+), {$($type_generics)*}; $({$($field)*})*)
            as core::ops::Add<$crate::__private::Padding<$($type)+ <$($type_generics)*>, $crate::__private::Field<__FieldToken<$($first)*, $index>>>>>::Output
            as core::ops::Add<$crate::__private::Field<__FieldToken<$($first)*, $index>>>>::Output
    };
}

#[macro_export]
macro_rules! impl_field_offset {
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ) => {};
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $first_field_ident:tt => {$($first_field:tt)*}) => {
        const _: () = {
            const fn __offset<$($impl_generics,)*>() -> usize where $($where_clause)* {
                $crate::offset_of!(($($type)+), {$($type_generics)*}, $first_field_ident)
            }
            impl<$($impl_generics,)*> $crate::__private::FieldOffset for __FieldToken<$($first_field)*, $index>
                where
                    $($where_clause)*
                    [(); __offset::<$($impl_generics)*>()]:
            {
                type Offset = $crate::__private::typenum::Const<{ __offset::<$($impl_generics,)*>() }>;
            }
            impl<$($impl_generics,)* __S> $crate::__private::CompoundWrapper<__S> for __FieldToken<$($first_field)*, $index>
                where
                    $($where_clause)*
                    $($first_field)*: $crate::__private::CompoundWrapper<__S>,
                    __FieldToken<<$($first_field)* as $crate::__private::CompoundWrapper<__S>>::Compound, $index>: $crate::__private::FieldUnwrapper
            {
                type Compound = <__FieldToken<<$($first_field)* as $crate::__private::CompoundWrapper<__S>>::Compound, $index> as $crate::__private::FieldUnwrapper>::Output;
            }
        };
    };
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $first_field_ident:tt => {$($first_field:tt)*}, $($field_ident:tt => {$($field:tt)*}),*) => {
        $crate::impl_field_offset!($index, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); $first_field_ident => {$($first_field)*});
        $crate::impl_field_offset!({ ($index) + 1 }, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); $($field_ident => {$($field)*}),*);
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
    (($($type:tt)+), {$($type_generics:tt)*}, $field:tt) => {{
        use core::mem::MaybeUninit;
        let origin: MaybeUninit<$($type)+ <$($type_generics)*>> = MaybeUninit::uninit();
        unsafe {
            $crate::__private::sub_ptr(
                &origin.assume_init_ref().$field as *const _ as *const u8,
                origin.assume_init_ref() as *const _ as *const u8,
            )
        }
    }};
}

macro_rules! impl_serializable {
    (($($type:tt)+), {$($type_generics:tt)*}) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics)*}, impl {} $($where_clause)*);
    };
    (($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        impl<$($impl_generics,)*> $crate::__private::Edge for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type First = $crate::__private::End<$($type)+ <$($type_generics)*>>;
            type Second = $crate::__private::End<$($type)+ <$($type_generics)*>>;
        }
        impl<$($impl_generics,)* __S> $crate::__private::CompoundWrapper<__S> for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type Compound = $crate::__private::PhantomLeaf<Self>;
        }
    };
}

macro_rules! impl_primitives {
    (($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*));

        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics)*>>();
        }
    };
    (($($type:tt)+)) => {
        $crate::__private::impl_primitives!(($($type)+), {}, impl {} ());
    };
}

macro_rules! impl_non_primitives {
    (($($type:tt)+)) => {
        $crate::__private::impl_non_primitives!(($($type)+), {}, impl {} ());
    };
    (($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*));

        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics)*> where $($where_clause)* {
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
    use std::any::type_name;

    use serialization_derive::Serializable;

    use crate::__private::{Flatten, Sorted};

    extern crate test;

    // #[repr(C)]
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

    #[test]
    fn actor() {
        type T = <<<Model as crate::__private::Edge>::Second as Sorted>::Output as Flatten>::Output;
        for i in 0..1000 {
            <T as crate::__private::Actor>::run_at(i);
        }
    }

    #[cfg(not(debug_assertions))]
    #[bench]
    fn bench_must_0ns(b: &mut test::Bencher) {
        for i in 0..1000 {
            <<Model as crate::__private::Edge>::Second as crate::__private::Actor>::run_at(i);
        }
    }
}
