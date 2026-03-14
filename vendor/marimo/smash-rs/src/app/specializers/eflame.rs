use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app25FighterSpecializer_EFlame19attach_esword_diverERNS_21FighterModuleAccessorE"]
        pub(super) fn attach_esword_diver(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app25FighterSpecializer_EFlame17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterElementFinalModuleCall);
    
        #[link_name = "_ZN3app25FighterSpecializer_EFlame19detach_esword_diverERNS_21FighterModuleAccessorE"]
        pub(super) fn detach_esword_diver(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app25FighterSpecializer_EFlame24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app25FighterSpecializer_EFlame22kirby_esword_update_lrERNS_21FighterModuleAccessorE"]
        pub(super) fn kirby_esword_update_lr(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app25FighterSpecializer_EFlame25set_target_fighter_offsetEiRKN3phx8Vector3fE"]
        pub(super) fn set_target_fighter_offset(arg: i32, offset: *const phx::Vector3f);
    
    }
}

pub fn attach_esword_diver(module_accessor: &mut app::FighterModuleAccessor) {
    unsafe {
        impl_::attach_esword_diver(module_accessor)
    }
}

pub fn call_final_module(fighter: &mut app::Fighter, call_arg: app::FighterElementFinalModuleCall) {
    unsafe {
        impl_::call_final_module(fighter, call_arg)
    }
}

pub fn detach_esword_diver(module_accessor: &mut app::FighterModuleAccessor) {
    unsafe {
        impl_::detach_esword_diver(module_accessor)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn kirby_esword_update_lr(module_accessor: &mut app::FighterModuleAccessor) {
    unsafe {
        impl_::kirby_esword_update_lr(module_accessor)
    }
}

pub fn set_target_fighter_offset(arg: i32, offset: &phx::Vector3f) {
    unsafe {
        impl_::set_target_fighter_offset(arg, offset)
    }
}

