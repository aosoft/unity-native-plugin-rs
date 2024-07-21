#[macro_export]
macro_rules! bitflag {
    ($flag_type:ident, $flag_enum_type:ty, $flag_value_type:ty) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $flag_type {
            pub flag: $flag_value_type,
        }

        impl From<$flag_enum_type> for $flag_type {
            fn from(value: $flag_enum_type) -> Self {
                $flag_type::new(value)
            }
        }
        
        impl From<$flag_value_type> for $flag_type {
            fn from(value: $flag_value_type) -> Self {
                $flag_type { flag: value }
            }
        }
        
        impl Into<$flag_value_type> for $flag_type {
            fn into(self) -> $flag_value_type {
                self.flag as $flag_value_type
            }
        }
        
        impl $flag_type {
            pub fn new(flag: $flag_enum_type) -> $flag_type {
                $flag_type { flag: flag as $flag_value_type }
            }
        
            pub const fn is_default(&self) -> bool {
                self.flag == 0 as $flag_value_type
            }
        
            pub const fn has_flag(&self, flag: $flag_enum_type) -> bool {
                (self.flag & flag as $flag_value_type) != 0
            }
        
            pub const fn set_flag(&self, flag: $flag_enum_type) -> $flag_type {
                $flag_type {
                    flag: self.flag | flag as $flag_value_type,
                }
            }
        
            pub const fn unset_flag(&self, flag: $flag_enum_type) -> $flag_type {
                $flag_type {
                    flag: self.flag & !(flag as $flag_value_type),
                }
            }
        }
    };
}