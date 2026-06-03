use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}


unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let flick_x = ControlModule::get_flick_no_reset_x(fighter.module_accessor);
    let special_smash_flick_x = fighter.get_param_int("common", "special_smash_flick_x");
    WorkModule::set_flag(fighter.module_accessor, flick_x >= special_smash_flick_x, *FIGHTER_PZENIGAME_STATUS_SPECIAL_S_FLAG_SMASH);
    AreaModule::enable_area(fighter.module_accessor, *FIGHTER_PZENIGAME_AREA_KIND_SPECIAL_S_TREADED, true, -1);
    special_s_change_motion(fighter, false, Hash40::new("special_s_start"), Hash40::new("special_air_s_start"), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_AIR_S_START);
    fighter.on_flag(*FIGHTER_PZENIGAME_STATUS_SPECIAL_S_FLAG_CONTINUE);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_change_motion(fighter, true, Hash40::new("special_s_start"), Hash40::new("special_air_s_start"), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_AIR_S_START);
    }

    return 0.into();
}

unsafe extern "C" fn special_s_change_motion(fighter: &mut L2CFighterCommon, inherit: bool, ground_motion: Hash40, air_motion: Hash40, ground_kinetic: i32, air_kinetic: i32) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, ground_kinetic);
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, ground_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, air_kinetic);
        if inherit {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, air_motion, -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    AreaModule::enable_area(fighter.module_accessor, *FIGHTER_PZENIGAME_AREA_KIND_SPECIAL_S_TREADED, true, -1);
    DamageModule::set_damage_lock(fighter.module_accessor, false);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_s_loop as *const () as _));
    special_s_change_motion(fighter, false, Hash40::new("special_s"), Hash40::new("special_air_s"), *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_S, *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_AIR_S);
    fighter.on_flag(*FIGHTER_PZENIGAME_STATUS_SPECIAL_S_FLAG_CONTINUE);

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
            let start_speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.start_speed_y_mul");
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(1.0, start_speed_y_mul, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        let start_speed_y_add = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.start_speed_y_add");
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(1.0, start_speed_y_add, 1.0));
    }

    fighter.main_shift(special_s_loop_main_loop)
}

unsafe extern "C" fn sub_special_s_loop(fighter: &mut L2CFighterCommon, param: L2CValue) -> L2CValue {
    if param.get_bool() {
        fighter.inc_int(*FIGHTER_PZENIGAME_STATUS_SPECIAL_S_INT_COUNTER);
    }

    return 0.into();
}

unsafe extern "C" fn special_s_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter = fighter.get_int(*FIGHTER_PZENIGAME_STATUS_SPECIAL_S_INT_COUNTER);
    let limit_frame_max = fighter.get_param_int("param_special_s", "limit_frame_max");
    if counter >= limit_frame_max {
        fighter.change_status(FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END.into(), true.into());
        return 0.into();
    }
    let limit_frame_min = fighter.get_param_int("param_special_s", "limit_frame_min");
    if counter >= limit_frame_min {
        if fighter.is_cat_flag(Cat2::CommonGuard)
        || fighter.is_cat_flag(Cat1::AttackN)
        || fighter.is_cat_flag(Cat1::SpecialAny) {
            fighter.change_status(FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END.into(), true.into());
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_change_motion(fighter, true, Hash40::new("special_s"), Hash40::new("special_air_s"), *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_S, *FIGHTER_KINETIC_TYPE_PZENIGAME_SPECIAL_AIR_S);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Main, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_main);
}