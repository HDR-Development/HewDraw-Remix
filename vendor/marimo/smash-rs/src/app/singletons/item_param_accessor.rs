use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app17ItemParamAccessorEE9instance_E"]
        pub(super) static INSTANCE: *mut app::ItemParamAccessor;

        #[link_name = "_ZN3app8lua_bind45ItemParamAccessor__boss_common_param_int_implEPNS_17ItemParamAccessorENS_8ItemKindENS_18BossCommonParamIntE"]
        pub(super) fn boss_common_param_int(param_accessor: *const app::ItemParamAccessor, item_kind: app::ItemKind, param: app::BossCommonParamInt) -> i32;

        #[link_name = "_ZN3app8lua_bind44ItemParamAccessor__get_self_param_float_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn get_self_param_float(param_accessor: *const app::ItemParamAccessor, item_kind: app::ItemKind, param_name: phx::Hash40) -> f32;

        #[link_name = "_ZN3app8lua_bind42ItemParamAccessor__get_self_param_int_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn get_self_param_int(param_accessor: *const app::ItemParamAccessor, item_kind: app::ItemKind, param_name: phx::Hash40) -> i32;

        #[link_name = "_ZN3app8lua_bind43ItemParamAccessor__is_valid_self_param_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn is_valid_self_param(param_accessor: *const app::ItemParamAccessor, item_kind: app::ItemKind, param_name: phx::Hash40) -> bool;
    }
}

#[repr(C)]
pub struct ItemParamAccessor {}

impl ItemParamAccessor {
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

    pub fn boss_common_param_int(&self, item_kind: app::ItemKind, param: app::BossCommonParamInt) -> i32 {
        unsafe {
            impl_::boss_common_param_int(self, item_kind, param)
        }
    }

    pub fn get_self_param_float(&self, item_kind: app::ItemKind, param_name: phx::Hash40) -> f32 {
        unsafe {
            impl_::get_self_param_float(self, item_kind, param_name)
        }
    }

    pub fn get_self_param_int(&self, item_kind: app::ItemKind, param_name: phx::Hash40) -> i32 {
        unsafe {
            impl_::get_self_param_int(self, item_kind, param_name)
        }
    }

    pub fn is_valid_self_param(&self, item_kind: app::ItemKind, param_name: phx::Hash40) -> bool {
        unsafe {
            impl_::is_valid_self_param(self, item_kind, param_name)
        }
    }
}
