use super::*;
use globals::*;

pub unsafe extern "C" fn init_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::reflet::instance::THUNDER_CHARGE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT));
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

pub fn install() {
    smashline::Agent::new("reflet")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, init_special_n)
        .install();
}
