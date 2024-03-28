use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let current_situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let boma = fighter.module_accessor;

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    //momentum hop code
    /*if frame == 3.0 {
        if current_situation_kind == *SITUATION_KIND_AIR &&
        WorkModule::is_flag(boma, vars::robot::instance::AIRTIME_SIDEB) {
            
            let velocity = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            KineticModule::clear_speed_all(fighter.module_accessor);

            if velocity >= 0.0 {
                let vec = Vector3f{x: 0.05, y: 0.25, z: 0.0};
                KineticModule::add_speed(fighter.module_accessor, &vec);
            } else {
                KineticModule::clear_speed_all(fighter.module_accessor);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                KineticModule::unable_energy_all(fighter.module_accessor);
                let ground_brake = sv_fighter_util::get_default_fighter_param_ground_brake(fighter.lua_state_agent);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    ground_brake,
                    0.0
                );
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::resume_energy_all(fighter.module_accessor);

                let vec = Vector3f{x: 0.35, y: 1.55, z: 0.0};
                KineticModule::add_speed(fighter.module_accessor, &vec);
            }
        } else if current_situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::clear_speed_all(fighter.module_accessor);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                KineticModule::unable_energy_all(fighter.module_accessor);
                let ground_brake = sv_fighter_util::get_default_fighter_param_ground_brake(fighter.lua_state_agent);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    ground_brake,
                    0.0
                );
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::resume_energy_all(fighter.module_accessor);

                let vec = Vector3f{x: 0.45, y: 0.0, z: 0.0};
                KineticModule::add_speed(fighter.module_accessor, &vec);
        }
    }*/

    if frame >= 12.0 {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        /*} else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);*/
        }
    }

    if frame > 15.0 
    && frame <= 18.0 {
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        if stick_y > 0.3 {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_hi"), -1.0, 1.0, 0.0, false, false);
                PLAY_SE(fighter, Hash40::new("se_robot_special_s03"));
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_hi"), -1.0, 1.0, 0.0, false, false);
                PLAY_SE(fighter, Hash40::new("se_robot_special_s03"));
            }
        } else if stick_y < -0.3 {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_lw"), -1.0, 1.0, 0.0, false, false);
                PLAY_SE(fighter, Hash40::new("se_robot_special_s04"));
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_lw"), -1.0, 1.0, 0.0, false, false);
                PLAY_SE(fighter, Hash40::new("se_robot_special_s04"));
            }
        } else {
            PLAY_SE(fighter, Hash40::new("se_robot_special_s02"));
        }
    }

    if frame == 18.0 && 
    current_situation_kind == *SITUATION_KIND_AIR &&
    WorkModule::is_flag(boma, vars::robot::instance::AIRTIME_SIDEB) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        app::sv_kinetic_energy::enable(fighter.lua_state_agent);
        
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }

    if frame == 19.0 {
        WorkModule::off_flag(fighter.module_accessor, vars::robot::instance::AIRTIME_SIDEB);
    }

    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
    } 
    
    0.into()
}

unsafe extern "C" fn stub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Main, *FIGHTER_KINETIC_TYPE_ROBOT_SPECIAL_S_ATTACK, stub_status);
}