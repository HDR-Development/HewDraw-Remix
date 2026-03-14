use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app13bombermanbomb24send_event_bomberman_hitERNS_18ItemModuleAccessorEjRKN3phx8Vector3fE"]
        pub(super) fn send_event_bomberman_hit(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: u32, pos: *const phx::Vector3f);
    
    }
}

pub fn send_event_bomberman_hit(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: u32, pos: &phx::Vector3f) {
    unsafe {
        impl_::send_event_bomberman_hit(module_accessor, battle_object_id, pos)
    }
}

