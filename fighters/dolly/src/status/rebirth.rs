use super::*;

pub unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(
        fighter.battle_object, 
        vars::dolly::instance::ADDED_METER_LEVELS,
        VarModule::get_int(fighter.battle_object, vars::dolly::instance::ADDED_METER_LEVELS) + 2
    );
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);
}