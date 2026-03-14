use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app25FighterSpecializer_Donkey16apply_lift_paramERNS_21FighterModuleAccessorEb"]
        pub(super) fn apply_lift_param(module_accessor: *mut app::FighterModuleAccessor, on_off: bool);    }
}

pub fn apply_lift_param(module_accessor: &mut app::FighterModuleAccessor, on_off: bool) {
    unsafe {
        impl_::apply_lift_param(module_accessor, on_off)
    }
}

