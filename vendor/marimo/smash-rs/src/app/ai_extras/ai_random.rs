mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app9ai_random9get_floatEv"]
        pub(super) fn get_float() -> f32;
    
        #[link_name = "_ZN3app9ai_random10is_percentEf"]
        pub(super) fn is_percent(percent: f32) -> bool;
    
        #[link_name = "_ZN3app9ai_random11range_floatEff"]
        pub(super) fn range_float(min: f32, max: f32) -> f32;
    
        #[link_name = "_ZN3app9ai_random9range_intEii"]
        pub(super) fn range_int(min: i32, max: i32) -> i32;
    
    }
}

pub fn get_float() -> f32 {
    unsafe {
        impl_::get_float()
    }
}

pub fn is_percent(percent: f32) -> bool {
    unsafe {
        impl_::is_percent(percent)
    }
}

pub fn range_float(min: f32, max: f32) -> f32 {
    unsafe {
        impl_::range_float(min, max)
    }
}

pub fn range_int(min: i32, max: i32) -> i32 {
    unsafe {
        impl_::range_int(min, max)
    }
}

