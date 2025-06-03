pub enum VarType {
    Bool,
    String,
    Int,
    // OptionString,
    // OptionInt,
}

pub trait GetType {
    fn get_type(&self) -> VarType;
}

// 用于批量实现 GetType trait 的宏
macro_rules! impl_for {
    ($variant:path; $($t:ty),+) => {
        $(
            impl GetType for $t {
                fn get_type(&self) -> VarType {
                    $variant
                }
            }
        )+
    };
}

impl_for!(VarType::Bool; bool);

impl_for!(VarType::String; String, &str);

// 为所有数字类型（整数和浮点数）实现 GetType
impl_for!(VarType::Int;
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
);

// impl_get_type_for_types!(VarType::OptionString; Option<String>, Option<&str>);
// impl_get_type_for_types!(VarType::OptionInt; Option<i32>, Option<u32>);
