use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || VarModule::is_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_GRAB) {
        VarModule::off_flag(fighter.battle_object, vars::ridley::instance::SPECIAL_LW_IS_GRAB);
        smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    }
    else {
        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);

            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
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
        ) {
            // deduces angle of slope for effects by using 2 Vector2f's and trigonometry
            let mut slope_angle = 0.0;
            let slope_check_pos = &mut Vector2f{x: 0.0, y: 0.0};
            if GroundModule::ray_check_hit_pos(
                fighter.module_accessor,
                &Vector2f{x: ground_hit_pos.x + (5.0 * lr), y: ground_hit_pos.y + 5.0},
                &Vector2f{x: 0.0, y: -10.0 },
                slope_check_pos,
                true
            ) {
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}