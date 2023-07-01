use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_final_hold,
        //pre_special_s1_attack,
        special_s1_attack,
        pre_special_s1_end,
        special_s1_end,
        special_s2_dash,
        special_s2_attack,
        pre_special_s2_end,
        special_s2_end,
        pre_chakram_hop,
        pre_special_hi2_rush,
        //exec_special_hi2_rush,
        //exec_special_hi2_rush_end,
        //exec_special_hi3_hold,
        pre_special_hi3_end,
        special_hi3_end,
        //special_lw,
        //miiswordsman_speciallw1hit_main,
        //special_lw3_end,
        special_hi2_bound_end
    );
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_FINAL_HOLD

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_FINAL_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_final_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::WAVE_SPECIAL_N);
    original!(fighter)
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_s1_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let force_air = Vector2f {x: 0.0, y: 10.0};

    PostureModule::add_pos_2d(fighter.module_accessor, &force_air);
    
    original!(fighter)

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

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s1_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_s1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s2_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_dash_mot_change(fighter);
    // if !StopModule::is_stop(fighter.module_accessor) {
    //     WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    // }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s2_dash_dec_int as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_dash_main as *const () as _))
}

unsafe extern "C" fn special_s2_dash_mot_change(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_dash"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_dash"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s2_dash_dec_int(fighter: &mut L2CFighterCommon, unk: L2CValue) -> L2CValue {
    if unk.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_dash_unk(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut val = 0;
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let is_status_cliff = GroundModule::is_status_cliff(fighter.module_accessor);
            if is_status_cliff {
                fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
                val = 1;
            }
        }
    }
    return val.into()
}

unsafe extern "C" fn special_s2_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut sub_check = fighter.sub_transition_group_check_air_cliff().get_bool();
    if sub_check {
        return 0.into();
    }
    sub_check = fighter.sub_ground_check_stop_wall().get_bool();
    if sub_check {
        return 0.into();
    }
    // custom [
    // Jump and Attack cancels
    let pad_flag = ControlModule::get_pad_flag(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && MotionModule::frame(fighter.module_accessor) > 7.0 {
        if fighter.check_jump_cancel(true) {
            return 1.into()
        }
    }
    if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK.into(),true.into());
        return 1.into()
    }
    // Wall Jump
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP) {
            let touch_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            if touch_left || touch_right {
                if compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) {
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP);
                    fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(),true.into());
                    return 1.into()
                }
            }
        }
    }
    // ]
    let slash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if slash_frame <= 0 {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                special_s2_dash_mot_change(fighter);
            }
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                special_s2_dash_mot_change(fighter);
            }
        }
    }
    else {
        special_s2_dash_mot_change(fighter);
    }
    special_s2_dash_unk(fighter);
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s2_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_attack_mot_change(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_attack_main as *const () as _))
}

unsafe extern "C" fn special_s2_attack_mot_change(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_attack"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_attack"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s2_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        special_s2_attack_main_helper(fighter);
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_attack_main_helper(fighter: &mut L2CFighterCommon) {
    if !MotionModule::is_end(fighter.module_accessor) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    special_s2_attack_mot_change(fighter);
                }
            }
            else {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    special_s2_attack_mot_change(fighter);
                }
            }
        }
        special_s2_dash_unk(fighter);
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                if StatusModule::is_changing(fighter.module_accessor) {
                    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                            special_s2_attack_mot_change(fighter);
                        }
                    }
                    else {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                            special_s2_attack_mot_change(fighter);
                        }
                    }
                }
                special_s2_dash_unk(fighter);
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
    }
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
    original!(fighter)
}

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    special_s2_end_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_end_Main as *const () as _))
}

unsafe extern "C" fn special_s2_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if StatusModule::is_changing(fighter.module_accessor) {
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                        // OG [ special_s2_end_helper(fighter); ]
                        // custom [
                        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(true)
                        );
                        // ]
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
                else {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
                        special_s2_end_helper(fighter);
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
            }
            else {
                // custom [
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
                    }
                }
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(true)
                        );
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
                // ]
                special_s2_end_helper(fighter);
            }
            sub_special_s2_end(fighter);
        }
        else {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                    L2CValue::Bool(true)
                );
            }
            else {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(true)
                );
            }
        }
    }
    else {
        if !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into()
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_end_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        // OG [ GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP)); ]
        // custom [
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        // ]
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_NONE));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn sub_special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if GroundModule::is_status_cliff(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END),
                    L2CValue::Bool(true)
                );
                return 1.into()
            }
        }
    }
    return 0.into()
}

// WEAPON_MIISWORDSMAN_CHAKRAM_STATUS_KIND_HOP

