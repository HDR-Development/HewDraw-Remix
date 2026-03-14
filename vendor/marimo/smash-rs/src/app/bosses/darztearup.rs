mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app10darztearup19cancel_fighter_slowEi"]
        pub(super) fn cancel_fighter_slow(entry_id: i32);
    
        #[link_name = "_ZN3app10darztearup16set_fighter_slowEiii"]
        pub(super) fn set_fighter_slow(entry_id: i32, magnitude: i32, frames: i32);
    
    }
}

pub fn cancel_fighter_slow(entry_id: i32) {
    unsafe {
        impl_::cancel_fighter_slow(entry_id)
    }
}

pub fn set_fighter_slow(entry_id: i32, magnitude: i32, frames: i32) {
    unsafe {
        impl_::set_fighter_slow(entry_id, magnitude, frames)
    }
}

