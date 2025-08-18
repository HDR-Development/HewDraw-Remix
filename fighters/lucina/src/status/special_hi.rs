use super::*;

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL) {
        let fall_x_mul = fighter.get_param_float("param_special_hi", "fall_x_mul_value");
        fighter.set_float(fall_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
}