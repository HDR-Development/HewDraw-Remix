use smash::app::BattleObject;
extern "Rust" {
    #[link_name = "MeterModule__show"]
    fn MeterModule__show(object: *mut BattleObject);

    #[link_name = "MeterModule__stop_show"]
    fn MeterModule__stop_show(object: *mut BattleObject);

    #[link_name = "MeterModule__set_meter_per_level"]
    fn MeterModule__set_meter_per_level(object: *mut BattleObject, amount: f32);

    #[link_name = "MeterModule__meter_per_level"]
    fn MeterModule__meter_per_level(object: *mut BattleObject) -> f32;

    #[link_name = "MeterModule__set_meter_cap"]
    fn MeterModule__set_meter_cap(object: *mut BattleObject, amount: i32);

    #[link_name = "MeterModule__meter_cap"]
    fn MeterModule__meter_cap(object: *mut BattleObject) -> i32;

    #[link_name = "MeterModule__meter"]
    fn MeterModule__meter(object: *mut BattleObject) -> f32;

    #[link_name = "MeterModule__level"]
    fn MeterModule__level(object: *mut BattleObject) -> i32;

    #[link_name = "MeterModule__watch_damage"]
    fn MeterModule__watch_damage(object: *mut BattleObject, watch: bool);

    #[link_name = "MeterModule__set_damage_gain_mul"]
    fn MeterModule__set_damage_gain_mul(object: *mut BattleObject, mul: f32);

    #[link_name = "MeterModule__damage_gain_mul"]
    fn MeterModule__damage_gain_mul(object: *mut BattleObject) -> f32;

    #[link_name = "MeterModule__drain"]
    fn MeterModule__drain(object: *mut BattleObject, count: i32) -> bool;

    #[link_name = "MeterModule__drain_direct"]
    fn MeterModule__drain_direct(object: *mut BattleObject, amount: f32);

    #[link_name = "MeterModule__add"]
    fn MeterModule__add(object: *mut BattleObject, amount: f32);

    #[link_name = "MeterModule__reset"]
    fn MeterModule__reset(object: *mut BattleObject);

    #[link_name = "MeterModule__update"]
    fn MeterModule__update(object: *mut BattleObject, show_flash_on_change: bool);

}

#[allow(non_snake_case)]
pub mod MeterModule {
    use super::*;
    pub fn show(object: *mut BattleObject) {
        unsafe {
            MeterModule__show(object)
        }
    }
    pub fn stop_show(object: *mut BattleObject) {
        unsafe {
            MeterModule__stop_show(object)
        }
    }
    pub fn set_meter_per_level(object: *mut BattleObject, amount: f32) {
        unsafe {
            MeterModule__set_meter_per_level(object, amount)
        }
    }
    pub fn meter_per_level(object: *mut BattleObject) -> f32 {
        unsafe {
            MeterModule__meter_per_level(object)
        }
    }
    pub fn set_meter_cap(object: *mut BattleObject, amount: i32) {
        unsafe {
            MeterModule__set_meter_cap(object, amount)
        }
    }
    pub fn meter_cap(object: *mut BattleObject) -> i32 {
        unsafe {
            MeterModule__meter_cap(object)
        }
    }
    pub fn meter(object: *mut BattleObject) -> f32 {
        unsafe {
            MeterModule__meter(object)
        }
    }
    pub fn level(object: *mut BattleObject) -> i32 {
        unsafe {
            MeterModule__level(object)
        }
    }
    pub fn watch_damage(object: *mut BattleObject, watch: bool) {
        unsafe {
            MeterModule__watch_damage(object, watch)
        }
    }
    pub fn set_damage_gain_mul(object: *mut BattleObject, mul: f32) {
        unsafe {
            MeterModule__set_damage_gain_mul(object, mul)
        }
    }
    pub fn damage_gain_mul(object: *mut BattleObject) -> f32 {
        unsafe {
            MeterModule__damage_gain_mul(object)
        }
    }
    pub fn drain(object: *mut BattleObject, count: i32) -> bool {
        unsafe {
            MeterModule__drain(object, count)
        }
    }
    pub fn drain_direct(object: *mut BattleObject, amount: f32) {
        unsafe {
            MeterModule__drain_direct(object, amount)
        }
    }
    pub fn add(object: *mut BattleObject, amount: f32) {
        unsafe {
            MeterModule__add(object, amount)
        }
    }
    pub fn reset(object: *mut BattleObject) {
        unsafe {
            MeterModule__reset(object)
        }
    }
    pub fn update(object: *mut BattleObject, show_flash_on_change: bool) {
        unsafe {
            MeterModule__update(object, show_flash_on_change)
        }
    }
}