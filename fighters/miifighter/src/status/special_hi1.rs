use super::*;

unsafe extern "C" fn special_hi1_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0x50000000, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_TENCHI_KICK_FALL_HIT_OBJECT_ID);
    sub_special_hi1_3(fighter);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    fighter.set_int64(hash40("special_air_hi1_3") as i64, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_GROUND_MOT);
    fighter.set_int64(hash40("special_air_hi1_3") as i64, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_AIR_MOT);
    special_hi_change_motion(fighter);
    special_hi_set_control(fighter);
    app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    let hi1_add_fall_speed_y = fighter.get_param_float("param_special_hi", "hi1_add_fall_speed_y");
    let hi1_add_fall_accel_y = fighter.get_param_float("param_special_hi", "hi1_add_fall_accel_y");
    let hi1_add_fall_speed_limit_y = fighter.get_param_float("param_special_hi", "hi1_add_fall_speed_limit_y");
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -hi1_add_fall_speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -hi1_add_fall_speed_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, hi1_add_fall_speed_limit_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    fighter.main_shift(special_hi1_3_main_loop)
}

unsafe fn sub_special_hi1_3(fighter: &mut L2CFighterCommon) {
    fighter.off_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    fighter.off_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
}

unsafe fn special_hi_change_motion(fighter: &mut L2CFighterCommon) {
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.get_int64(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_GROUND_MOT)
    }
    else {
        fighter.get_int64(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_AIR_MOT)
    };
    if fighter.is_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
}

unsafe fn special_hi_set_control(fighter: &mut L2CFighterCommon) {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let air_accel_x_mul = fighter.get_param_float("air_accel_x_mul", "");
    let hi1_jump_mul_x = fighter.get_param_float("param_special_hi", "hi1_jump_mul_x");
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
    let hi1_control_limit_mul_x = fighter.get_param_float("param_special_hi", "hi1_control_limit_mul_x");
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(mul_x_speed_max, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x_mul * hi1_jump_mul_x);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 100.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * hi1_control_limit_mul_x, 0.0);
}

unsafe extern "C" fn special_hi1_3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_4.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3, special_hi1_3_main);
}