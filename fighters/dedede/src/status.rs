use super::*;
use globals::*;

// Prevent going into air jet hammer when Special is released during Jumpsquat

unsafe extern "C" fn special_lw_jump_squat_exec(agent: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(agent.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

unsafe extern "C" fn dedede_special_hi_status_main(agent: &mut L2CFighterCommon) -> L2CValue{
    MotionModule::change_motion(agent.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::change_status_force(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

    return 0.into();
}

unsafe extern "C" fn on_start(agent: &mut L2CFighterCommon){
    VarModule::set_int(agent.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    agent.status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, special_lw_jump_squat_exec);
    agent.status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, dedede_special_hi_status_main);
}