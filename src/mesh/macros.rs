#[macro_export]
macro_rules! impl_mesh {
    ({$($type_generics_without_lt:tt),*}, $brace:ident, ($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
        impl<$($impl_generics,)* __C> $crate::__private::Edge<__C> for $($type)+ <$($type_generics),*>
            where
                $($where_clause)*
                $($type_generics_without_lt: $crate::__private::Edge<__C>),*
        {
            type First = $crate::__private::End<__C, $($type)+ <$($type_generics),*>>;
            type Second = $crate::meshup!(0, ($($type)+), {$($type_generics),*}; $({$($field)*})*);
        }
        impl<$($impl_generics,)*> $crate::__private::Size for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
        }
        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
        }
        impl<$($impl_generics,)* __C, __S> $crate::__private::CompoundWrapper<__C, __S> for $($type)+ <$($type_generics),*>
            where
                $($where_clause)*
                Self: $crate::__private::CompoundUnwrapper<__C, __S>
        {
            type Compound = <$($type)+ <$($type_generics),*> as $crate::__private::CompoundUnwrapper<__C, __S>>::Output;
        }

        impl<$($impl_generics,)*> $crate::Encode for $($type)+ <$($type_generics),*> where $($where_clause)* {
            fn encode<E: $crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
                Ok(())
            }
        }

        $crate::impl_field_token!();
        $crate::impl_field_offset!($brace, 0, ($($type)+), {$($type_generics),*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($field_ident),*), $($field_ident => {$($field)*}),*);
    };
}

