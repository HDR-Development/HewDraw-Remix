// status imports
use super::*;
use globals::*;
// This file contains code related to teching

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_check_passive_button_for_damage
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_check_passive_button_for_damage)]
pub unsafe fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, trigger_frame: L2CValue) -> L2CValue {
    let is_valid_tech_input = fighter.sub_check_passive_button(trigger_frame).get_bool();
    return L2CValue::Bool(is_valid_tech_input)
}