use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app13crazyhandbomb35send_event_parents_invincible_frameERNS_18ItemModuleAccessorEji"]
        pub(super) fn send_event_parents_invincible_frame(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: u32, invincible_frame: i32);
    
    }
}

pub fn send_event_parents_invincible_frame(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: u32, invincible_frame: i32) {
    unsafe {
        impl_::send_event_parents_invincible_frame(module_accessor, battle_object_id, invincible_frame)
    }
}