#[macro_export]
macro_rules! impl_enum_mesh {
    ({$($type_generics_without_lt:tt),*}, ($($type:tt)+), {$($type_generics:tt),*}, ($($variants:ident),*), ($($variant_indices:expr),*), ($($discriminants:expr),*), ($($braces:ident),*), impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
        impl<$($impl_generics,)*> $crate::__private::EnumDiscriminantDecoder<$($type)+ <$($type_generics),*>> for $($type)+ <$($type_generics),*> where $($where_clause)* {
            fn decode_enum_discriminant(variant_index: &$crate::EnumVariantIndex, out: &mut core::mem::MaybeUninit<$($type)+ <$($type_generics),*>>) {
                match variant_index.0 {
                    $($variant_indices => {
                        __VariantToken2::<$variant_indices>::decode_enum_discriminant(variant_index, out);
                    })*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }
        impl<$($impl_generics),*> core::convert::Into<$crate::EnumVariantStringId> for &$($type)+ <$($type_generics),*> where $($where_clause)* {
            fn into(self) -> $crate::EnumVariantStringId {
                use $($type)+::*;
                match self {
                    $($crate::wrap_brace!($braces, ($variants), ..) => $crate::EnumVariantStringId(stringify!($variants)),)*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }

        impl<$($impl_generics,)*> core::convert::Into<$crate::EnumVariantDiscriminantId<$($type)+ <$($type_generics),*>>> for &$($type)+ <$($type_generics),*>
            where
                $($where_clause)*
                [(); core::mem::size_of::<core::mem::Discriminant<$($type)+ <$($type_generics),*>>>()]:
        {
            fn into(self) -> $crate::EnumVariantDiscriminantId<$($type)+ <$($type_generics),*>> {
                $crate::EnumVariantDiscriminantId::new(self)
            }
        }

 pub struct __VariantToken<$($impl_generics,)* const I: usize>(core::marker::PhantomData<($($type_generics),*)>) where $($where_clause)*;
 pub struct __VariantToken2<const I: usize>;
pub struct __Variants<$($impl_generics,)*>(core::marker::PhantomData<($($type_generics,)*)>) where $($where_clause)*;

        impl<$($impl_generics),*> $crate::EnumIdentifierToVariantIndex<$crate::EnumVariantStringId> for $($type)+ <$($type_generics),*>
        where
            $($where_clause)*
            [(); core::mem::size_of::<core::mem::Discriminant<Self>>()]:
        {
            fn enum_variant_index_by_identifier(id: $crate::EnumVariantStringId) -> Result<$crate::EnumVariantIndex, $crate::EnumIdentifierToVariantIndexError> {
                match id.0 {
                    $(stringify!($variants) => Ok($crate::EnumVariantIndex($variant_indices)),)*
                    _ => Err($crate::EnumIdentifierToVariantIndexError::InvalidIdentifier)
                }
            }
        }

        impl<$($impl_generics),*> $crate::EnumIdentifierToVariantIndex<$crate::EnumVariantDiscriminantId<$($type)+ <$($type_generics),*>>> for $($type)+ <$($type_generics),*>
        where
            $($where_clause)*
            [(); core::mem::size_of::<core::mem::Discriminant<Self>>()]:
        {
            fn enum_variant_index_by_identifier(id: $crate::EnumVariantDiscriminantId<$($type)+ <$($type_generics),*>>) -> Result<$crate::EnumVariantIndex, $crate::EnumIdentifierToVariantIndexError> {

                         $(
                         #[allow(unused_variables, non_snake_case)]
                         let $variants = unsafe { core::mem::transmute_copy::<_, [u8; core::mem::size_of::<core::mem::Discriminant<Self>>()]>(&($discriminants)) };
                         )*
                         match unsafe { core::mem::transmute::<_, [u8; core::mem::size_of::<core::mem::Discriminant<Self>>()]>(id) } {
                             $(#[allow(unused_variables, non_snake_case)] $variants => Ok($crate::EnumVariantIndex($variant_indices)),)*
                                 _ => Err($crate::EnumIdentifierToVariantIndexError::InvalidIdentifier)

                         }
            }
        }

        impl<$($impl_generics,)* __C> $crate::__private::Edge<__C> for __Variants<$($type_generics),*>
            where
                $($where_clause)*
                $($type_generics_without_lt: $crate::__private::Edge<__C>),*
        {
            type First = $crate::__private::End<__C, Self>;
            type Second = $crate::variant_meshup!(0, (__VariantToken), {$($type_generics,)*}; $({$variants})*);
        }

        impl<$($impl_generics,)* const I: usize> $crate::__private::Len for __VariantToken<$($type_generics,)* I>
        {
            const SIZE: usize = <$($type)+ <$($type_generics),*> as $crate::__private::Len>::SIZE;
        }
        impl<$($impl_generics,)* const I: usize> $crate::__private::Size for __VariantToken<$($type_generics,)* I>
        {
            const SIZE: usize = <$($type)+ <$($type_generics),*> as $crate::__private::Size>::SIZE;
        }
        $crate::impl_field_token!();

        impl<$($impl_generics,)* __C> $crate::__private::Edge<__C> for $($type)+ <$($type_generics),*>
            where
                $($where_clause)*
                $($type_generics_without_lt: $crate::__private::Edge<__C>),*
        {
            type First = $crate::__private::End<__C, $($type)+ <$($type_generics),*>>;
            type Second = $crate::__private::PhantomEdge<__C, Self, ($crate::__private::Enum<$($type)+ <$($type_generics),*>, __Variants<$($type_generics),*>>, $crate::__private::End<__C, Self>)>;
        }
        impl<$($impl_generics,)*> $crate::__private::Size for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
        }
        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
        }
        impl<$($impl_generics,)* __C, __S> $crate::__private::CompoundWrapper<__C, __S> for $($type)+ <$($type_generics),*>
            where
                $($where_clause)*
                Self: $crate::__private::CompoundUnwrapper<__C, __S>
        {
            type Compound = <$($type)+ <$($type_generics),*> as $crate::__private::CompoundUnwrapper<__C, __S>>::Output;
        }

        impl<$($impl_generics,)*> $crate::Encode for $($type)+ <$($type_generics),*> where $($where_clause)* {
            fn encode<E: $crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_variant_mesh {
    ({$($type_generics_without_lt:tt),*}, $brace:ident, ($($type:tt)+), {$($type_generics:tt),*}, $variant:ident, $variant_index:expr, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); $($field_ident:tt => {$($field:tt)*}),*) => {
        impl<$($impl_generics),*> $crate::__private::EnumDiscriminantDecoder<$($type)+ <$($type_generics),*>> for  __VariantToken2<$variant_index>
                where
                    $($where_clause)*
        {
            fn decode_enum_discriminant(variant_index: &$crate::EnumVariantIndex, out: &mut core::mem::MaybeUninit<$($type)+ <$($type_generics),*>>) {
        unsafe {
                $(let $field_ident = core::mem::MaybeUninit::uninit().assume_init();)*
                *out.assume_init_mut() = $crate::wrap_brace!($brace, ($($type)+::$variant), $($field_ident),*);
            }
        }
        }
            impl<$($impl_generics,)* __C> $crate::__private::Edge<__C> for __VariantToken<$($type_generics,)* $variant_index>
                where
                    $($where_clause)*
                    $($type_generics_without_lt: $crate::__private::Edge<__C>),*
            {
                type First = $crate::__private::End<__C, ()>;
                type Second = $crate::__private::PhantomEdge<__C, Self, ($crate::__private::Padding<__C, Self, $crate::__private::FrontOffsetToken>, $crate::meshup!(0, (__VariantToken), {$($type_generics,)* $variant_index}; $({$($field)*})*))>;
            }

        $crate::impl_enum_field_offset!($brace, 0, ($($type)+), {$($type_generics),*}, $variant, $variant_index, impl {$($impl_generics,)*} ($($where_clause)*); ($($field_ident),*), $($field_ident => {$($field)*}),*);
                };
}

#[macro_export]
macro_rules! impl_enum_field_offset {
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, $variant:ident, $variant_index:expr, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), ) => {};
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, $variant:ident, $variant_index:expr, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}) => {
        const _: () = {
impl<$($impl_generics,)*> $crate::__private::FieldOffset for __FieldToken<__VariantToken<$($impl_generics,)* $variant_index>, $($first_field)*, $index>

where
                    $($where_clause)*
                    [(); __field_offset::<$($impl_generics,)*>()]:

        {
        type Offset = $crate::__private::typenum::Const<{ __field_offset::<$($impl_generics,)*>() }>;
    }
pub const fn __field_offset<$($impl_generics,)*>() -> usize where $($where_clause)* {
                 $crate::offset_of_enum!($brace, $($type)+, {$($type_generics),*}, $variant, ($($fields_idents),*), $first_field_ident)
            }

                                };
    };
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, $variant:ident, $variant_index:expr, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}, $($field_ident:tt => {$($field:tt)*}),*) => {
        $crate::impl_enum_field_offset!($brace, $index, ($($type)+), {$($type_generics),*}, $variant, $variant_index, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $first_field_ident => {$($first_field)*});
        $crate::impl_enum_field_offset!($brace, { ($index) + 1 }, ($($type)+), {$($type_generics),*}, $variant, $variant_index, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $($field_ident => {$($field)*}),*);
    };
}

