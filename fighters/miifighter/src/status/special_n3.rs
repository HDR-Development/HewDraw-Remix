use super::*;

pub unsafe extern "C" fn special_n3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_status_kind_interrupt(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH);
    return 1.into();
}

unsafe extern "C" fn special_n3_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n3_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy_all(fighter.module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let brake_x = fighter.get_param_float("ground_brake", "");
        let start_limit_speed_ground = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.start_limit_speed_ground");
        let start_brake_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.start_brake_x_mul");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_limit_speed_ground, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_x * start_brake_x_mul, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw3_catch"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_brake_x = fighter.get_param_float("air_brake_x", "");
        let start_air_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.start_air_speed_x_mul");
        let start_air_brake_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.start_air_brake_x_mul");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x * start_air_speed_x_mul, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x * start_air_brake_x_mul, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if !VarModule::is_flag(fighter.object(), vars::miifighter::instance::SPECIAL_N3_STALL) {
            VarModule::on_flag(fighter.object(), vars::miifighter::instance::SPECIAL_N3_STALL);
            let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_throw_start_accel_y"));
            let throw_speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_throw_speed_max_y"));
        let start_air_speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.start_air_speed_y_mul");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * start_air_speed_y_mul, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -start_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, throw_speed_max_y);
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_catch"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_n3_catch_main_loop)
}

unsafe extern "C" fn special_n3_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.status_frame() < 15 {
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
    } else {
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_catch"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw3_catch"), -1.0, 1.0, 0.0, false, false);
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_n3_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n3_throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_float(1.0, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        let throw_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.throw_accel_y");
        let throw_stable_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "wild_throw.throw_stable_speed_y");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, throw_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, throw_stable_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, throw_stable_speed_y);
    }

    return 0.into();
}

unsafe extern "C" fn special_n3_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
    sv_module_access::_catch(fighter.lua_state_agent);
    fighter.set_int(0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
    fighter.set_int(0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
    let counter_attack_power = fighter.get_float(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    let attack_mul = fighter.get_param_float("param_special_lw", "lw3_attack_mul");
    let mut attack_power = counter_attack_power * attack_mul;
    let attack_power_limit = fighter.get_param_float("param_special_lw", "lw3_attack_power_limit");
    if attack_power < attack_power_limit {
        attack_power = 0.0;
    }
    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_IS_ATTACK_ENEMY) {
        let attack_max_for_enemy = fighter.get_param_float("param_special_lw", "lw3_attack_max_for_enemy");
        if attack_max_for_enemy < attack_power {
            attack_power = attack_max_for_enemy;
        }
    }
    else {
        let attack_max = fighter.get_param_float("param_special_lw", "lw3_attack_max");
        if attack_max < attack_power {
            attack_power = attack_max
        }
    }
    let get_node_object_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    fighter.set_float(attack_power, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    fighter.set_int(get_node_object_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw3_throw").into(), Hash40::new("special_air_lw3_throw").into(), false.into());

    fighter.main_shift(special_n3_throw_main_loop)
}

unsafe extern "C" fn special_n3_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING) {
        if !fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    let attack_power = fighter.get_float(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    if 0.0 < attack_power {
        AttackModule::set_power(fighter.module_accessor, 0, attack_power, true);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_n3_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    return 0.into();
}

unsafe extern "C" fn special_n3_throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, special_n3_catch_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, special_n3_catch_main);

    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_pre);
    agent.status(Init, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_init);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_main);
    agent.status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_end);
    agent.status(Exit, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_exit);
}
