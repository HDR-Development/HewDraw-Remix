use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Buddy17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterBuddyFinalModuleCall);
    
        #[link_name = "_ZN3app24FighterSpecializer_Buddy24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Buddy31update_special_n_partner_motionERNS_7FighterEb"]
        pub(super) fn update_special_n_partner_motion(fighter: *mut app::Fighter, arg: bool);
    
    }
}

pub fn call_final_module(fighter: &mut app::Fighter, call_arg: app::FighterBuddyFinalModuleCall) {
    unsafe {
        impl_::call_final_module(fighter, call_arg)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn update_special_n_partner_motion(fighter: &mut app::Fighter, arg: bool) {
    unsafe {
        impl_::update_special_n_partner_motion(fighter, arg)
    }
}

