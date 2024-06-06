use super::*;

// FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE

unsafe extern "C" fn special_hi_failure_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_GROUND_BONK){
        smashline::original_status(Main, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE)(fighter); //long ass landing anim, doesnt work with jet flag rn...
    }
    else{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
    return 0.into();

}

pub fn install(agent: &mut Agent){
    agent.status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, special_hi_failure_main);
}