use std::ops::{Deref, DerefMut};

use crate::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp15L2CWeaponCommon26sub_weapon_common_settingsEv"]
    fn sub_weapon_common_settings(this: *mut L2CWeaponCommon);

    #[link_name = "_ZN7lua2cpp15L2CWeaponCommon35sub_set_ground_correct_by_situationEN3lib8L2CValueE"]
    fn sub_set_ground_correct_by_situation(this: *mut L2CWeaponCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp15L2CWeaponCommon45sub_ground_module_is_touch_all_consider_speedEv"]
    fn sub_ground_module_is_touch_all_consider_speed(this: *mut L2CWeaponCommon) -> lib::L2CValueHack;
}

#[repr(C)]
pub struct L2CWeaponCommon {
    fighter_base: lua2cpp::L2CFighterBase
}

impl Deref for L2CWeaponCommon {
    type Target = lua2cpp::L2CFighterBase;

    fn deref(&self) -> &Self::Target {
        &self.fighter_base
    }
}

impl DerefMut for L2CWeaponCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fighter_base
    }
}

impl L2CWeaponCommon {
    pub fn sub_weapon_common_settings(&mut self) {
        unsafe {
            sub_weapon_common_settings(self)
        }
    }

    pub fn sub_set_ground_correct_by_situation(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_set_ground_correct_by_situation(self, arg1.into()).into()
        }
    }

    pub fn sub_ground_module_is_touch_all_consider_speed(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ground_module_is_touch_all_consider_speed(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CWeaponCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CWeaponCommon), 0x118);
        assert_eq!(offset_of!(L2CWeaponCommon, fighter_base), 0x0);
    }
}