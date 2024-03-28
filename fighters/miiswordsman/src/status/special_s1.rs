use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK

unsafe extern "C" fn pre_special_s1_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let force_air = Vector2f {x: 0.0, y: 10.0};

    PostureModule::add_pos_2d(fighter.module_accessor, &force_air);
    
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK)(fighter)

}

/*
#[status_script(agent = "miiswordsman", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn miiswordsman_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    //l2c_agent.clear_lua_stack();
    //l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683 as u64))
    lua_args!(fighter, 0x20cbc92683, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 - 1);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    l2c_agent.clear_lua_stack();
    lua_args!(fighter, 0x3a40337e2c, FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 - 1);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_specials1_main as *const () as _))

}
*/

unsafe extern "C" fn miiswordsman_specials1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) == true {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), false.into());
        return L2CValue::I32(1);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        fighter.set_situation(SITUATION_KIND_AIR.into());
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {

    }

    L2CValue::I32(0)
}

unsafe extern "C" fn special_s1_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT); // Removes on side special attack
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
    }
    // Commented out but this was the original script
    // if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
    //     GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    //     fighter.set_situation(SITUATION_KIND_AIR.into());
    // }
    // else {
    //     GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    //     fighter.set_situation(SITUATION_KIND_GROUND.into());
    // }
    // But HERE's what we (probably) want
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(SITUATION_KIND_AIR.into());
    // Now for whatever reason they have these change motions in the same conditions as the previous set, so I'm keeping it separate anyway.
    // You can *probably* get away with not impling the ground version but eh safety first
    //if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
    //  MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 0.0, 1.0, false, 0.0, false, false);
    //}
    //else {
    //  MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 0.0, 1.0, false, 0.0, false, false);
    //}

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
      MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
      MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1"), 0.0, 1.0, false, 0.0, false, false);
    }

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    //let x_vel = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
    let x_vel_flip = ControlModule::get_stick_x(fighter.module_accessor) * 1.65 + (0.1 * PostureModule::lr(fighter.module_accessor));
    let y_vel_flip = 3.5;
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, x_vel_flip, y_vel_flip);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    /*
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s1") {
        println!("ground side special"); 
    }
    */

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 1.75, -1.5);
    app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, -0.1);
    app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

    // But HERE's what we (probably) want
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_specials1attack_mainloop as *const () as _))
}

unsafe extern "C" fn miiswordsman_specials1attack_mainloop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // custom [
    // drift at apex of flip
    if MotionModule::frame(fighter.module_accessor) > 7.0 && MotionModule::frame(fighter.module_accessor) < 32.0 {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        if stick_x != 0.0 {
            let drift_speed = 0.15 * stick_x.signum();
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, drift_speed, 0.0);
            app::sv_kinetic_energy::add_speed(fighter.lua_state_agent);
            // this is here so adding speed doesn't surpass x speed cap we set earlier (by setting a slightly lower speed cap)
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, (1.75 - drift_speed.abs()), -1.5);
            app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 1.75, -1.5);
            app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= 15.0 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        if compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return 1.into()
        }
        if (compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && fighter.is_stick_forward())
            || (compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && fighter.is_stick_forward()) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return 1.into()
        }
        if (compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) && fighter.is_stick_forward())
            || (compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) && fighter.is_stick_forward()) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return 1.into()
        }
        if compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return 1.into()
        }
        if compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return 1.into()
        }
    }
    // ]
    
    // This is just (as straight of) an impl that I can get of the main loop, what the hell were the smash devs
    // Can only use side special once per aerial time
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) == false {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
                return L2CValue::I32(1);
            }
            return L2CValue::I32(0);
        }
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
    }
    L2CValue::I32(0)
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END

pub unsafe extern "C" fn pre_special_s1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_SOUND
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s1_end_Main as *const () as _))
}

unsafe extern "C" fn special_s1_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
            if !MotionModule::is_end(fighter.module_accessor) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_HENSOKU_SLASH_WORK_FLAG_END_LANDING) {
                    if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND {
                        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                            return 1.into();
                        }
                    }
                }
                if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                                StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
                                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s1_end"), -1.0, 1.0, 0.0, false, false);
                                let s1_control_limit_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("s1_control_limit_mul_x"));
                                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                                fighter.clear_lua_stack();
                                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
                                smash::app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                                fighter.clear_lua_stack();
                                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, s1_control_limit_mul_x);
                                smash::app::sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);
                                return 1.into();
                            }
                        }
                    }
                    else {
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s1_end"), -1.0, 1.0, 0.0, false, false);

                    }
                }
                else {
                    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                            return 1.into();
                        }
                    }
                }
                return 0.into()
            }
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            if !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return 1.into()
                }
            }
        }
    }
    return 1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK, pre_special_s1_attack);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK, special_s1_attack);
    
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END, pre_special_s1_end);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END, special_s1_end);
}