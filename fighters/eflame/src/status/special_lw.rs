use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        (
            0x100 | // *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SLOW
            *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_SPRING |
            0x8000000 | // *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_STOP
            *FIGHTER_STATUS_ATTR_START_TURN
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    transfer_hit_cancel(fighter);
    
    return 0.into();
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    VarModule::off_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL);
    ret
}

unsafe extern "C" fn special_lw_out_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL);
    return 0.into();
}

unsafe extern "C" fn transfer_hit_cancel(fighter: &mut L2CFighterCommon) {
    if let Some(object_id) = Some(fighter.battle_object_id + 0x10000) {
        let object = crate::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let object = unsafe { &mut *object };
            let kind = object.kind as i32;
            if kind == *FIGHTER_KIND_ELIGHT {
                if VarModule::is_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL) {
                    VarModule::off_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL);
                    VarModule::on_flag(object, vars::elight::instance::HIT_CANCEL);
                }
                return;
            }
        }
    }
    //This is used if the player selects Mythra first
    if let Some(object_id) = Some(fighter.battle_object_id - 0x10000) {
        let object = crate::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let object = unsafe { &mut *object };
            let kind = object.kind as i32;
            if kind == *FIGHTER_KIND_ELIGHT {
                if VarModule::is_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL) {
                    VarModule::off_flag(fighter.battle_object, vars::eflame::instance::HIT_CANCEL);
                    VarModule::on_flag(object, vars::elight::instance::HIT_CANCEL);
                }
                return;
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);

    agent.status(End, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, special_lw_out_end);
}