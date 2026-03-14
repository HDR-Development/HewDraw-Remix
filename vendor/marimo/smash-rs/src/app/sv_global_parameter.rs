use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app19sv_global_parameter28melee_melee_info_item_appearEv"]
        pub(super) fn melee_melee_info_item_appear() -> phx::Hash40;

        #[link_name = "_ZN3app19sv_global_parameter15melee_rule_modeEv"]
        pub(super) fn melee_rule_mode() -> phx::Hash40;

        #[link_name = "_ZN3app19sv_global_parameter15melee_rule_typeEv"]
        pub(super) fn melee_rule_type() -> phx::Hash40;
    }
}

pub fn melee_melee_info_item_appear() -> phx::Hash40 {
    unsafe {
        impl_::melee_melee_info_item_appear()
    }
}

pub fn melee_rule_mode() -> phx::Hash40 {
    unsafe {
        impl_::melee_rule_mode()
    }
}

pub fn melee_rule_type() -> phx::Hash40 {
    unsafe {
        impl_::melee_rule_type()
    }
}
