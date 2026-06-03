use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    // prevent arm attack statuses from circumventing disabled up special measures
    if fighter.is_prev_status_one_of(&[
        *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL,
        *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL_AERIAL,
        *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP,
        *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT,
        *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL,
    ]) {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }

    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
    fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_LASSO_IMMIDIATE);
    fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR.into(), false.into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(situation),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    let rush_angle_g = fighter.get_param_float("param_special_hi", "rush_angle_g");
    fighter.set_float(rush_angle_g.to_radians(), *FIGHTER_TANTAN_STATUS_SPECIAL_HI_WORK_FLOAT_GROUND_ANGLE_RAD);

    fighter.main_shift(special_hi_ground_main_loop)
}

unsafe extern "C" fn special_hi_ground_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_start_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x151502d27a);
    if fighter.status_frame() >= charge_start_frame {
        let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x15c81d2557);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                || fighter.status_frame() >= max_charge_frame {
                let high_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x16aa6f1051);
                if fighter.status_frame() < high_jump_frame {
                    fighter.off_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                }
                else {
                    fighter.on_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                }
                fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP.into(), false.into());
                return 0.into();
            }
        }
        else {
            if fighter.status_frame() == max_charge_frame - 5 {
                let high_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x16aa6f1051);
                fighter.on_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP.into(), false.into());
                return 0.into();
            }
        } 
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        //VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_AIR_JUMP);
        let speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.air_jump_speed_mul");
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    else {
        VarModule::on_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START);
    }

    return smashline::original_status(Init, fighter, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP)(fighter);
}

unsafe extern "C" fn special_hi_ground_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_AIR_STOP,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_TANTAN_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_JUMP_FLAG,
        *FIGHTER_TANTAN_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_JUMP_INT,
        *FIGHTER_TANTAN_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_JUMP_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control_accel_x_mul = fighter.get_param_float("param_special_hi", "end_control_accel_x_mul_g");
    let control_max_speed_x_mul = fighter.get_param_float("param_special_hi", "end_control_max_speed_x_mul_g");
    let control_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x199b5af664);
    let control_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1220fc2660);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_start_speed_mul = if VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START)
        { 1.0 } else { ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.air_jump_start_speed_mul") };
    let air_start_accel_mul = 1.0;// if VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START)
        //{ 1.0 } else { ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.air_jump_start_accel_mul") };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_brake_x, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * control_max_speed_x_mul * air_start_speed_mul, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x_mul * control_accel_x_mul * air_start_accel_mul);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x_add * control_accel_add * air_start_accel_mul);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let motion = if VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_AIR_JUMP)
        { Hash40::new("special_air_hi_short_end") } else { Hash40::new("special_hi_long_end") };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_hi_ground_end_main_loop)
}

unsafe extern "C" fn special_hi_ground_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_frame() >= 12 {
        fighter.sub_air_check_dive();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        };
        return 0.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.landing_frame");
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && !fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if situation == *SITUATION_KIND_GROUND {
        fighter.on_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
    }
    else {
        VarModule::on_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_ENABLE_FREEFALL);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(situation),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.check_hold_input(0, 9, Buttons::SpecialAll) {  // this is in exec, so it will pass on frame 10 of the status
        if !VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START)
        && !VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_AIR_JUMP) {
            // start charging, but only if we haven't already used Arm Jump yet
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
            fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND.into(), false.into());
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09) + -1);
            return 1.into();
        }
    }
    else {
        if fighter.status_frame() == 9 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
            GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_TANTAN_CLIFF_HANG_DATA_AIR_LASSO as u32);
        }
    }
    
    let angle = (fighter.stick_x() * -10.0 * PostureModule::lr(fighter.module_accessor)) - 5.0;
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if (fighter.motion_frame() >= 10.0) {
        fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    
    agent.status(Pre, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_pre);
    agent.status(Init, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_init);
    agent.status(Main, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_main);

    agent.status(Init, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP, special_hi_ground_jump_init);

    agent.status(Pre, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_END, special_hi_ground_end_pre);
    agent.status(Main, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_END, special_hi_ground_end_main);
    
    agent.status(Pre, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, special_hi_air_pre);
    //agent.status(Main, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, special_hi_air_main);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, special_hi_air_exec);

    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH, special_hi_air_reach_exec);
}