use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app6cyborg13set_reflectorERNS_18ItemModuleAccessorE"]
        pub(super) fn set_reflector(module_accessor: *mut app::ItemModuleAccessor);
    
    }
}

pub fn set_reflector(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::set_reflector(module_accessor)
    }
}

