use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app7ai_rule7is_1on1EP9lua_State"]
        pub(super) fn is_1on1(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app7ai_rule17is_blow_rate_highEP9lua_State"]
        pub(super) fn is_blow_rate_high(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app7ai_rule5is_hpEv"]
        pub(super) fn is_hp() -> bool;
    
        #[link_name = "_ZN3app7ai_rule15is_normal_meleeEv"]
        pub(super) fn is_normal_melee() -> bool;
    
        #[link_name = "_ZN3app7ai_rule15is_sudden_deathEv"]
        pub(super) fn is_sudden_death() -> bool;
    
    }
}

pub fn is_1on1(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_1on1(state)
    }
}

pub fn is_blow_rate_high(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_blow_rate_high(state)
    }
}

pub fn is_hp() -> bool {
    unsafe {
        impl_::is_hp()
    }
}

pub fn is_normal_melee() -> bool {
    unsafe {
        impl_::is_normal_melee()
    }
}

pub fn is_sudden_death() -> bool {
    unsafe {
        impl_::is_sudden_death()
    }
}

