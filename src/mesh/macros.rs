#[macro_export]
macro_rules! impl_mesh {
    ($brace:ident, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
    const _: () = {
        impl<$($impl_generics,)*> $crate::__private::Edge for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type First = $crate::__private::End<$($type)+ <$($type_generics)*>>;
            type Second = $crate::meshup!(0, ($($type)+), {$($type_generics)*}; $({$($field)*})*);
        }
        impl<$($impl_generics,)*> $crate::__private::Size for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics)*>>();
        }
        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics)*>>();
        }
        impl<$($impl_generics,)* __S> $crate::__private::CompoundWrapper<__S> for $($type)+ <$($type_generics)*>
            where
                $($where_clause)*
                Self: $crate::__private::CompoundUnwrapper<__S>
        {
            type Compound = <$($type)+ <$($type_generics)*> as $crate::__private::CompoundUnwrapper<__S>>::Output;
        }

        impl<$($impl_generics,)*> $crate::Encode for $($type)+ <$($type_generics)*> where $($where_clause)* {
            fn encode<E: $crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
                Ok(())
            }
        }

        $crate::impl_field_token!();
        $crate::impl_field_offset!($brace, 0, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($field_ident),*), $($field_ident => {$($field)*}),*);
    };
    };
}

#[macro_export]
macro_rules! impl_enum_mesh {
    ($brace:ident, ($($type:tt)+), {$($type_generics:tt)*}, $variant:ident impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
        const _: () = {
            //TODO do not use PhantomData for size_of::<__EnumToken>()
            pub struct __EnumToken;
            impl __EnumToken {}
        };
    };
}

#[macro_export]
macro_rules! wrap_brace {
    (brace, $type:path, $($fields_idents:tt),*) => {
        $type { $($fields_idents),* }
    };
    (parentheses, $type:path, $($fields_idents:tt),*) => {
        $type ($($fields_idents),*)
    };
    (unit, $($fields_idents:tt),*) => {
        $type
    };
}

