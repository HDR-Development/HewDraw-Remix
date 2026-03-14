use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app27FighterSpecializer_Gekkouga24set_effect_visible_groupEb"]
        pub(super) fn set_effect_visible_group(arg: bool);
    
        #[link_name = "_ZN3app27FighterSpecializer_Gekkouga42set_special_s_transition_term_forbid_groupERNS_21FighterModuleAccessorEb"]
        pub(super) fn set_special_s_transition_term_forbid_group(module_accessor: *mut app::FighterModuleAccessor, enable: bool);
    
    }
}

pub fn set_effect_visible_group(arg: bool) {
    unsafe {
        impl_::set_effect_visible_group(arg)
    }
}

pub fn set_special_s_transition_term_forbid_group(module_accessor: &mut app::FighterModuleAccessor, enable: bool) {
    unsafe {
        impl_::set_special_s_transition_term_forbid_group(module_accessor, enable)
    }
}