#[macro_export]
macro_rules! wrap_brace {
    (brace, ($($type:tt)+), $($fields_idents:tt),*) => {
        $($type)+ { $($fields_idents),* }
    };
    (parentheses, ($($type:tt)+), $($fields_idents:tt),*) => {
        $($type)+ ($($fields_idents),*)
    };
    (unit, ($($type:tt)+), $($fields_idents:tt),*) => {
        $($type)+
    };
}

#[macro_export]
macro_rules! impl_field_token {
    () => {
        #[repr(transparent)]
        pub struct __FieldToken<S, T, const I: usize>(
            core::mem::MaybeUninit<T>,
            core::marker::PhantomData<S>,
        );
        impl<C, S, T, const I: usize> $crate::__private::Edge<C> for __FieldToken<S, T, I>
        where
            T: $crate::__private::Edge<C>,
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
                unsafe { self.0.assume_init_ref() }.encode(encoder)
            }
        }
        impl<C, S, S2, T, const I: usize> $crate::__private::CompoundWrapper<C, S>
            for __FieldToken<S2, T, I>
        where
            T: $crate::__private::CompoundWrapper<C, S>,
        {
            type Compound = <T as $crate::__private::CompoundWrapper<C, S>>::Compound;
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
                unsafe { self.0.assume_init_ref() }.as_iter()
            }

            fn as_ptr(&self) -> *const Self::Item {
                unsafe { self.0.assume_init_ref() }.as_ptr()
            }

            fn len(&self) -> usize {
                unsafe { self.0.assume_init_ref() }.len()
            }

            fn set_len(&mut self, len: usize) {
                unsafe { self.0.assume_init_mut() }.set_len(len)
            }
        }
    };
}

#[macro_export]
macro_rules! meshup {
    ($index:expr, ($($type:tt)+), {$($type_generics:tt),*};) => { $crate::__private::End<__C, $($type)+ <$($type_generics),*>> };
    ($index:expr, ($($type:tt)+), {$($type_generics:tt),*}; {$($first:tt)*} $({$($field:tt)*})*) => {
        <<$crate::meshup!({ ($index) + 1 }, ($($type)+), {$($type_generics),*}; $({$($field)*})*)
            as core::ops::Add<$crate::__private::Padding<__C, $($type)+ <$($type_generics),*>, $crate::__private::Field<__FieldToken<$($type)+ <$($type_generics),*>, $($first)*, $index>>>>>::Output
            as core::ops::Add<$crate::__private::Field<__FieldToken<$($type)+ <$($type_generics),*>, $($first)*, $index>>>>::Output
    };
}

#[macro_export]
macro_rules! variant_meshup {
    ($index:expr, ($($type:tt)+), {$($type_generics:tt,)*};) => { $crate::__private::End<__C, $($type)+ <$($type_generics, )* $index>> };
    //TODO rename field to variant
    ($index:expr, ($($type:tt)+), {$($type_generics:tt,)*}; {$($first:tt)*} $({$($field:tt)*})*) => {
        <$crate::variant_meshup!({ ($index) + 1 }, ($($type)+), {$($type_generics,)*}; $({$($field)*})*)
            as core::ops::Add<$crate::__private::Variant<__VariantToken<$($type_generics,)* $index>, $index>>>::Output
    };
}

