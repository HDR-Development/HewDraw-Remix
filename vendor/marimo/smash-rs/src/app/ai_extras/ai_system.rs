use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9ai_system10add_buttonEP9lua_StateNS_20ControlPadButtonKindE"]
        pub(super) fn add_button(state: *mut lua_State, button: app::ControlPadButton);
    
        #[link_name = "_ZN3app9ai_system9add_stickEP9lua_StateRKN3phx8Vector2fE"]
        pub(super) fn add_stick(state: *mut lua_State, offset: *const phx::Vector2f);
    
        #[link_name = "_ZN3app9ai_system13add_stick_absEP9lua_StateRKN3phx8Vector2fE"]
        pub(super) fn add_stick_abs(state: *mut lua_State, offset: *const phx::Vector2f);
    
        #[link_name = "_ZN3app9ai_system13add_stick_degEP9lua_Statef"]
        pub(super) fn add_stick_deg(state: *mut lua_State, deg: f32);
    
        #[link_name = "_ZN3app9ai_system11change_modeEP9lua_StateNS_9FighterAI4ModeE"]
        pub(super) fn change_mode(state: *mut lua_State, arg2: app::FighterAI::Mode);
    
        #[link_name = "_ZN3app9ai_system18change_mode_actionEP9lua_State"]
        pub(super) fn change_mode_action(state: *mut lua_State);
    
        #[link_name = "_ZN3app9ai_system13change_targetEP9lua_State"]
        pub(super) fn change_target(state: *mut lua_State);
    
        #[link_name = "_ZN3app9ai_system20change_target_randomEP9lua_State"]
        pub(super) fn change_target_random(state: *mut lua_State);
    
        #[link_name = "_ZN3app9ai_system28is_input_available_for_entryEP9lua_State"]
        pub(super) fn is_input_available_for_entry(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app9ai_system4modeEP9lua_State"]
        pub(super) fn mode(state: *mut lua_State) -> app::FighterAI::Mode;
    
        #[link_name = "_ZN3app9ai_system9prev_modeEP9lua_State"]
        pub(super) fn prev_mode(state: *mut lua_State) -> app::FighterAI::Mode;
    
        #[link_name = "_ZN3app9ai_system11reset_stickEP9lua_State"]
        pub(super) fn reset_stick(state: *mut lua_State);
    
        #[link_name = "_ZN3app9ai_system18set_action_id_noneEP9lua_State"]
        pub(super) fn set_action_id_none(state: *mut lua_State);
    
        #[link_name = "_ZN3app9ai_system16set_update_countEP9lua_Statei"]
        pub(super) fn set_update_count(state: *mut lua_State, update_count: i32);
    
    }
}

pub fn add_button(state: *mut lua_State, button: app::ControlPadButton) {
    unsafe {
        impl_::add_button(state, button)
    }
}

pub fn add_stick(state: *mut lua_State, offset: &phx::Vector2f) {
    unsafe {
        impl_::add_stick(state, offset)
    }
}

pub fn add_stick_abs(state: *mut lua_State, offset: &phx::Vector2f) {
    unsafe {
        impl_::add_stick_abs(state, offset)
    }
}

pub fn add_stick_deg(state: *mut lua_State, deg: f32) {
    unsafe {
        impl_::add_stick_deg(state, deg)
    }
}

pub fn change_mode(state: *mut lua_State, arg2: app::FighterAI::Mode) {
    unsafe {
        impl_::change_mode(state, arg2)
    }
}

pub fn change_mode_action(state: *mut lua_State) {
    unsafe {
        impl_::change_mode_action(state)
    }
}

pub fn change_target(state: *mut lua_State) {
    unsafe {
        impl_::change_target(state)
    }
}

pub fn change_target_random(state: *mut lua_State) {
    unsafe {
        impl_::change_target_random(state)
    }
}

pub fn is_input_available_for_entry(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_input_available_for_entry(state)
    }
}

pub fn mode(state: *mut lua_State) -> app::FighterAI::Mode {
    unsafe {
        impl_::mode(state)
    }
}

pub fn prev_mode(state: *mut lua_State) -> app::FighterAI::Mode {
    unsafe {
        impl_::prev_mode(state)
    }
}

pub fn reset_stick(state: *mut lua_State) {
    unsafe {
        impl_::reset_stick(state)
    }
}

pub fn set_action_id_none(state: *mut lua_State) {
    unsafe {
        impl_::set_action_id_none(state)
    }
}

pub fn set_update_count(state: *mut lua_State, update_count: i32) {
    unsafe {
        impl_::set_update_count(state, update_count)
    }
}

