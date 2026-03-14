use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app20crazyhandgravityball20send_catch_cut_eventERNS_18ItemModuleAccessorE"]
        pub(super) fn send_catch_cut_event(module_accessor: *mut app::ItemModuleAccessor);
    
        #[link_name = "_ZN3app20crazyhandgravityball20send_catch_end_eventERNS_18ItemModuleAccessorEi"]
        pub(super) fn send_catch_end_event(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: i32);
    
        #[link_name = "_ZN3app20crazyhandgravityball22send_catch_start_eventERNS_18ItemModuleAccessorEi"]
        pub(super) fn send_catch_start_event(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: i32);
    
    }
}

pub fn send_catch_cut_event(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::send_catch_cut_event(module_accessor)
    }
}

pub fn send_catch_end_event(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: i32) {
    unsafe {
        impl_::send_catch_end_event(module_accessor, battle_object_id)
    }
}

pub fn send_catch_start_event(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: i32) {
    unsafe {
        impl_::send_catch_start_event(module_accessor, battle_object_id)
    }
}

