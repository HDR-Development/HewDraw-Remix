use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app11androssshot9hit_checkERNS_18ItemModuleAccessorE"]
        pub(super) fn hit_check(module_accessor: *mut app::ItemModuleAccessor);
    
        #[link_name = "_ZN3app11androssshot14is_valid_pos_zEff"]
        pub(super) fn is_valid_pos_z(pos_z: f32, clip_far_offset: f32) -> bool;
    
    }
}

pub fn hit_check(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::hit_check(module_accessor)
    }
}

pub fn is_valid_pos_z(pos_z: f32, clip_far_offset: f32) -> bool {
    unsafe {
        impl_::is_valid_pos_z(pos_z, clip_far_offset)
    }
}