#[status_script(agent = "miiswordsman_chakram", status = WEAPON_MIISWORDSMAN_CHAKRAM_STATUS_KIND_HOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_chakram_hop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    VarModule::off_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
    original!(weapon)
}

// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "miiswordsman", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_hi2_rush(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
    original!(fighter)
}

// not running for some reason
#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn exec_special_hi2_rush(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
        //println!("SSD Hit");
    }
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END
// not running for some reason
#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn exec_special_hi2_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        //println!("SSD Success");
        if MotionModule::frame(fighter.module_accessor) >= 30.0 {
            //println!("SSD Fall Act");
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_special_hi3_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let mut motion_value = 0.28;

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        if stick_x != 0.0 {
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f { x: (motion_value * stick_x.signum()), y: 0.0, z: 0.0});
        }
    }
    return 0.into()
}


// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIISWORDSMAN_SPECIAL_HI3_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIISWORDSMAN_SPECIAL_HI3_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIISWORDSMAN_SPECIAL_HI3_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi3_end_Main as *const () as _))
}

unsafe extern "C" fn special_hi3_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut motion_value = 0.7;

    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if frame < 46.0 {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f { x: (motion_value * stick_x.signum()), y: 0.0, z: 0.0});
            }
        }
        if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
            if miisword_situation_helper(fighter).get_bool() {
                if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_END_FLAG_FIRST) {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi3"), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_END_FLAG_FIRST);
                    }
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                    return 0.into()
                }
            }
            if miisword_situation_helper(fighter).get_bool() {
                if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_END_FLAG_FIRST) {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi3"), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi3"), 0.0, 1.0, false, 0.0, false, false);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RSLASH_END_FLAG_FIRST);
                    }
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                    fighter.shift(L2CValue::Ptr(sub_special_hi3_end_Main as *const () as _));
                    return 0.into()
                }
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                        L2CValue::Bool(false)
                    );
                    return 1.into()
                }
            }
        }
    }
    else {
        return 1.into()
    }
    return 0.into()
}

unsafe extern "C" fn sub_special_hi3_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    
    GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_cliff().get_bool() {
                return 1.into()
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) && MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
            if miisword_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                    L2CValue::Bool(true)
                );
                return 1.into()
            }
        }
        else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                L2CValue::Bool(true)
            );
            return 1.into()
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
        if miisword_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                L2CValue::Bool(true)
            );
            return 1.into()
        }
    }
    return 0.into()
}

unsafe extern "C" fn miisword_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            return 1.into()
        }
        if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            return 1.into()
        }
    }
    return 0.into()
}

//FIGHTER_STATUS_KIND_SPECIAL_LW

#[status_script(agent = "miiswordsman", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
    main_setup(fighter);
    let mut l2c_agent = L2CAgent::new(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(1));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1));
    sv_battle_object::notify_event_msc_cmd(lua_state);
    l2c_agent.pop_lua_stack(1);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x3a40337e2c));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1));
    sv_battle_object::notify_event_msc_cmd(lua_state);
    l2c_agent.pop_lua_stack(1);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut num = -1;

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw1") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw1") {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            //println!("Kinesis activation");
            VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
            fighter.change_status(
                L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw3") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw3") {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            //println!("Swordfighter gon' give it to ya");
            fighter.change_status(
                L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND
                || fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                    if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                            main_setup(fighter);
                        }
                    }
                }
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLY_POWERUP_MOTION_RATE) == false {
                    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE) {
                        MotionModule::set_rate(fighter.module_accessor,1.0);
                        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE);
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE) == false {
                        let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_motion_rate"));
                        MotionModule::set_rate(fighter.module_accessor,rate);
                        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE);
                    }
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
                    }
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                    }
                }
                counter_setup(fighter);
                num = 0;
            }
        }
        if num == -1 { num = 1; }
    }
    L2CValue::I32(num)
}

unsafe extern "C" fn main_setup(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT) == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw1"),0.0,1.0,false,0.0,false,false);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor,Hash40::new("special_air_lw1"),-1.0,1.0,0.0);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT) == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw1"),0.0,1.0,false,0.0,false,false);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor,Hash40::new("special_lw1"),-1.0,1.0,0.0);
        }
    }
}

unsafe extern "C" fn counter_setup(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let attack_power = WorkModule::get_float(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_FLOAT_ATTACK_POWER);
        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT {
            return;
        }
        if attack_power > 0.0 {
            let part = (AttackModule::part_size(fighter.module_accessor) as i32) - 1;
            if -1 < part {
                let mut box_num = -1;
                while box_num <= part {
                    if AttackModule::is_attack(fighter.module_accessor,box_num + 1, false) {
                        AttackModule::set_power(fighter.module_accessor,box_num + 1, attack_power, false);
                    }
                    box_num += 1;
                }
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK) == false {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD) == false {
                return;
            }
            ShieldModule::set_status(fighter.module_accessor,0,app::ShieldStatus(*SHIELD_STATUS_NONE),*FIGHTER_MIISWORDSMAN_SHIELD_GROUP_KIND_COUNTER_GUARD);
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD) == false {
                return;
            }
            ShieldModule::set_status(fighter.module_accessor,0,app::ShieldStatus(*SHIELD_STATUS_NORMAL),*FIGHTER_MIISWORDSMAN_SHIELD_GROUP_KIND_COUNTER_GUARD);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK);
        }
    }
}

//FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn miiswordsman_speciallw1hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if StatusModule::situation_kind(fighter.module_accessor) == WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV) {
            return L2CValue::I32(0)
        }
        let speed = KineticModule::get_sum_speed3f(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_GROUND,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
        else {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw1_hit") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw1_hit") {
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP,ENERGY_STOP_RESET_TYPE_AIR,speed.x,0.0,0.0,0.0,0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                let brake_x = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_start_air_acl_x"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP,brake_x,0.0);
                app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed.y,0.0,0.0,0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                let accel_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_acl_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,accel_y);
                app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_max_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,stable_y);
                app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
                WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_AIR,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
            }
            else{
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR,speed.x,0.0,0.0,0.0,0.0);
                //app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                //let brake_x = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_start_air_acl_x"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP,brake_x,0.0);
                //app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                //app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed.y,0.0,0.0,0.0);
                //app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                //let accel_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_acl_y"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY,accel_y);
                //app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_max_y"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,stable_y);
                //app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                //lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                //app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_MOTION);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION);
                WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_AIR,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
            }
        }
    }
    return L2CValue::I32(0)
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) != *SITUATION_KIND_GROUND {
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0x13226e47df),0.0,1.0,false,0.0,false,false);
                if StopModule::is_stop(fighter.module_accessor) == false {
                    some1(fighter,false.into());
                }
                fighter.global_table[0x15].assign(&L2CValue::Ptr(some2 as *const () as _));
                fighter.sub_shift_status_main(L2CValue::Ptr(some3 as *const () as _));
            }
        }
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_AIR {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                some4(fighter);
                KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
                let rate = WorkModule::get_param_float(fighter.module_accessor,0x1018dfb2f4,0x1a840fe7e4);
                MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0x139e9a22d6),0.0,rate,false,0.0,false,false);
                fighter.sub_shift_status_main(L2CValue::Ptr(some5 as *const () as _));
            }
        }
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_AIR {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0x1755e40919),0.0,1.0,false,0.0,false,false);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_WAIT);
                fighter.sub_shift_status_main(L2CValue::Ptr(some6 as *const () as _));
            }
        }
        L2CValue::I32(0)
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let rate = WorkModule::get_param_float(fighter.module_accessor,0x1018dfb2f4,0x1d8867dce8);
        MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0xf9503af02),0.0,rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(some5 as *const () as _))
    }
}

unsafe extern "C" fn some1(fighter: &mut L2CFighterCommon, unk: L2CValue) {
    if unk.get_bool() == false {
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_FALL_ONOFF) == false {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_FALL);
        }
    }
}

unsafe extern "C" fn some2(fighter: &mut L2CFighterCommon, unk2: L2CValue) {
    some1(fighter,unk2);
}

unsafe extern "C" fn some3(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut num = -1;
    if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        num = 0;
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                }
                num = 0;
            }
        }
        if num == -1 { num = 1; }
    }
    L2CValue::I32(num)
}

unsafe extern "C" fn some4(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
}

unsafe extern "C" fn some5(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut num = -1;
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
                num = 0;
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            num = 0;
        }
    }
    if num == -1 { num = 1; }
    L2CValue::I32(num)
}

unsafe extern "C" fn some6(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut num = -1;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                if WorkModule::is_enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL) == false {
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                        if MotionModule::is_end(fighter.module_accessor) == false {
                            num = 0;
                        }
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                    }
                }
                else {
                    if MotionModule::is_end(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                        num = 0;
                    }
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_LANDING_FALL_SPECIAL) {
                            let landing_frame = WorkModule::get_param_int(fighter.module_accessor,0x1018dfb2f4,0x11a96aed54);
                            WorkModule::set_float(fighter.module_accessor,landing_frame as f32,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
                        }
                        some4(fighter);
                        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
                        MotionModule::change_motion(fighter.module_accessor,Hash40::new_raw(0x139e9a22d6),0.0,1.0,false,0.0,false,false);
                        WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL);
                        WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_FALL);
                        WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_WAIT);
                    }
                }
            }
            if num == -1 { num = 1; }
        }
    }
    L2CValue::I32(num)
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND

#[status_script(agent = "miiswordsman", status = FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi2_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    0.into()
}