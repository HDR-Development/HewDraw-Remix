use super::*;

// FIGHTER_STATUS_KIND_JUMP 

pub unsafe extern "C" fn jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE {
        if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP {
            if !fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool() {
                fighter.status_pre_Jump_sub_param(
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                    L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                    L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                    L2CValue::I32(0)
                );
            }
            return L2CValue::I32(1);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_POWBLOCK_QUAKE_JUMP);
    fighter.status_pre_Jump_sub_param(
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
        L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
        L2CValue::I32(0)
    );
    return L2CValue::I32(0);
}

pub unsafe extern "C" fn jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP {
        fighter.sub_jump_item_rocketbelt();
        fighter.status_Jump_sub(L2CValue::Hash40s("invalid"), L2CValue::F32(0.0));
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _))
    }
    else {
        // uncomment to let sideB ride remove double jump on hit
        //let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        //WorkModule::set_int(fighter.module_accessor, jump_count_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.status_Jump_sub(L2CValue::new_hash(hash40("jump_b")), L2CValue::F32(0.0));
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Jump_Main as *const () as _))
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP, jump_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP, jump_main);
}