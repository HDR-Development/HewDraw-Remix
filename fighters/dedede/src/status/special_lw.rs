use super::*;
use globals::*;


unsafe extern "C" fn speciallwjumpsquat_exec(agent: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(agent.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, speciallwjumpsquat_exec);
}