use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        bayonetta_specialairs_d_main,
        bayonetta_specialairs_u_main
    );
}

// FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S_D //

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bayonetta_specialairs_d_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S);
    if fighter.is_prev_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 6.0, 1.0, false, 0.0, false, false);
    } else {MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 0.0, 1.0, false, 0.0, false, false); }
    if !StopModule::is_stop(fighter.module_accessor) {
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT) {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT.into(), false.into());
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_air_s_d_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_special_air_s_d_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StopModule::is_stop(fighter.module_accessor) && fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT) {
        if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED); }
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT.into(), false.into());
    }
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK) {wall_check(fighter); }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING.into(), false.into());
        }
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

// FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S_U //

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bayonetta_specialairs_u_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_u"), 0.0, 1.0, false, 0.0, false, false);
    fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_air_s_u_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_special_air_s_u_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
        if fighter.get_param_int("param_special_s", "ab_u_disable_landing_frame") < frame {
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
        }
    }
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK) {wall_check(fighter); }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    } else {
        if MotionModule::is_end(fighter.module_accessor) { fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
        movement(fighter);
    }
    if frame <= 7 && !StopModule::is_stop(fighter.module_accessor) {
        if fighter.is_button_on(Buttons::Attack | Buttons::Catch) {fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D); }
        if frame == 7 && fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D) {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn wall_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut touch_wall = false;
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }
    if touch_wall {
        if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED); }
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn movement(fighter: &mut L2CFighterCommon) -> L2CValue { //was like 400 lines
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                let x_reset = fighter.get_float(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X);
                let y_reset = fighter.get_float(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_Y);
                let initial_x = fighter.get_param_float("param_special_s", "ab_u_shooting_speed_x_mul");
                let initial_y = fighter.get_param_float("param_special_s", "ab_u_shooting_speed_y_mul");
                let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut KineticEnergy; //272
                let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut KineticEnergy; //288
                let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut KineticEnergy;
                //motion to stop energy
                app::lua_bind::KineticEnergy::reset_energy(stop_energy as _, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f { x: x_reset * initial_x, y: 0.0 }, &Vector3f::zero(), fighter.module_accessor);
                lua_bind::KineticEnergyNormal::set_accel(stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f { x: 0.0, y: 0.0 });
                lua_bind::KineticEnergyNormal::set_brake(stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_brake_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_stable_speed(stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_limit_speed(stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f { x: -1.0, y: -1.0 });
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                //motion to gravity
                app::lua_bind::KineticEnergy::reset_energy(gravity_energy as _, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f { x: 0.0, y: initial_y }, &Vector3f::zero(), fighter.module_accessor);
                smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy as *mut FighterKineticEnergyGravity, -fighter.get_param_float("param_special_s", "ab_u_shooting_accel_y"));
                smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(gravity_energy as *mut FighterKineticEnergyGravity, fighter.get_param_float("param_special_s", "ab_u_shooting_max_speed_y"));
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP);
            }
        } else { //save motion to vars
            let speed = Vector2f{
                x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN),
                y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            };
            fighter.set_float(speed.x, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X);
            fighter.set_float(speed.y, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_Y);
        }
    } else if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING { //shooting 
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut KineticEnergy;
            let speed = Vector2f{
                x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
                y: lua_bind::KineticEnergy::get_speed_y(stop_energy)
            };
            let x_cap = fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x");
            if speed.x <= x_cap {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP);
            }
        }
    }
    0.into()
}