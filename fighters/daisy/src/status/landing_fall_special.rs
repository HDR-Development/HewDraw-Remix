use super::*;

unsafe extern "C" fn landing_fall_special_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(fighter);
    if fighter.is_prev_status(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END) {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, 0.0);
    }
    
    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_init);
}