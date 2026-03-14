use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app7chicken17is_behind_fighterERNS_18ItemModuleAccessorE"]
        pub(super) fn is_behind_fighter(module_accessor: *mut app::ItemModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app7chicken16is_front_fighterERNS_18ItemModuleAccessorE"]
        pub(super) fn is_front_fighter(module_accessor: *mut app::ItemModuleAccessor) -> bool;
    
    }
}

pub fn is_behind_fighter(module_accessor: &mut app::ItemModuleAccessor) -> bool {
    unsafe {
        impl_::is_behind_fighter(module_accessor)
    }
}

pub fn is_front_fighter(module_accessor: &mut app::ItemModuleAccessor) -> bool {
    unsafe {
        impl_::is_front_fighter(module_accessor)
    }
}

