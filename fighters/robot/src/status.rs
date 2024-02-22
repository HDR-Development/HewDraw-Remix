use super::*;
use globals::*;

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
    *FIGHTER_STATUS_KIND_DAMAGE_AIR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
    *FIGHTER_STATUS_KIND_DAMAGE_FALL];

    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let prev_status_kind_2 = StatusModule::prev_status_kind(fighter.module_accessor, 1);
    
    VarModule::set_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB, 0.0);
    VarModule::set_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE, 0.0);

    if damage_statuses.contains(&prev_status_kind) || damage_statuses.contains(&prev_status_kind_2) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            
            //KineticModule::suspend_energy_all(fighter.module_accessor);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable * 0.2
            );

            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                0.0
            );
    
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            
            VarModule::on_flag(fighter.battle_object, vars::robot::instance::GROUNDED_UPB);
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
            PostureModule::add_pos(fighter.module_accessor, &Vector3f{x: 0.00, y: 3.0, z: 0.0});
            
            //KineticModule::suspend_energy_all(fighter.module_accessor);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                0.0
              );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable * 0.2
            );
        }
    } else {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::suspend_energy_all(fighter.module_accessor);
        KineticModule::unable_energy_all(fighter.module_accessor);
        let air_brake = sv_fighter_util::get_default_fighter_param_air_brake_x(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            air_brake,
            0.0
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::robot::instance::GROUNDED_UPB);
            
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
            PostureModule::add_pos(fighter.module_accessor, &Vector3f{x: 0.00, y: 3.0, z: 0.0});
            
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::suspend_energy_all(fighter.module_accessor);
            KineticModule::unable_energy_all(fighter.module_accessor);
            let air_brake = sv_fighter_util::get_default_fighter_param_air_brake_x(fighter.lua_state_agent);
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                air_brake,
                0.0
            );
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
    }

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let robotFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB);
    let current_fuel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stickX = ControlModule::get_stick_x(fighter.module_accessor);
    let mut rotX = PostureModule::rot_x(fighter.module_accessor, 0);
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    VarModule::add_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB, 1.0);

    //caps angle of rotation
    if (rotX > 86.0) {
        rotX = 86.0;
    } else if (rotX < 0.0) {
        rotX = 0.0;
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

      /*if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if lr == 1.0 {
            if stickX > 0.1 {
                VarModule::add_float(fighter.battle_object, vars::robot::instance::JOINT_ROT, 1.5);
                let jointX = VarModule::get_float(fighter.battle_object, vars::robot::instance::JOINT_ROT);
                rotation = Vector3f{x: 0.0, y: 0.0, z: jointX};
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist1"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else if stickX < -0.1 {
                VarModule::sub_float(fighter.battle_object, vars::robot::instance::JOINT_ROT, 1.5);
                let jointX = VarModule::get_float(fighter.battle_object, vars::robot::instance::JOINT_ROT);
                rotation = Vector3f{x: 0.0, y: 0.0, z: jointX};
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist1"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
        } else {
            if stickX > 0.1 {
                VarModule::sub_float(fighter.battle_object, vars::robot::instance::JOINT_ROT, 1.5);
                let jointX = VarModule::get_float(fighter.battle_object, vars::robot::instance::JOINT_ROT);
                rotation = Vector3f{x: 0.0, y: 0.0, z: jointX};
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist1"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else if stickX < -0.1 {
                VarModule::add_float(fighter.battle_object, vars::robot::instance::JOINT_ROT, 1.5);
                let jointX = VarModule::get_float(fighter.battle_object, vars::robot::instance::JOINT_ROT);
                rotation = Vector3f{x: 0.0, y: 0.0, z: jointX};
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist1"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
        }
    } else { */
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
    /*}

    if rotX == 0.0 {
        rotX = VarModule::get_float(fighter.battle_object, vars::robot::instance::JOINT_ROT);
    }*/

    //determines strength of upb release
    if (current_fuel <= (robotFrames * 2.0)) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy_all(fighter.module_accessor);

        let vec = Vector3f{x: (0.05*rotX.abs()), y: ((0.05*robotFrames)-(0.025*rotX.abs())), z: 0.0};
        KineticModule::add_speed(fighter.module_accessor, &vec);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

    } else if (current_fuel <= 0.0) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy_all(fighter.module_accessor);

        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());
    }

    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy_all(fighter.module_accessor);

        if (current_fuel <= (robotFrames * 2.0)) {
            let vec = Vector3f{x: (0.05*rotX.abs()), y: ((0.05*robotFrames)-(0.025*rotX.abs())), z: 0.0};
            KineticModule::add_speed(fighter.module_accessor, &vec);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_s"));
            
            fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());
    
        } else if robotFrames < 10.0 {
            let vec = Vector3f{x: (0.05*rotX.abs()), y: 0.5-(0.025*rotX.abs()), z: 0.0};
            KineticModule::add_speed(fighter.module_accessor, &vec);
            if (current_fuel - 20.0) > 0.0 {
                WorkModule::set_float(fighter.module_accessor, (current_fuel) - (20.0), *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            } else {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            }

            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));

        } else {
            let vec = Vector3f{x: (0.05*rotX.abs()), y: (1.5 + (0.05*robotFrames))-(0.025*rotX.abs()), z: 0.0};
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
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy_all(fighter.module_accessor);
        
        let vec = Vector3f{x: (0.05*rotX.abs()), y: 3.75-(0.025*rotX.abs()), z: 0.0};
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

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::resume_energy_all(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_keep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_rise"), 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_hi_keep_main_loop)
}

unsafe extern "C" fn special_hi_keep_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::add_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE, 1.0);
    let robotKeepFrames = VarModule::get_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE);

    /*if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }*/

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

    /*Cancels status transition if button is being held
    if robotKeepFrames == 17.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || 
        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::off_flag(fighter.module_accessor, vars::robot::status::HELD_BUTTON);
        }
    }

    if (robotKeepFrames > 18.0) &&
    WorkModule::is_flag(fighter.module_accessor, vars::robot::status::HELD_BUTTON) {
        if fighter.is_button_on(Buttons::Attack) || fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::Guard) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
    } */

    if VarModule::is_flag(fighter.battle_object, vars::robot::instance::UPB_CANCEL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        VarModule::off_flag(fighter.battle_object, vars::robot::instance::UPB_CANCEL);
    }

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
    }
    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    } 

    0.into()
}

unsafe extern "C" fn special_hi_keep_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_keep_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_keep_end(fighter: &mut L2CFighterCommon) -> L2CValue {

    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    //KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::resume_energy_all(fighter.module_accessor);
    0.into()
}

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

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_s_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    smashline::Agent::new("robot")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init)
        .status(
            Main,
            *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP,
            special_hi_keep_main,
        )
        .status(
            Init,
            *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP,
            special_hi_keep_init,
        )
        .status(
            Exec,
            *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP,
            special_hi_keep_exec,
        )
        .status(
            End,
            *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP,
            special_hi_keep_end,
        )
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec)
        .status(
            ExecStop,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            special_s_exec_stop,
        )
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end)
        .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exit)
        .status(
            Main,
            *FIGHTER_KINETIC_TYPE_ROBOT_SPECIAL_S_ATTACK,
            special_s_attack_main,
        )
        .install();
}