#[macro_export]
macro_rules! impl_field_token {
    () => {
        #[repr(transparent)]
        pub struct __FieldToken<T, const I: usize>(T);
        impl<T, const I: usize> $crate::__private::Edge for __FieldToken<T, I>
        where
            T: $crate::__private::Edge,
        {
            type First = T::First;
            type Second = T::Second;
        }
        impl<T, const I: usize> $crate::__private::Len for __FieldToken<T, I>
        where
            T: $crate::__private::Len,
        {
            const SIZE: usize = T::SIZE;
        }
        impl<T, const I: usize> $crate::__private::Size for __FieldToken<T, I>
        where
            T: $crate::__private::Size,
        {
            const SIZE: usize = T::SIZE;
        }
        impl<T, const I: usize> $crate::Encode for __FieldToken<T, I>
        where
            T: $crate::Encode,
        {
            fn encode<E: $crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                self.0.encode(encoder)
            }
        }
        impl<S, T, const I: usize> $crate::__private::CompoundWrapper<S> for __FieldToken<T, I>
        where
            T: $crate::__private::CompoundWrapper<S>,
        {
            type Compound = <T as $crate::__private::CompoundWrapper<S>>::Compound;
        }
        impl<T, const I: usize> $crate::Decode for __FieldToken<T, I>
        where
            T: $crate::Decode,
        {
            fn decode_in_place<D: $crate::Decoder>(
                decoder: &mut D,
                out: &mut core::mem::MaybeUninit<Self>,
            ) -> Result<(), D::Error> {
                T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
            }
        }
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
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), ) => {};
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}) => {
        const _: () = {
            pub const fn __offset<$($impl_generics,)*>() -> usize where $($where_clause)* {
                $crate::offset_of!($brace, $($type)+, {$($type_generics)*}, ($($fields_idents),*), $first_field_ident)
            }
            impl<$($impl_generics,)*> $crate::__private::FieldOffset for __FieldToken<$($first_field)*, $index>
                where
                    $($where_clause)*
                    [(); __offset::<$($impl_generics)*>()]:
            {
                type Offset = $crate::__private::typenum::Const<{ __offset::<$($impl_generics,)*>() }>;
            }
        };
    };
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}, $($field_ident:tt => {$($field:tt)*}),*) => {
        $crate::impl_field_offset!($brace, $index, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $first_field_ident => {$($first_field)*});
        $crate::impl_field_offset!($brace, { ($index) + 1 }, ($($type)+), {$($type_generics)*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $($field_ident => {$($field)*}),*);
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
    ($brace:ident, $type:ident, {$($type_generics:tt)*}, ($($fields_idents:tt),*), $field:tt) => {{
        use core::mem::MaybeUninit;
        let origin: MaybeUninit<$type <$($type_generics)*>> = MaybeUninit::uninit();
        #[allow(unused_variables)]
        let $crate::wrap_brace!($brace, $type, $($fields_idents),*) = unsafe { origin.assume_init_ref() };
        unsafe {
            $crate::__private::sub_ptr(
                $field as *const _ as *const u8,
                origin.assume_init_ref() as *const _ as *const u8,
            )
        }
    }};
}

#[macro_export]
macro_rules! offset_of_enum {
    ($brace:ident, $type:ident, {$($type_generics:tt)*}, $variant:ident, ($($fields_idents:tt),*), $field:tt) => {{
        use core::mem::MaybeUninit;
        unsafe {
            let origin = {
                $(let $fields_idents = MaybeUninit::zeroed().assume_init();)*
               let origin: $type <$($type_generics)*> = $crate::wrap_brace!($brace, $type::$variant, $($fields_idents),*);
               $(core::mem::forget($fields_idents);)*
                MaybeUninit::new(origin)
            };
            match origin.assume_init_ref() {
                $crate::wrap_brace!($brace, $type::$variant, $($fields_idents),*) => {
                    $crate::__private::sub_ptr(
                        $field as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
                _ => unreachable!()
            }
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
            type Second = $crate::__private::PhantomEdge<$($type)+ <$($type_generics)*>, (
                    $crate::__private::Field<$($type)+ <$($type_generics)*>>,
                    $crate::__private::End<$($type)+ <$($type_generics)*>>
                )>;
        }
        impl<$($impl_generics,)* __S> $crate::__private::CompoundWrapper<__S> for $($type)+ <$($type_generics)*> where $($where_clause)* {
            type Compound = $crate::__private::PhantomLeaf<Self>;
        }
        impl<$($impl_generics,)*> $crate::__private::Size for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics)*>>();
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
    use std::{any::type_name, hint::black_box};

    use test::Bencher;

    use crate::{
        mock::{self, Codec},
        prelude::{Edge, Mesh},
    };

    extern crate test;

    #[repr(C)]
    #[derive(serialization::Serializable)]
    struct Model {
        field0: u8, // offset 0 size 1
        // padding 3
        field1: Foo, // offset 4 size 12
        // padding 0
        field2: Vec<u8>, // offset 16 size 24
        // padding 0
        field3: u32, // offset 40 size 4
        // padding 0
        field4: Bar, // offset 44 size 2
        // padding 2
        field5: u32, // offset 48 size 4
                     // padding 4
                     // model size 56
    }

    #[repr(C)]
    #[derive(serialization::Serializable)]
    struct Foo {
        field0: u32, // offset 0  size 4
        // padding 0
        field1: u32, // offset 4 size 4
        // padding 0
        field2: Bar, //offset 8 size 2
                     // padding 2
                     // size 12
    }

    #[repr(C)]
    #[derive(serialization::Serializable)]
    struct Bar {
        field0: u8, // offset 0 size 1
        // padding 0
        field1: u8, // offset 0 size 1
                    // padding 0
    }

    fn model() -> Model {
        Model {
            field0: 11,
            field1: Foo {
                field0: 22,
                field1: 33,
                field2: Bar {
                    field0: 44,
                    field1: 55,
                },
            },
            field2: vec![1, 2, 3, 4],
            field3: 66,
            field4: Bar {
                field0: 77,
                field1: 88,
            },
            field5: 99,
        }
    }

    #[test]
    fn actor() {
        println!("{}", type_name::<<Model as Mesh<Codec<*mut u8>>>::Output>());
        #[allow(invalid_value)]
        let mut dst = [0u8; 10000];
        println!("--------");
        mock::encode(&model(), &mut dst).unwrap();
        println!("{:?}", &dst[..66]);
        black_box(&dst);
        println!("--------");
    }

    #[cfg(not(debug_assertions))]
    #[bench]
    fn bench_encode(b: &mut Bencher) {
        let model = &model();
        let mut dst = [0u8; 10000];
        b.iter(|| mock::encode(model, &mut dst));
        println!("{:?}", &dst[..66]);
        black_box(&dst);
    }
}
