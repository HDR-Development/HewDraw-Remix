use super::*;

pub unsafe extern "C" fn special_n3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

// Chakram main
pub unsafe extern "C" fn special_n3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.set_int64(hash40("special_s3_1") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION);
    fighter.set_int64(hash40("special_air_s3_1") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION_AIR);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) - 1);

    let main_loop = smashline::api::get_target_function("lua2cpp_miiswordsman.nrs", 0x215b0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub unsafe extern "C" fn special_n3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, ArticleOperationTarget(0));
    return 0.into();
}