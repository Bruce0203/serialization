macro_rules! input_num_types {
    ($fn_name:ident!()$(, $extra:ty)*) => {
        $fn_name!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize, i128, u128 $(,$extra)*);
    }
}

macro_rules! input_all_prmitives_of {
    ($fn_name:ident!()) => {
        serialization::macros::input_num_types!($fn_name!(), bool);
    };
}

pub(crate) use {input_all_prmitives_of, input_num_types};
