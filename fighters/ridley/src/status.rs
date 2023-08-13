use super::*;
use globals::*;

/// Hold neutral special to explode
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_shoot_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_fire_num"))
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
    }else {
        original!(fighter)
    }
}
unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
        special_n_air_to_ground_transition(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
unsafe extern "C" fn special_n_air_to_ground_transition(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

// Normalizes side b landing lag to be based on remaining aerial lag
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_failure_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cancel_frame = (FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_start"), false) - MotionModule::frame(fighter.module_accessor)) + WorkModule::get_param_int(fighter.module_accessor, hash40("landing_heavy_frame"), 0) as f32 + 5.0;
    if cancel_frame < 1.0 {
        VarModule::set_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME, 1.0);
    }
    else {
        VarModule::set_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME, cancel_frame);
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_failure"), 0.0, 1.0, false, 0.0, false, false);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("failure_speed_x_mul"));

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(side_special_failure_main_loop as *const () as _))
}
pub unsafe fn side_special_failure_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    else if VarModule::get_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME) <= MotionModule::frame(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    false.into()
}

/// Pogo down special
#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_GRAB) {
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_GRAB);
        original!(fighter)
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH);
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING);
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_LANDING);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
    }
}
unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_LANDING) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into()
        }
        else if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return true.into()
        }
        else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return true.into()
            }
        }
    }
    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_LANDING) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_pogo_landing"), 0.0, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_LANDING);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return true.into()
        }
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    else {
        special_lw_pogo_bounce_check(fighter, VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE));
    }
    false.into()
}
unsafe extern "C" fn special_lw_pogo_bounce_check(fighter: &mut L2CFighterCommon, check_hit: bool) {
    let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("tail8"), v3f_tail_pos, false);
    let pos_x_global = PostureModule::pos_x(fighter.module_accessor);
    let pos_y_global = PostureModule::pos_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stored_pos = VarModule::get_vec2(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_BOUNCE_PREV_POS);

    let pos_x_prev = stored_pos.x;
    let pos_y_prev = stored_pos.y;

    // save current tail position relative to fighter
    VarModule::set_vec2(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_BOUNCE_PREV_POS, Vector2f{x: v3f_tail_pos.x - pos_x_global, y: v3f_tail_pos.y - pos_y_global});

    if check_hit {
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(
            // checks for ground between previous tail pos and current tail pos, saves collision pos to "ground_hit_pos"
            fighter.module_accessor,
            &Vector2f{x: pos_x_prev + pos_x_global, y: pos_y_prev + pos_y_global},
            &Vector2f{x: (v3f_tail_pos.x - (pos_x_prev + pos_x_global)) + (8.0 * lr), y: v3f_tail_pos.y - (pos_y_prev + pos_y_global) - 8.0},
            ground_hit_pos,
            true
        ) == 1 {
            // deduces angle of slope for effects by using 2 Vector2f's and trigonometry
            let mut slope_angle = 0.0;
            let slope_check_pos = &mut Vector2f{x: 0.0, y: 0.0};
            if GroundModule::ray_check_hit_pos(
                fighter.module_accessor,
                &Vector2f{x: ground_hit_pos.x + (5.0 * lr), y: ground_hit_pos.y + 5.0},
                &Vector2f{x: 0.0, y: -10.0 },
                slope_check_pos,
                true
            ) == 1 {
                let pos_diff_y = ground_hit_pos.y - slope_check_pos.y;
                if pos_diff_y > 0.0 {
                    slope_angle = (pos_diff_y / 5.0).atan().to_degrees();
                }
                else {
                    slope_angle = 360.0 -((-pos_diff_y / 5.0).atan().to_degrees());
                }
            }
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), (ground_hit_pos.x - pos_x_global) * lr, ground_hit_pos.y - pos_y_global, 0, slope_angle, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), (ground_hit_pos.x - pos_x_global) * lr, ground_hit_pos.y - pos_y_global, 0, slope_angle, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);

            PLAY_SE(fighter, Hash40::new("se_ridley_special_h03"));
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);

            let bounce_vel_max = 2.5;
            let bounce_vel_min = 0.0;
            let bounce_vel_mul = 0.04; //bigger number = less velocity
            let mut velocity_y = (-bounce_vel_mul * (pos_y_global - ground_hit_pos.y)) + bounce_vel_max; // calculates bounce height based off distance from ground
            if pos_y_global -ground_hit_pos.y <= 0.0 {
                velocity_y = bounce_vel_max;
            }
            else if velocity_y < bounce_vel_min {
                velocity_y = bounce_vel_min;
            }
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            SET_SPEED_EX(fighter, velocity_x*0.5, velocity_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            // hitting a hurtbox gives set momentum
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            SET_SPEED_EX(fighter, velocity_x*0.5, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_ENABLE_BOUNCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
}

// Air horizontal stab
// #[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_AIR_LASSO, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn attack_air_lasso_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     VarModule::on_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_GRAB);
//     fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
//     return true.into()
// }

pub fn install() {
    smashline::install_status_scripts!(
        special_n_shoot_status_main,
        special_lw_status_main,
        special_s_failure_status_main,
        //attack_air_lasso_status_main,
    );
}
