use super::*;

pub unsafe extern "C" fn special_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_FLAG_ST_INIT);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND);
    }
    else {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR);
    }

    return 1.into();
}

unsafe extern "C" fn special_s3_2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_s3_2_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_MOT_RESTART) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s3_2"), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_MOT_RESTART);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s3_2"), -1.0, 1.0, 0.0, false, false);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) - 1);

    fighter.main_shift(special_s3_2_air_main_loop)
}

unsafe extern "C" fn special_s3_2_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s3_2"), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s3_2"), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        }
    }
    sub_special_s3_2_air(fighter);
    if fighter.status_frame() > 23 {
        fighter.check_land_cancel(Some(12.0));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }

    return 0.into();
}

unsafe fn sub_special_s3_2_air(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[IS_STOPPING].get_bool()
    || fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_S {
        return;
    }
    if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON) {
        if !fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON_GENERATED) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_SUPERMISSILE, false, -1);
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_SUPERMISSILE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON_GENERATED);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, special_s3_2_pre);
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, special_s3_2_air_main);
}