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
        pub struct __FieldToken<S, T, const I: usize>(T, core::marker::PhantomData<S>);
        impl<S, T, const I: usize> $crate::__private::Edge for __FieldToken<S, T, I>
        where
            T: $crate::__private::Edge,
        {
            type First = T::First;
            type Second = T::Second;
        }
        impl<S, T, const I: usize> $crate::__private::Len for __FieldToken<S, T, I>
        where
            T: $crate::__private::Len,
        {
            const SIZE: usize = T::SIZE;
        }
        impl<S, T, const I: usize> $crate::__private::Size for __FieldToken<S, T, I>
        where
            T: $crate::__private::Size,
        {
            const SIZE: usize = T::SIZE;
        }
        impl<S, T, const I: usize> $crate::Encode for __FieldToken<S, T, I>
        where
            T: $crate::Encode,
        {
            fn encode<E: $crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                self.0.encode(encoder)
            }
        }
        impl<S, S2, T, const I: usize> $crate::__private::CompoundWrapper<S>
            for __FieldToken<S2, T, I>
        where
            T: $crate::__private::CompoundWrapper<S>,
        {
            type Compound = <T as $crate::__private::CompoundWrapper<S>>::Compound;
        }
        impl<S, T, const I: usize> $crate::Decode for __FieldToken<S, T, I>
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

        impl<S, T, const I: usize> $crate::__private::Vector for __FieldToken<S, T, I>
        where
            T: $crate::__private::Vector,
        {
            type Item = T::Item;

            fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
                self.0.as_iter()
            }

            fn as_ptr(&self) -> *const Self::Item {
                self.0.as_ptr()
            }

            fn len(&self) -> usize {
                self.0.len()
            }
        }
    };
}

#[macro_export]
macro_rules! meshup {
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*};) => { $crate::__private::End<$($type)+ <$($type_generics)*>> };
    ($index:expr, ($($type:tt)+), {$($type_generics:tt)*}; {$($first:tt)*} $({$($field:tt)*})*) => {
        <<$crate::meshup!({ ($index) + 1 }, ($($type)+), {$($type_generics)*}; $({$($field)*})*)
            as core::ops::Add<$crate::__private::Padding<$($type)+ <$($type_generics)*>, $crate::__private::Field<__FieldToken<$($type)+ <$($type_generics)*>, $($first)*, $index>>>>>::Output
            as core::ops::Add<$crate::__private::Field<__FieldToken<$($type)+ <$($type_generics)*>, $($first)*, $index>>>>::Output
    };
}

#[macro_export]
macro_rules! impl_field_offset {
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), ) => {};
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt)*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}) => {
        const _: () = {
            impl<$($impl_generics,)*> $crate::__private::FieldOffset for __FieldToken<$($type)+ <$($type_generics)*>, $($first_field)*, $index>
                where
                    $($where_clause)*
                    [(); __field_offset::<$($impl_generics,)*>()]:
            {
                type Offset = $crate::__private::typenum::Const<{ __field_offset::<$($impl_generics,)*>() }>;
            }
            pub const fn __field_offset<$($impl_generics,)*>() -> usize where $($where_clause)* {
                $crate::offset_of!($brace, $($type)+, {$($type_generics)*}, ($($fields_idents),*), $first_field_ident)
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
