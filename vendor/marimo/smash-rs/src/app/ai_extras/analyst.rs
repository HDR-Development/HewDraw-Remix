use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app7analyst13chanced_frameEP9lua_State"]
        pub(super) fn chanced_frame(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app7analyst13change_statusEP9lua_StateNS_16FighterAIAnalyst8AIStatusE"]
        pub(super) fn change_status(state: *mut lua_State, status: app::FighterAIAnalyst::AIStatus);
    
        #[link_name = "_ZN3app7analyst17set_chanced_frameEP9lua_Statei"]
        pub(super) fn set_chanced_frame(state: *mut lua_State, chanced_frame: i32);
    
        #[link_name = "_ZN3app7analyst6statusEP9lua_State"]
        pub(super) fn status(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus;
    
        #[link_name = "_ZN3app7analyst12status_countEP9lua_State"]
        pub(super) fn status_count(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app7analyst11status_prevEP9lua_State"]
        pub(super) fn status_prev(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus;
    
        #[link_name = "_ZN3app7analyst20target_chanced_frameEP9lua_State"]
        pub(super) fn target_chanced_frame(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app7analyst13target_statusEP9lua_State"]
        pub(super) fn target_status(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus;
    
        #[link_name = "_ZN3app7analyst19target_status_countEP9lua_State"]
        pub(super) fn target_status_count(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app7analyst18target_status_prevEP9lua_State"]
        pub(super) fn target_status_prev(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus;
    
    }
}

pub fn chanced_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::chanced_frame(state)
    }
}

pub fn change_status(state: *mut lua_State, status: app::FighterAIAnalyst::AIStatus) {
    unsafe {
        impl_::change_status(state, status)
    }
}

pub fn set_chanced_frame(state: *mut lua_State, chanced_frame: i32) {
    unsafe {
        impl_::set_chanced_frame(state, chanced_frame)
    }
}

pub fn status(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus {
    unsafe {
        impl_::status(state)
    }
}

pub fn status_count(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::status_count(state)
    }
}

pub fn status_prev(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus {
    unsafe {
        impl_::status_prev(state)
    }
}

pub fn target_chanced_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_chanced_frame(state)
    }
}

pub fn target_status(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus {
    unsafe {
        impl_::target_status(state)
    }
}

pub fn target_status_count(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_status_count(state)
    }
}

pub fn target_status_prev(state: *mut lua_State) -> app::FighterAIAnalyst::AIStatus {
    unsafe {
        impl_::target_status_prev(state)
    }
}