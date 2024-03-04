use super::*;

#[no_mangle]
unsafe fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let keep = if prev_status_kind == float_status.get_i32() {
        (
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT
        )
    }
    else {
        (
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT
        )
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep.0,
        keep.1,
        keep.2,
        *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_EFFECT
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}

#[no_mangle]
unsafe fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != float_status.get_i32() {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
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

    // fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}