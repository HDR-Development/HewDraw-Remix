use super::*;

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D

unsafe extern "C" fn special_air_s_d_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_DABK_COUNT);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S);
    if fighter.is_prev_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 5.0, 1.0, false, 0.0, false, false);
    } else { //removed qc input
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_d"), 0.0, 1.0, false, 0.0, false, false);
    }
    bounce_check(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_d_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_d_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    bounce_check(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        set_lag(fighter); 
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        } else {
            let special_lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); 
            let dabk_slide_lag = special_lag.max(fighter.get_param_int("param_special_s", "ab_d_landing_frame") as f32); //solid 30f landing lag.
            fighter.set_float(dabk_slide_lag, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); 
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING.into(), false.into());
        }
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn special_air_s_d_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {set_lag(fighter); }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U

unsafe extern "C" fn special_air_s_u_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_u"), 0.0, 1.0, false, 0.0, false, false);
    fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_s_u_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_s_u_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
        if fighter.get_param_int("param_special_s", "ab_u_disable_landing_frame") <= frame {
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
        }
    }
    if frame < 8 {
        cache_input(fighter);
        if frame == 7 && fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D) {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D.into(), false.into());
        }//dabk
    }    
    if (CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_air_check_fall_common().get_bool())
    || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK) {wall_check(fighter); }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        set_lag(fighter);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    } else {
        bullet_movement(fighter);
        angling(fighter);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_air_s_u_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    set_lag(fighter);
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP) {
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLAG_SITUATION_KEEP);
    }
    0.into()
}

unsafe extern "C" fn bounce_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) 
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ATTACK) {
            VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT.into(), false.into());
    } else {
         wall_check(fighter);
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
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && AttackModule::is_attack(fighter.module_accessor, 0, false) { //checks if hitbox cleared to prevent double dipping
            VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn cache_input(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D) {
        if fighter.is_button_on(Buttons::Attack | Buttons::Catch) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, false);
            fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D);
            VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE, -1.15); //angle forced down during dabk windup
        } else {
            VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE, fighter.left_stick_y());
        } //angle if no dabk
    }
    0.into()
}

unsafe extern "C" fn angling(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let facing = PostureModule::lr(fighter.module_accessor);
    let sticky = VarModule::get_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_S_ABK_ANGLE);
    joint_rotator(fighter, frame, Hash40::new("rot"), Vector3f{x: -14.5*sticky, y:0.0, z:0.0}, 1.0, 12.0, 31.0, 40.0);
    if fighter.global_table[CURRENT_FRAME].get_i32() == 7 {
        let base = fighter.get_param_float("param_special_s", "ab_u_rotate");
        let speed = fighter.get_param_float("param_special_s", "ab_u_motion_speed_mul");
        let maxrot = 13.0;
        let angle = if facing < 0.0 {
            -base - sticky *maxrot //l
        } else {
            base + sticky *maxrot //r
        };
        let angle = angle.to_radians();
        sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed - sticky*0.08); //min .71, max .87
    }
    0.into()
}

unsafe extern "C" fn bullet_movement(fighter: &mut L2CFighterCommon) -> L2CValue { //was like 400 lines
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                let x_reset = fighter.get_float(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X);
                let y_reset = fighter.get_float(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_Y);
                let initial_x = fighter.get_param_float("param_special_s", "ab_u_shooting_speed_x_mul");
                let initial_y = fighter.get_param_float("param_special_s", "ab_u_shooting_speed_y_mul");
                let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy; //272
                let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy; //288
                let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::KineticEnergy;
                //motion to stop energy
                lua_bind::KineticEnergy::reset_energy(stop_energy as _, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f { x: x_reset * initial_x, y: 0.0 }, &Vector3f::zero(), fighter.module_accessor);
                lua_bind::KineticEnergyNormal::set_accel(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: 0.0, y: 0.0 });
                lua_bind::KineticEnergyNormal::set_brake(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_brake_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_stable_speed(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x"), y: 0.0 });
                lua_bind::KineticEnergyNormal::set_limit_speed(stop_energy as *mut app::KineticEnergyNormal, &Vector2f { x: -1.0, y: -1.0 });
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                //motion to gravity
                lua_bind::KineticEnergy::reset_energy(gravity_energy as _, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f { x: 0.0, y: initial_y }, &Vector3f::zero(), fighter.module_accessor);
                lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy as *mut app::FighterKineticEnergyGravity, -fighter.get_param_float("param_special_s", "ab_u_shooting_accel_y"));
                lua_bind::FighterKineticEnergyGravity::set_stable_speed(gravity_energy as *mut app::FighterKineticEnergyGravity, fighter.get_param_float("param_special_s", "ab_u_shooting_max_speed_y"));
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
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
            let speed = Vector2f{
                x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
                y: lua_bind::KineticEnergy::get_speed_y(stop_energy)
            };
            let x_cap = fighter.get_param_float("param_special_s", "ab_u_shooting_stable_speed_x");
            if speed.x.abs() <= x_cap {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP);
            }
        }
    }
    0.into()
}

unsafe fn joint_rotator(fighter: &mut L2CFighterCommon, frame: f32, joint: Hash40, rotation_amount: Vector3f, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let lua_state = fighter.lua_state_agent;
    let max_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_x_rotate = max_rotation.x * (frame / (bend_frame - start_frame));
        let calc_y_rotate = max_rotation.y * (frame / (bend_frame - start_frame));
        let calc_z_rotate = max_rotation.z * (frame / (bend_frame - start_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.module_accessor, joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_rotation.x *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_y_rotate = max_rotation.y *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_z_rotate = max_rotation.z *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.module_accessor, joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

unsafe fn set_lag(fighter: &mut L2CFighterCommon) { 
    //vanilla: if special lag variable < lag to be set from current status, skips it to keep the higher number (the problem w whiff lag). Multiplies special lag by landing frame mul then sets it over lag variable (not sure if applicable here but idk)
    let resources = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT) as f32;
    let dabk = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_DABK_COUNT) as f32; //lag added to base abk lag
    let abk_total_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) as f32;
    let witch_twist_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) as f32;
    let whiff_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.whiff_lag"); //6
    let dabk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.dive_side_special");//9
    let abk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.side_special");//6
    let witch_twist_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.up_special");//6
    let base_lag: f32 = 8.0;
    let special_landing_frame_mul = fighter.get_param_float("special_landing_frame_mul", "");
    //normal special lag calc 
    //reworked from hard coded value based on move order (contrived) ->
    //calculate all burned resources, and add a base value. 14 -> 50 range
    let special_lag = (resources*whiff_lag)+(dabk*dabk_lag)+(abk_total_count*abk_lag)+(witch_twist_count*witch_twist_lag)+base_lag;
    //after lag frames decided
    let adjusted_special_lag = special_landing_frame_mul * special_lag;
    if adjusted_special_lag < 1.0 {let adjusted_special_lag = 1.0;} //vanilla
    //if abk_total_count + witch_twist_count > 0 && current_lag <= autocancel_lag {
    //    let adjusted_special_lag = autocancel_lag;//if lag was cleared via vanilla tech, set it to base value and leave it
    //} concept to keep the autocancel if you use another special after firing bullets, since it'd be really niche since she's a fastfaller. idrk
    fighter.set_float(adjusted_special_lag, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); 
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, special_air_s_d_main);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, special_air_s_d_end);

    agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, special_air_s_u_main);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, special_air_s_u_end);
}
