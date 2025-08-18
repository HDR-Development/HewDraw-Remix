use super::*;

unsafe extern "C" fn pass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Pass_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(pass_main_loop as *const () as _))
}

unsafe extern "C" fn pass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pass_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_PASS_WORK_INT_FRAME);
    let status = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let was_shield = [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD,
    ].contains(&status);
    if 0 < pass_frame
    && !was_shield {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R) {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
                return 0.into();
            }
        }
    }

    fighter.status_Pass_Main_sub(L2CValue::Ptr(pass_main_subfunction as *const () as _))
}

unsafe extern "C" fn pass_main_subfunction(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.end_pass_ground()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_PASS, pass_main);
}