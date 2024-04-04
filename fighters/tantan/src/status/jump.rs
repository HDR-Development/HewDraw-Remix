use super::*;

// FIGHTER_STATUS_KIND_JUMP

pub unsafe extern "C" fn jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let arg = !(fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP);
    
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(arg)).get_bool() {
        return 1.into()
    }
    else {

        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_NONE),
                L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION)
            );
        }
        else {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                L2CValue::I32(0)
            );
        }
        return 0.into()
    }
}

// FIGHTER_STATUS_KIND_JUMP_SQUAT
// Fixes bug related to pressing Jump and Catch at the same time results in an in-place wavedash

pub unsafe extern "C" fn jump_squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_on(Buttons::Catch) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH, false);
        return 1.into();
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_JUMP_SQUAT)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP, jump_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP_SQUAT, jump_squat_pre);
}
