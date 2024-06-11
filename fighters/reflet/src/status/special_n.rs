use super::*;

// FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START

pub unsafe extern "C" fn special_n_tron_start_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object, vars::reflet::instance::THUNDER_CHARGE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) as f32);
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, special_n_tron_start_init);
}
