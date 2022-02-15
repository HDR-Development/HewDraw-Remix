// status imports
use super::*;
use globals::*;
// This file contains code related to teching

pub fn install() {
    install_hooks!(
        sub_check_passive_button_for_damage
    );
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon35sub_check_passive_button_for_damageEN3lib8L2CValueE")]
unsafe extern "C" fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, trigger_frame: L2CValue) -> L2CValue {
    let is_valid_tech_input = fighter.sub_check_passive_button(trigger_frame).get_bool();
    return is_valid_tech_input.into()
}