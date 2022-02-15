use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_jump,
    );
}

// FIGHTER_STATUS_KIND_JUMP //

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut arg = true;
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
        arg = false;
    }
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(arg)).get_bool() {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*KINETIC_TYPE_NONE),
                L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION | *FS_SUCCEEDS_KEEP_CANCEL)
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