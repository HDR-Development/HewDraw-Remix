use super::*;
utils::import!(common::djc::attack_air_main_status);

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter, statuses::mewtwo::FLOAT.into())
}

unsafe extern "C" fn attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != statuses::mewtwo::FLOAT {
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    }
    fighter.sub_attack_air_uniq_process_init()
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != statuses::mewtwo::FLOAT {
        return common::djc::attack_air_main_status(fighter);
    }

    let motion = MotionModule::motion_kind(fighter.module_accessor);

    let log = match motion {
        0xc3a4e2597 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N),
        0xc3495ada5 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F),
        0xc33f869bc => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B),
        0xdde67d935 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI),
        0xd40042152 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW),
        _ => None
    };

    if let Some(log) = log {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, log);
    }
    // allow fast fall during float release aerials
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_fall_common_uniq as *const () as _));
    // fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}