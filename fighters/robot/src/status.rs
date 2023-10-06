use super::*;
use globals::*;


#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
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

    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
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
    }

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::add_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB, 1.0);

    let robotFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);
    let current_fuel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stickX = ControlModule::get_stick_x(fighter.module_accessor);
    let mut rotX = PostureModule::rot_x(fighter.module_accessor, 0);

    //caps angle of rotation
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if (rotX > 60.0) {
            rotX = 60.0;
        } else if (rotX < 0.0) {
            rotX = 0.0;
        }
    } else {
        if (rotX > 86.0) {
            rotX = 86.0;
        } else if (rotX < 0.0) {
            rotX = 0.0;
        }
    }

    //changes direction if rotation crosses threshhold
    if (rotX == 0.0) {
        if (lr == 1.0) &&
        (stickX < -0.1) {
            PostureModule::set_lr(fighter.module_accessor, (lr*-1.0));
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        } else if (lr == -1.0) &&
        (stickX > 0.1) {
            PostureModule::set_lr(fighter.module_accessor, (lr*-1.0));
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }

    //rotates model based off stick direction
    if lr == 1.0 {
        if stickX > 0.1 {
            PostureModule::set_rot(
                fighter.module_accessor,
                &Vector3f::new(rotX+1.5, 0.0, 0.0),
                0
            );
        } else if stickX < -0.1 {
            PostureModule::set_rot(
                fighter.module_accessor,
                &Vector3f::new(rotX-1.5, 0.0, 0.0),
                0
            );
        }
    } else {
        if stickX > 0.1 {
            PostureModule::set_rot(
                fighter.module_accessor,
                &Vector3f::new(rotX-1.5, 0.0, 0.0),
                0
            );
        } else if stickX < -0.1 {
            PostureModule::set_rot(
                fighter.module_accessor,
                &Vector3f::new(rotX+1.5, 0.0, 0.0),
                0
            );
        }
    }

    //determines strength of upb release
    if (current_fuel <= (robotFrames * 2.0)) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::resume_energy_all(fighter.module_accessor);

        let vec = Vector3f{x: (0.05*rotX), y: ((0.05*robotFrames)-(0.025*rotX.abs())), z: 0.0};
        KineticModule::add_speed(fighter.module_accessor, &vec);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

    } else if (current_fuel <= 0.0) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::resume_energy_all(fighter.module_accessor);

        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());
    }

    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::resume_energy_all(fighter.module_accessor);

        if robotFrames < 10.0 {
            let vec = Vector3f{x: (0.05*rotX), y: 1.5-(0.025*rotX.abs()), z: 0.0};
            KineticModule::add_speed(fighter.module_accessor, &vec);
            if (current_fuel - 20.0) > 0.0 {
                WorkModule::set_float(fighter.module_accessor, (current_fuel) - (20.0), *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            } else {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            }

            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));

        } else {
            let vec = Vector3f{x: (0.05*rotX), y: (1.5 + (0.05*robotFrames))-(0.025*rotX.abs()), z: 0.0};
            KineticModule::add_speed(fighter.module_accessor, &vec);
            if ((current_fuel) - (robotFrames * 2.0) > 0.0) {
                WorkModule::set_float(fighter.module_accessor, (current_fuel) - (robotFrames * 2.0), *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            } else {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            }

            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        }

        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

    } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && MotionModule::is_end(fighter.module_accessor) {

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::resume_energy_all(fighter.module_accessor);
        
        let vec = Vector3f{x: (0.05*rotX), y: 3.75-(0.025*rotX.abs()), z: 0.0};
        KineticModule::add_speed(fighter.module_accessor, &vec);

        if ((current_fuel) - (100.0) > 0.0) {
            WorkModule::set_float(fighter.module_accessor, (current_fuel) - (100.0), *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        } else {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        }

        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));

        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

    } else {

    }

    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_keep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_rise"), 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_hi_keep_main_loop)
}

unsafe extern "C" fn special_hi_keep_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::add_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE, 1.0);
    let robotKeepFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE);

    //return to upright
    let rotX = PostureModule::rot_x(fighter.module_accessor, 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    let diff = (rotX/8.0);

    if (robotKeepFrames > 12.0) {
        if lr == 1.0 {
            if rotX > 0.0 {
                PostureModule::set_rot(
                    fighter.module_accessor,
                    &Vector3f::new(rotX-diff, 0.0, 0.0),
                    0
                );
            } else if rotX < 0.0 {
                PostureModule::set_rot(
                    fighter.module_accessor,
                    &Vector3f::new(rotX+diff, 0.0, 0.0),
                    0
                );
            }
        } else {
            if rotX > 0.0 {
                PostureModule::set_rot(
                    fighter.module_accessor,
                    &Vector3f::new(rotX-diff, 0.0, 0.0),
                    0
                );
            } else if rotX < 0.0 {
                PostureModule::set_rot(
                    fighter.module_accessor,
                    &Vector3f::new(rotX+diff, 0.0, 0.0),
                    0
                );
            }
        }
    }

    if (robotKeepFrames > 22.0) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
    } 
    
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_hi_keep_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_hi_keep_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_keep_end(fighter: &mut L2CFighterCommon) -> L2CValue {

    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
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

    if frame == 3.0 {
        if current_situation_kind == *SITUATION_KIND_AIR &&
        WorkModule::is_flag(boma, vars::robot::instance::AIRTIME_SIDEB) {
            
            //let velocity = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            //KineticModule::clear_speed_all(fighter.module_accessor);

            //if velocity >= 0.0 {
            //    let vec = Vector3f{x: 0.05, y: 0.25, z: 0.0};
            //    KineticModule::add_speed(fighter.module_accessor, &vec);
            //} else {
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
                KineticModule::resume_energy_all(fighter.module_accessor);

                let vec = Vector3f{x: 0.35, y: 1.55, z: 0.0};
                KineticModule::add_speed(fighter.module_accessor, &vec);
            //}
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
                KineticModule::resume_energy_all(fighter.module_accessor);

                let vec = Vector3f{x: 0.45, y: 0.0, z: 0.0};
                KineticModule::add_speed(fighter.module_accessor, &vec);
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

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn special_s_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "robot", status = FIGHTER_KINETIC_TYPE_ROBOT_SPECIAL_S_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {

    install_status_scripts!(
        special_hi_main,
        special_hi_end,
        special_hi_exec,
        special_hi_init,

        special_hi_keep_main,
        special_hi_keep_init,
        special_hi_keep_exec,
        special_hi_keep_end,

        special_s_main,
        special_s_init,
        special_s_exec,
        special_s_exec_stop,
        special_s_end,
        special_s_exit,
        special_s_attack_main
    );
}