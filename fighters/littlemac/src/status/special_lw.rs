use super::*;

unsafe extern "C" fn special_lw_hit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_frame() == 0 {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if fighter.is_stick_forward() {
            fighter.change_motion_by_situation("special_lw_hit_f", "special_air_lw_hit_f", 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.change_motion_by_situation("special_lw_hit", "special_air_lw_hit", 0.0, 1.0, false, 0.0, false, false);
        }
        return 0.into();
    }
    fighter.on_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.main_shift(special_lw_hit_main_loop)
}

unsafe extern "C" fn special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if fighter.is_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START) {
        if fighter.is_stick_forward() {
            fighter.change_motion_inherit_frame_by_situation("special_lw_hit_f", "special_air_lw_hit_f", -1.0, 1.0, 0.0, false, false);
        }
        else {
            fighter.change_motion_inherit_frame_by_situation("special_lw_hit", "special_air_lw_hit", -1.0, 1.0, 0.0, false, false);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let dir = if fighter.is_motion(Hash40::new("special_lw_hit_f")) { 1.0 } else { -1.0 };
            let feint_x_speed = fighter.get_param_float("param_special_lw", "special_lw_feint_x_speed_") * fighter.lr() * dir;
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, feint_x_speed, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, feint_x_speed, 0.0);
        }
        else {
            let dir = if fighter.is_motion(Hash40::new("special_air_lw_hit_f")) { 1.0 } else { -1.0 };
            let feint_x_speed = fighter.get_param_float("param_special_lw", "special_air_lw_feint_x_speed_") * fighter.lr() * dir;
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, feint_x_speed, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, feint_x_speed, 0.0);
        }
        fighter.off_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    if fighter.is_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let move_after_accel_y = fighter.get_param_float("param_special_lw", "special_air_lw_move_after_accel_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -move_after_accel_y);
        let move_after_speed_y_max = fighter.get_param_float("param_special_lw", "special_air_lw_move_after_speed_y_max");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, move_after_speed_y_max);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        fighter.off_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        if fighter.is_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            if fighter.is_motion_one_of(&[Hash40::new("special_lw_hit_f"), Hash40::new("special_air_lw_hit_f")]) {
                fighter.change_motion_inherit_frame_by_situation("special_lw_hit_f", "special_air_lw_hit_f", -1.0, 1.0, 0.0, false, false);
            }
            else {
                fighter.change_motion_inherit_frame_by_situation("special_lw_hit", "special_air_lw_hit", -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            if fighter.is_motion_one_of(&[Hash40::new("special_lw_hit_f"), Hash40::new("special_air_lw_hit_f")]) {
                fighter.change_motion_by_situation("special_lw_hit_f", "special_air_lw_hit_f", 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                fighter.change_motion_by_situation("special_lw_hit", "special_air_lw_hit", 0.0, 1.0, false, 0.0, false, false);
            }
            fighter.on_flag(*FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let counter_brake_x = fighter.get_param_float("param_special_lw", "special_lw_counter_brake_x_");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, counter_brake_x);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let counter_brake_x = fighter.get_param_float("param_special_lw", "special_lw_counter_brake_x_");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, counter_brake_x);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    
    return 0.into();
}

unsafe extern "C" fn special_lw_hit_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_pre);
    agent.status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_main);
    agent.status(Exec, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_exec);
}