use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app26FighterSpecializer_Gaogaen13generate_ropeERNS_21FighterModuleAccessorE"]
        pub(super) fn generate_rope(module_accessor: *mut app::FighterModuleAccessor);
    
        #[link_name = "_ZN3app26FighterSpecializer_Gaogaen11revenge_offERNS_7FighterEb"]
        pub(super) fn revenge_off(fighter: *mut app::Fighter, turn_off: bool);
    
    }
}

pub fn generate_rope(module_accessor: &mut app::FighterModuleAccessor) {
    unsafe {
        impl_::generate_rope(module_accessor)
    }
}

pub fn revenge_off(fighter: &mut app::Fighter, turn_off: bool) {
    unsafe {
        impl_::revenge_off(fighter, turn_off)
    }
}

