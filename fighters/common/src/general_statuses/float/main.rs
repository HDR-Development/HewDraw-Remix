use super::*;

#[no_mangle]
unsafe fn float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    float_jump_leniency(fighter);
    float_drift_common(fighter);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);

    let float_duration = VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION);
    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_FRAME, float_duration);
    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ, 1);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_FLOATING);

    if !VarModule::is_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fuwafuwa_start"), 0.0, 1.0, false, 0.0, false, false);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    float_check_aerial(fighter);
    float_ray_check(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(float_main_loop_common as *const () as _))
}

#[no_mangle]
unsafe fn float_check_aerial(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || VarModule::is_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL) {
        if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) != 1 {
            float_set_aerial(fighter);
            return;
        }
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_NONE, FIGHTER_LOG_ACTION_KIND_NONE);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS, 2);
}

#[no_mangle]
unsafe fn float_set_aerial(fighter: &mut L2CFighterCommon) {
    let reflet = fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_REFLET;
    if VarModule::is_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL) {
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        let log = match motion {
            0xc3a4e2597 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N),
            0xc3495ada5 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F),
            0xc33f869bc => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B),
            0xdde67d935 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI),
            0xd40042152 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW),
            _ => None
        };
    
        if let Some(log) = log {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, log);
        }

        VarModule::off_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL);
    }
    else {
        if reflet {
            VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
        }
        let mot = fighter.sub_attack_air_kind_set_log_info();
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot.get_u64()),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    if reflet {
        if let Some(target) = smashline::api::get_target_function("lua2cpp_reflet.nrs", 0x3000) {
            let check_levin: fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(target);
            check_levin(fighter);
        }
        if let Some(target) = smashline::api::get_target_function("lua2cpp_reflet.nrs", 0x28220) {
            let set_levin: fn(&mut L2CFighterCommon) = std::mem::transmute(target);
            set_levin(fighter);
        }
    }

    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_CANCEL_UNABLE_CANCEL);
    sv_module_access::cancel(fighter.lua_state_agent);
    let _ = fighter.pop_lua_stack(1);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS, 1);
}

#[no_mangle]
unsafe fn float_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_mewtwo = fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_MEWTWO;
    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as i32;
    let float_duration = VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION);
    let float_frame = VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME); // dec at end of loop

    if (float_frame - float_duration).abs() == 4
    && VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT) {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FREE,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
        float_drift_common(fighter);
    }

    // Mewtwo jump burn
    if (float_frame - float_duration).abs() == buffer
    && is_mewtwo {
        fighter.inc_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // if StatusModule::is_changing(fighter.module_accessor)
    // && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_AIR
    // && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
    //     FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    //     float_set_aerial(fighter);
    //     return 0.into();
    // }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 2
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 1
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 2
    && float_frame <= 0 {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 1
    && float_frame <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    // Unique Down Special stuff is usually here but we don't want it

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ) == 1 {
        if float_frame > 0
        && !StopModule::is_stop(fighter.module_accessor) {
            VarModule::dec_int(fighter.battle_object, vars::common::status::FLOAT_FRAME);
            if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME) == 0 {
                VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ, 0);
            }
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING) {
            fighter.sub_transition_group_check_air_landing();
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
            fighter.sub_transition_group_check_air_special();
        }

        let mut check_aerial = false;

        if !StatusModule::is_changing(fighter.module_accessor) {
            let mtrans = VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS);

            if mtrans == 1 {
                if MotionModule::is_end(fighter.module_accessor) {
                    check_aerial = true;
                }
                if CancelModule::is_enable_cancel(fighter.module_accessor) {
                    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                        check_aerial = true;
                    }
                }
            }
            else if mtrans == 2 {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                    check_aerial = true;
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fuwafuwa"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }

        if check_aerial {
            if fighter.sub_transition_group_check_air_lasso().get_bool() {
                return 1.into();
            }
            float_check_aerial(fighter);
        }
    }
    0.into()
}

#[no_mangle]
unsafe fn float_drift_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let float_stable = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_float.float_stable");
    let float_accel_add = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_float.float_accel_add");
    let float_accel_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_float.float_accel_mul");
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, float_stable, float_stable);
    lua_bind::FighterKineticEnergyController::set_accel_x_add(fighter.get_controller_energy(), float_accel_add);
    lua_bind::FighterKineticEnergyController::set_accel_x_mul(fighter.get_controller_energy(), float_accel_mul);
    lua_bind::FighterKineticEnergyController::set_accel_y_add(fighter.get_controller_energy(), float_accel_add);
    lua_bind::FighterKineticEnergyController::set_accel_y_mul(fighter.get_controller_energy(), float_accel_mul);
    0.into()
}

#[no_mangle]
unsafe fn float_jump_leniency(fighter: &mut L2CFighterCommon) -> L2CValue {
    let float_jump_aerial_refresh_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "float_jump_aerial_refresh_frame");
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_JUMP_AERIAL
    && fighter.global_table[PREV_STATUS_FRAME].get_i32() <= float_jump_aerial_refresh_frame {
        fighter.dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_jump_aerial"), true, true);
        smash::app::sv_animcmd::EFFECT_OFF_KIND(fighter.lua_state_agent);
    }
    0.into()
}

unsafe fn float_ray_check(fighter: &mut L2CFighterCommon) {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let result = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_check = GroundModule::ray_check_hit_pos(
        fighter.module_accessor,
        &Vector2f{x: pos_x, y: pos_y},
        &Vector2f{x: 0.0, y: -6.0},
        result,
        true
    );
    if ray_check {
        let ray_check_y = result.y;
        // let uniq_float_start_y = WorkModule::get_float(fighter.module_accessor, hash40("param_private"), hash40("uniq_float_start_y"));
        let mut uniq_float_start_y = 1.5;
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT) {uniq_float_start_y += 0.5}
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
    }
}