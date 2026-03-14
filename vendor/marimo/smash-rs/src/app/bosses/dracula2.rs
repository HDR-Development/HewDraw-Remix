mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app8dracula233notify_melee_start_to_hud_managerEv"]
        pub(super) fn notify_melee_start_to_hud_manager();
    
    }
}

pub fn notify_melee_start_to_hud_manager() {
    unsafe {
        impl_::notify_melee_start_to_hud_manager()
    }
}

