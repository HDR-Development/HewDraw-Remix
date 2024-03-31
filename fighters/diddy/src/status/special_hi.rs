use super::*;

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE

pub unsafe extern "C" fn special_hi_charge_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE)(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE

pub unsafe extern "C" fn special_hi_upper_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE)(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE, special_hi_charge_damage_main);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE, special_hi_upper_damage_main);
}