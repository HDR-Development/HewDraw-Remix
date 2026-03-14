use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app15ai_notify_event11motion_kindEP9lua_State"]
        pub(super) fn motion_kind(state: *mut lua_State) -> phx::Hash40;
    
        #[link_name = "_ZN3app15ai_notify_event11status_kindEP9lua_State"]
        pub(super) fn status_kind(state: *mut lua_State) -> i32;
    
    }
}

pub fn motion_kind(state: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::motion_kind(state)
    }
}

pub fn status_kind(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::status_kind(state)
    }
}

