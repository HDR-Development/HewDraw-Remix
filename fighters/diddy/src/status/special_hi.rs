use super::*;
use globals::*;


// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE

#[status_script(agent = "diddy", status = FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_charge_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE

#[status_script(agent = "diddy", status = FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_upper_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_charge_damage_main,
        special_hi_upper_damage_main
    );
}