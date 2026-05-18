use super::*;

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);

    if fighter.is_motion(Hash40::new("attack_air_b"))
    && VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_b_s"), 0.0, 1.0, false, 0.0, false, false);
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}