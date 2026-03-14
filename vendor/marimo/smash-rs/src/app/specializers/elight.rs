use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app25FighterSpecializer_ELight19attach_esword_diverERNS_21FighterModuleAccessorE"]
        pub(super) fn attach_esword_diver(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterElementFinalModuleCall);
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight19detach_esword_diverERNS_21FighterModuleAccessorE"]
        pub(super) fn detach_esword_diver(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight28is_inside_camera_range_waistERNS_21FighterModuleAccessorE"]
        pub(super) fn is_inside_camera_range_waist(module_accessor: *mut app::FighterModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight16is_training_zoomEv"]
        pub(super) fn is_training_zoom() -> bool;
    
        #[link_name = "_ZN3app25FighterSpecializer_ELight22kirby_esword_update_lrERNS_21FighterModuleAccessorE"]
        pub(super) fn kirby_esword_update_lr(module_accessor: *mut app::FighterModuleAccessor);
    
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

pub fn is_inside_camera_range_waist(module_accessor: &mut app::FighterModuleAccessor) -> bool {
    unsafe {
        impl_::is_inside_camera_range_waist(module_accessor)
    }
}

pub fn is_training_zoom() -> bool {
    unsafe {
        impl_::is_training_zoom()
    }
}

pub fn kirby_esword_update_lr(module_accessor: &mut app::FighterModuleAccessor) {
    unsafe {
        impl_::kirby_esword_update_lr(module_accessor)
    }
}

