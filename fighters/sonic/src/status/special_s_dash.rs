use super::*;
use globals::*;
use smashline::*;

// FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH

pub unsafe extern "C" fn special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let log_mask_flags;
    let power_up_bit;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        log_mask_flags = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
        power_up_bit = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW;
    }
    else {
        log_mask_flags = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
        power_up_bit = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S;
    }
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        log_mask_flags as u64,
        0,
        power_up_bit as u32,
        0
    );
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_pre);
}
