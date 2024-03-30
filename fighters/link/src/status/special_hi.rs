use super::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_HI

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        let start_speed = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let start_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_x_mul");
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed * start_x_mul);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        0,
        0,
        0,
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rslash_charge_spd = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));

    MotionModule::set_rate(fighter.module_accessor, rslash_charge_spd);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
            fighter.change_status_req(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, false);
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD) {
            if MotionModule::is_end(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD),
                    L2CValue::Bool(true)
                );
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END),
                    L2CValue::Bool(false)
                );
                return 1.into();
            }
        }
    }
    else if !link_situation_helper(fighter).get_bool() || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if !link_situation_helper(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
                return 0.into();
            }
        }
        else if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
                return 0.into();
            }
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
            GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            }
            return fighter.fastshift(L2CValue::Ptr(sub_specialhi_Main as *const () as _));
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
    }
    0.into()
}

unsafe extern "C" fn sub_specialhi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD) {
        if MotionModule::is_end(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD),
                L2CValue::Bool(true)
            );
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                L2CValue::Bool(false)
            );
        }
        else {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X) {
                    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI {
                        let rslash_end_air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_accel_x_mul"));
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI, rslash_end_air_accel_x_mul);
                        app::sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
                        let rslash_air_max_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));
                        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                        let rslash_end_air_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_max_x"));
                        let mul_x_speed_max = ((rslash_end_air_max_x / air_speed_x_stable) / rslash_air_max_x_mul);
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, mul_x_speed_max);
                        app::sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
                        return 0.into()
                    }
                }
                return 0.into()
            }
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                L2CValue::Bool(false)
            );
        }
    }
    0.into()
}

// FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD

pub unsafe extern "C" fn special_hi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_hi_hold(fighter, L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi_hold as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_hold_main_loop as *const () as _))
}

unsafe extern "C" fn sub_special_hi_hold(fighter: &mut L2CFighterCommon, arg2: L2CValue) -> L2CValue {
    if !arg2.get_bool() {
        return 0.into();
    }
    let rslash_charge_spd_div = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));
    WorkModule::add_float(fighter.module_accessor, 1.0 / rslash_charge_spd_div, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let rslash_hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame"));
    if !MotionModule::is_end(fighter.module_accessor) {
        if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
            if frame < rslash_hold_frame_max as f32 {
                return 0.into();
            }
        }
    }
    fighter.change_status_req(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, true);
    1.into()
}

unsafe extern "C" fn special_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, false);
        return 1.into();
    }
    0.into()
}

// FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END //

pub unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT,
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

pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut motion_value = 0.55;

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if link_situation_helper(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if !StatusModule::is_changing(fighter.module_accessor) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                        L2CValue::Bool(false)
                    );
                    return 1.into()
                }
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                fighter.shift(L2CValue::Ptr(sub_specialhi_end_Main as *const () as _));
                return 0.into()
            }
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
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn sub_specialhi_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);

    GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if link_situation_helper(fighter).get_bool() {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                        L2CValue::Bool(true)
                    );
                    return 1.into()
                }
            }
            else {
                if fighter.sub_transition_group_check_air_cliff().get_bool() {
                    return 1.into()
                }
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
        if link_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                L2CValue::Bool(true)
            );
            return 1.into()
        }
    }
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, special_hi_hold_main);
    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
}