use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9curryshot20init_log_attack_infoERNS_18ItemModuleAccessorE"]
        pub(super) fn init_log_attack_info(module_accessor: *mut app::ItemModuleAccessor);
    
        #[link_name = "_ZN3app9curryshot15is_preview_modeEv"]
        pub(super) fn is_preview_mode() -> bool;
    
    }
}

pub fn init_log_attack_info(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::init_log_attack_info(module_accessor)
    }
}

pub fn is_preview_mode() -> bool {
    unsafe {
        impl_::is_preview_mode()
    }
}

