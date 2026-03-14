use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9bomberman16create_item_haveERNS_18ItemModuleAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn create_item_have(module_accessor: *mut app::ItemModuleAccessor, item_kind: app::ItemKind, node: phx::Hash40) -> u32;
    
        #[link_name = "_ZN3app9bomberman8put_itemERNS_18ItemModuleAccessorEjRKN3phx8Vector3fE"]
        pub(super) fn put_item(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: u32, offset: *const phx::Vector3f);
    
        #[link_name = "_ZN3app9bomberman10set_searchERNS_18ItemModuleAccessorEN3phx6Hash40E"]
        pub(super) fn set_search(module_accessor: *mut app::ItemModuleAccessor, param_name: phx::Hash40);
    
    }
}

pub fn create_item_have(module_accessor: &mut app::ItemModuleAccessor, item_kind: app::ItemKind, node: phx::Hash40) -> u32 {
    unsafe {
        impl_::create_item_have(module_accessor, item_kind, node)
    }
}

pub fn put_item(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: u32, offset: &phx::Vector3f) {
    unsafe {
        impl_::put_item(module_accessor, battle_object_id, offset)
    }
}

pub fn set_search(module_accessor: &mut app::ItemModuleAccessor, param_name: phx::Hash40) {
    unsafe {
        impl_::set_search(module_accessor, param_name)
    }
}

