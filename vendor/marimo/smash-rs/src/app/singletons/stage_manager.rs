mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app12StageManagerEE9instance_E"]
        pub(super) static INSTANCE: *mut app::StageManager;

        #[link_name = "_ZN3app8lua_bind46StageManager__is_discretion_final_enabled_implEPNS_12StageManagerE"]
        pub(super) fn is_discretion_final_enabled(manager: *const app::StageManager) -> bool;

        #[link_name = "_ZN3app8lua_bind33StageManager__stage_all_stop_implEPNS_12StageManagerEb"]
        pub(super) fn stage_all_stop(manager: *mut app::StageManager, stop: bool);
    }
}

#[repr(C)]
pub struct StageManager {
    data: [u8; 0x260],
}

impl StageManager {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            impl_::INSTANCE.as_ref()
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            impl_::INSTANCE.as_mut()
        }
    }

    pub fn is_discretion_final_enabled(&self) -> bool {
        unsafe {
            impl_::is_discretion_final_enabled(self)
        }
    }

    pub fn stage_all_stop(&mut self, stop: bool) {
        unsafe {
            impl_::stage_all_stop(self, stop)
        }
    }
}
