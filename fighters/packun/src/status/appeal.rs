use super::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_APPEAL)(fighter);

    // left taunt
    if fighter.is_button_trigger(Buttons::AppealSL) {
        // this is also done in acmd to account for the button_trigger check failing on the last frame of tap buffer for... reasons
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_2"), 0.0, 1.0, false, 0.0, false, false);
    }

    if app::smashball::is_training_mode() {
        if fighter.is_motion(Hash40::new("appeal_s_2")) { SET_STANCE(fighter, 0, false); }
        if fighter.is_motion_one_of(&[Hash40::new("appeal_lw_l"), Hash40::new("appeal_lw_r")]) { SET_STANCE(fighter, 1, false); }
        else if fighter.is_motion_one_of(&[Hash40::new("appeal_s_l"), Hash40::new("appeal_s_r")]) { SET_STANCE(fighter, 2, false); }
    }

    return ret;
}

unsafe extern "C" fn appeal_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion(Hash40::new("appeal_s_2"))
    && fighter.motion_frame() == 92.0
    && fighter.is_button_on(Buttons::AppealSL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_2"), 45.0, 1.0, false, 0.0, false, false);
    }
    
    return 0.into();
}

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("packun_appeal_left"), true, false);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, appeal_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}