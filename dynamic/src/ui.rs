extern "C" {
    #[link_name = "UiManager__set_dk_barrel_enable"]
    fn ui_manager_set_dk_barrel_enable(entry_id: u32, enable: bool);

    #[link_name = "UiManager__set_shoto_meter_enable"]
    fn ui_manager_set_shoto_meter_enable(entry_id: u32, enable: bool);

    #[link_name = "UiManager__set_shoto_bar_percentage"]
    fn ui_manager_set_shoto_bar_percentage(entry_id: u32, percentage: f32);

    #[link_name = "UiManager__set_shoto_number"]
    fn ui_manager_set_shoto_number(entry_id: u32, number: i32);

    #[link_name = "UiManager__set_ex_meter_enable"]
    fn ui_manager_set_ex_meter_enable(entry_id: u32, enable: bool);

    #[link_name = "UiManager__set_ex_meter_info"]
    fn ui_manager_set_ex_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32);

    #[link_name = "UiManager__set_ff_meter_enable"]
    fn ui_manager_set_ff_meter_enable(entry_id: u32, enable: bool);

    #[link_name = "UiManager__set_ff_meter_info"]
    fn ui_manager_set_ff_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32);

    #[link_name = "UiManager__set_pichu_meter_enable"]
    fn ui_manager_set_pichu_meter_enable(entry_id: u32, enable: bool);

    #[link_name = "UiManager__set_pichu_meter_info"]
    fn ui_manager_set_pichu_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32, charged: bool);
}

pub mod UiManager {
    pub fn set_dk_barrel_enable(entry_id: u32, enable: bool) {
        unsafe {
            super::ui_manager_set_dk_barrel_enable(entry_id, enable)
        }
    }

    pub fn set_shoto_meter_enable(entry_id: u32, enable: bool) {
        unsafe {
            super::ui_manager_set_shoto_meter_enable(entry_id, enable)
        }
    }

    pub fn set_shoto_bar_percentage(entry_id: u32, percentage: f32) {
        unsafe {
            super::ui_manager_set_shoto_bar_percentage(entry_id, percentage)
        }
    }

    pub fn set_shoto_number(entry_id: u32, number: i32) {
        unsafe {
            super::ui_manager_set_shoto_number(entry_id, number)
        }
    }

    pub fn set_ex_meter_enable(entry_id: u32, enable: bool) {
        unsafe {
            super::ui_manager_set_ex_meter_enable(entry_id, enable)
        }
    }

    pub fn set_ex_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        unsafe {
            super::ui_manager_set_ex_meter_info(entry_id, current, max, per_level)
        }
    }

    pub fn set_ff_meter_enable(entry_id: u32, enable: bool) {
        unsafe {
            super::ui_manager_set_ff_meter_enable(entry_id, enable)
        }
    }

    pub fn set_ff_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        unsafe {
            super::ui_manager_set_ff_meter_info(entry_id, current, max, per_level)
        }
    }

    pub fn set_pichu_meter_enable(entry_id: u32, enable: bool) {
        unsafe {
            super::ui_manager_set_pichu_meter_enable(entry_id, enable)
        }
    }

    pub fn set_pichu_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32, charged: bool) {
        unsafe {
            super::ui_manager_set_pichu_meter_info(entry_id, current, max, per_level, charged)
        }
    }
}