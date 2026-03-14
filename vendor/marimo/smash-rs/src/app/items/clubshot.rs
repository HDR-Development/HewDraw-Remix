use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app8clubshot18check_roll_up_areaERNS_18ItemModuleAccessorE"]
        pub(super) fn check_roll_up_area(module_accessor: *mut app::ItemModuleAccessor) -> bool;
    
    }
}

pub fn check_roll_up_area(module_accessor: &mut app::ItemModuleAccessor) -> bool {
    unsafe {
        impl_::check_roll_up_area(module_accessor)
    }
}

