use super::*;

// FIGHTER_STATUS_KIND_CATCH

unsafe extern "C" fn catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Catch();
}

unsafe extern "C" fn catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_Catch();
}

// FIGHTER_STATUS_KIND_CATCH_PULL

unsafe extern "C" fn catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_CATCH {
        fighter.status_CatchPull_common(hash40("catch_wait").into());
        ControlModule::reset_trigger(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull2"), 9.0, 1.0, false, 0.0, false, false);
        return fighter.main_shift(catch_pull_main_loop)
    }
    else {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_PULL)(fighter);
    }
}

unsafe extern "C" fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull_Main()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_CATCH, catch_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH, catch_main);
    agent.status(Main,*FIGHTER_STATUS_KIND_CATCH_PULL,catch_pull_main);
}
