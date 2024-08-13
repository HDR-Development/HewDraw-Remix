use super::*;
 
// FIGHTER_STATUS_KIND_JUMP

unsafe extern "C" fn pickel_jump_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_prev_status_one_of(&[
        *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP]) {
        return false.into();
    } else {
        return true.into();
    }
}

pub unsafe extern "C" fn jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool()
    {
        return 1.into();
    } else {
        if pickel_jump_status_check(fighter).get_bool() {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*KINETIC_TYPE_NONE),
                L2CValue::I32(
                    *FS_SUCCEEDS_KEEP_EFFECT
                    | *FS_SUCCEEDS_KEEP_SOUND
                    | *FS_SUCCEEDS_KEEP_TRANSITION
                    | *FS_SUCCEEDS_KEEP_CANCEL,
                ),
            );
        } else {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                L2CValue::I32(0),
            );
        }
        return 0.into();
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP, jump_pre);
}