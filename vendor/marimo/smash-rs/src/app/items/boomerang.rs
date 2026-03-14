use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9boomerang23is_catch_area_hit_checkERNS_18ItemModuleAccessorERKN3phx8Vector3fES6_f"]
        pub(super) fn is_catch_area_hit_check(module_accessor: *mut app::ItemModuleAccessor, arg2: *const phx::Vector3f, arg3: *const phx::Vector3f, arg4: f32) -> bool;
    
        #[link_name = "_ZN3app9boomerang16send_event_catchERNS_18ItemModuleAccessorE"]
        pub(super) fn send_event_catch(module_accessor: *mut app::ItemModuleAccessor) -> bool;
    }
}

pub fn is_catch_area_hit_check(module_accessor: &mut app::ItemModuleAccessor, arg2: &phx::Vector3f, arg3: &phx::Vector3f, arg4: f32) -> bool {
    unsafe {
        impl_::is_catch_area_hit_check(module_accessor, arg2, arg3, arg4)
    }
}

pub fn send_event_catch(module_accessor: &mut app::ItemModuleAccessor) -> bool {
    unsafe {
        impl_::send_event_catch(module_accessor)
    }
}

