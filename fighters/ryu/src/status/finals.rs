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

unsafe extern "C" fn ryu_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43f70).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final_hit"
    }
    else {
        "final_air_hit"
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

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x430a0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43e90).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_HIT) {
        "final_hit_air_end"
    }
    else {
        "final_air_end"
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

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x435d0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_HIT) {
        "final_hit_fall"
    }
    else {
        "final_fall"
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

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_final_fall_substatus as *const () as _));

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43b30).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_fall_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);
    }
    0.into()
}

unsafe extern "C" fn ryu_final_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_HIT) {
        "final_hit_landing"
    }
    else {
        "final_landing"
    };
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new(mot));
    let final2_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final_landing_frame")) as f32;
    let rate = end_frame / final2_landing_frame;

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43770).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

// FIGHTER_RYU_STATUS_KIND_FINAL2

pub unsafe extern "C" fn final2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_RYU_STATUS_KIND_FINAL2)(fighter);
    let meter_amount = MeterModule::meter(fighter.battle_object);
    MeterModule::drain_direct(fighter.battle_object, meter_amount);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_FINAL, final_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_FINAL, ryu_final_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_HIT, ryu_final_hit_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP, ryu_final_jump_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_AIR_END, ryu_final_air_end_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_FALL, ryu_final_fall_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_LANDING, ryu_final_landing_main);
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_FINAL2, final2_pre);
}
