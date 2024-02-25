use super::*;
use globals::*;

unsafe extern "C" fn packun_special_s_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !(fighter.is_situation(*SITUATION_KIND_GROUND))  {
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
        let motion = if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 
        {Hash40::new("special_air_s_shoot_s")} else {Hash40::new("special_air_s_shoot")};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        let motion = if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 
        {Hash40::new("special_s_shoot_s")} else {Hash40::new("special_s_shoot")};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }  
    fighter.sub_shift_status_main(L2CValue::Ptr(packun_special_s_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn packun_special_s_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                let motion = if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 
                  {Hash40::new("special_s_shoot_s")} else {Hash40::new("special_s_shoot")};
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor,*FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
            }
            else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC) {
                    let accel = -1.0 * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("accel_y"));
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel);
                    app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        FIGHTER_KINETIC_ENERGY_ID_CONTROL, 
                        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    );
                    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    let x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("control_accel_x"));
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        x
                    );
                    app::sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        0
                    );
                    app::sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
                    let y = -(WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0));
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        y
                    );
                    app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                    let stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        stable
                    );
                    app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                    let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        brake
                    );
                    app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                    let limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
                    fighter.clear_lua_stack();
                    lua_args!(
                        fighter, 
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        limit
                    );
                    app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
                }
            }
            let motion = if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 
              {Hash40::new("special_air_s_shoot_s")} else {Hash40::new("special_air_s_shoot")};
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
        }
    }

    if !(fighter.global_table[IS_STOPPING].get_bool()) {
        special_s_shoot_helper(fighter);
    }
    
    return 0.into();
}
unsafe fn special_s_shoot_helper(fighter: &mut L2CFighterCommon) {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            let sum_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let mut stop_type = ENERGY_STOP_RESET_TYPE_NONE;
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                stop_type = ENERGY_STOP_RESET_TYPE_GROUND;
            }
            else {
                stop_type = ENERGY_STOP_RESET_TYPE_AIR;
            }
            fighter.clear_lua_stack();
            lua_args!(
                fighter, 
                FIGHTER_KINETIC_ENERGY_ID_STOP, 
                stop_type, 
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                sum_x,
                0.0
            );
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    return;
}

pub fn install() {
    smashline::Agent::new("packun")
        .status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, packun_special_s_shoot_main)
        .install();
}