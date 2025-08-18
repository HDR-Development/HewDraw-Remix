use super::*;

// FIGHTER_STATUS_KIND_FINAL

pub unsafe extern "C" fn final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32,
        0
    );
    let meter_amount = MeterModule::meter(fighter.battle_object);
    MeterModule::drain_direct(fighter.battle_object, meter_amount);
    return 0.into();
}

unsafe extern "C" fn ken_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0x50000000, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_TARGET_BATTLE_OBJECT_ID);
    fighter.set_float(f32::MAX, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_DIST_MIN);

    let kinetic = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        *FIGHTER_KINETIC_TYPE_MOTION
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION_AIR
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);

    let final_shake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final_shake_scale"));
    AttackModule::set_damage_shake_scale(fighter.module_accessor, final_shake_scale);

    let color = FighterUtil::renderer_get_clear_color();
    let rate = FighterUtil::renderer_get_color_rate();
    fighter.set_float(color.x, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_R);
    fighter.set_float(color.y, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_G);
    fighter.set_float(color.z, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_B);
    fighter.set_float(color.w, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_A);
    fighter.set_float(rate, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_COLOR_RATE);

    fighter.set_int(0, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_INVISIBLE_STAGE_TIME);
    fighter.off_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_INVISIBLE_STAGE);

    ken_final_set_area(fighter, false.into());

    ModelModule::disable_gold_eye(fighter.module_accessor);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final"
    }
    else {
        "final_air"
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ItemModule::set_change_status_event(fighter.module_accessor, false);

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x42400).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ken_final_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let camera_func : fn(fighter: &mut L2CFighterCommon, cam_const: L2CValue) = std::mem::transmute(smashline::api::get_target_function("lua2cpp_ken.nrs", 0x40590).unwrap());
    camera_func(fighter, FIGHTER_RYU_FINAL_CAMERA_OFFSET_1.into());

    let final_shake_scale2 = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final_shake_scale2"));
    AttackModule::set_damage_shake_scale(fighter.module_accessor, final_shake_scale2);

    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);

    ken_final_set_area(fighter, false.into());

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mut zoom = 1.1;
    if PostureModule::scale(fighter.module_accessor) > 1.0 {
        zoom *= PostureModule::scale(fighter.module_accessor);
    }
    CameraModule::zoom_in(
        fighter.module_accessor,
        5,
        0,
        zoom,
        &Vector2f{x: 0.0, y: 0.0},
        true
    );

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final_hit"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x40b30).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ken_final_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_damage_shake_scale(fighter.module_accessor, 1.0);

    ken_final_set_area(fighter, false.into());

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x42100).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

// FIGHTER_RYU_STATUS_KIND_FINAL2

pub unsafe extern "C" fn final2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_RYU_STATUS_KIND_FINAL2)(fighter);
    let meter_amount = MeterModule::meter(fighter.battle_object);
    MeterModule::drain_direct(fighter.battle_object, meter_amount);
    ret
}

unsafe extern "C" fn ken_final2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        *FIGHTER_KINETIC_TYPE_MOTION
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION_AIR
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);

    let final_shake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final_shake_scale"));
    AttackModule::set_damage_shake_scale(fighter.module_accessor, final_shake_scale);

    let color = FighterUtil::renderer_get_clear_color();
    let rate = FighterUtil::renderer_get_color_rate();
    fighter.set_float(color.x, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_R);
    fighter.set_float(color.y, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_G);
    fighter.set_float(color.z, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_B);
    fighter.set_float(color.w, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_A);
    fighter.set_float(rate, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_COLOR_RATE);

    fighter.set_int(0, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_INVISIBLE_STAGE_TIME);
    fighter.off_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_INVISIBLE_STAGE);

    ken_final_set_area(fighter, false.into());

    ModelModule::disable_gold_eye(fighter.module_accessor);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final2"
    }
    else {
        "final2_air"
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ItemModule::set_change_status_event(fighter.module_accessor, false);

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KEN_GENERATE_ARTICLE_SHINRYUKEN, false, -1);

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3e630).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ken_final2_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_fall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let final2_fall_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final2_fall_frame"));
    fighter.set_int(final2_fall_frame, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_final2_fall_substatus as *const () as _));

    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let final2_fall_brake_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final2_fall_brake_y_mul"));
    lua_bind::FighterKineticEnergyGravity::set_brake(gravity as *mut smash::app::FighterKineticEnergyGravity, final2_fall_brake_y_mul);

    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_y_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
    let final2_air_speed_y_stable_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final2_air_speed_y_stable_mul"));
    lua_bind::FighterKineticEnergyGravity::set_stable_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        air_speed_y_stable * final2_air_speed_y_stable_mul
    );
    lua_bind::FighterKineticEnergyGravity::set_limit_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        air_speed_y_limit * final2_air_speed_y_stable_mul
    );

    ken_final_set_area(fighter, false.into());

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3e1f0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ken_final2_fall_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);
    }
    0.into()
}

unsafe extern "C" fn ken_final2_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_air_end"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(ken_final2_air_end_main_loop as *const () as _))
}

unsafe extern "C" fn ken_final2_air_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn ken_final2_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("final2_landing"));
    let final2_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final2_landing_frame")) as f32;
    let rate = end_frame / final2_landing_frame;

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_landing"),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3de90).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_FINAL, final_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_FINAL, ken_final_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_HIT, ken_final_hit_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP, ken_final_jump_main);
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_FINAL2, final2_pre);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2, ken_final2_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_FALL, ken_final2_fall_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_AIR_END, ken_final2_air_end_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING, ken_final2_landing_main);
}