#[macro_export]
macro_rules! impl_field_offset {
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), ) => {};
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}) => {
        const _: () = {
            impl<$($impl_generics,)*> $crate::__private::FieldOffset for __FieldToken<$($type)+ <$($type_generics),*>, $($first_field)*, $index>
                where
                    $($where_clause)*
                    [(); __field_offset::<$($impl_generics,)*>()]:
            {
                type Offset = $crate::__private::typenum::Const<{ __field_offset::<$($impl_generics,)*>() }>;
            }
            pub const fn __field_offset<$($impl_generics,)*>() -> usize where $($where_clause)* {
                $crate::offset_of!($brace, $($type)+, {$($type_generics),*}, ($($fields_idents),*), $first_field_ident)
            }

        };
    };
    ($brace:ident, $index:expr, ($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*); ($($fields_idents:tt),*), $first_field_ident:tt => {$($first_field:tt)*}, $($field_ident:tt => {$($field:tt)*}),*) => {
        $crate::impl_field_offset!($brace, $index, ($($type)+), {$($type_generics),*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $first_field_ident => {$($first_field)*});
        $crate::impl_field_offset!($brace, { ($index) + 1 }, ($($type)+), {$($type_generics),*}, impl {$($impl_generics,)*} ($($where_clause)*); ($($fields_idents),*), $($field_ident => {$($field)*}),*);
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
    ($brace:ident, $type:ident, {$($type_generics:tt),*}, ($($fields_idents:tt),*), $field:tt) => {{
        use core::mem::MaybeUninit;
        let origin: MaybeUninit<$type <$($type_generics),*>> = MaybeUninit::uninit();
        #[allow(unused_variables)]
        let $crate::wrap_brace!($brace, ($type), $($fields_idents),*) = unsafe { origin.assume_init_ref() };
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
    ($brace:ident, $type:ident, {$($type_generics:tt),*}, $variant:ident, ($($fields_idents:tt),*), $field:tt) => {{
        use core::mem::MaybeUninit;
        unsafe {
            let origin = {
                $(let $fields_idents = MaybeUninit::zeroed().assume_init();)*
               let origin: $type <$($type_generics),*> = $crate::wrap_brace!($brace, ($type::$variant), $($fields_idents),*);
                MaybeUninit::new(origin)
            };
            match origin.assume_init_ref() {
                $crate::wrap_brace!($brace, ($type::$variant), $($fields_idents),*) => {
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
    (($($type:tt)+), {$($type_generics:tt),*}) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics),*}, impl {} $($where_clause)*);
    };
    (($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        impl<$($impl_generics,)* __C> $crate::__private::Edge<__C> for $($type)+ <$($type_generics),*> where $($where_clause)* {
            type First = $crate::__private::End<__C, $($type)+ <$($type_generics),*>>;
            type Second = $crate::__private::PhantomEdge<__C, $($type)+ <$($type_generics),*>, (
                    $crate::__private::Field<$($type)+ <$($type_generics),*>>,
                    $crate::__private::End<__C, $($type)+ <$($type_generics),*>>
                )>;
        }
        impl<$($impl_generics,)* __C, __S> $crate::__private::CompoundWrapper<__C, __S> for $($type)+ <$($type_generics),*> where $($where_clause)* {
            type Compound = $crate::__private::PhantomLeaf<Self>;
        }
        impl<$($impl_generics,)*> $crate::__private::Size for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
        }

    };
}

macro_rules! impl_primitives {
    (($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics),*}, impl {$($impl_generics,)*} ($($where_clause)*));

        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics)*> where $($where_clause)* {
            const SIZE: usize = core::mem::size_of::<$($type)+ <$($type_generics),*>>();
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
    (($($type:tt)+), {$($type_generics:tt),*}, impl {$($impl_generics:tt,)*} ($($where_clause:tt)*)) => {
        $crate::__private::impl_serializable!(($($type)+), {$($type_generics),*}, impl {$($impl_generics,)*} ($($where_clause)*));

        impl<$($impl_generics,)*> $crate::__private::Len for $($type)+ <$($type_generics),*> where $($where_clause)* {
            const SIZE: usize = $crate::__private::UNSIZED;
        }
    };
}

pub(crate) use {impl_non_primitives, impl_primitives, impl_serializable};

/// Replacement for `feature = "ptr_sub_ptr"` which isn't yet stable.
pub const unsafe fn sub_ptr<T>(field: *const T, origin: *const T) -> usize {
    unsafe { field.offset_from(origin) as usize }
}
