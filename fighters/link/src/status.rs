use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_specialhi,
        //specialhi,
        pre_specialhi_end,
        specialhi_end,
        //special_n
    );
}

// FIGHTER_STATUS_KIND_SPECIAL_HI //


#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi(fighter: &mut L2CFighterCommon, arg: u64) -> L2CValue {
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_Main as *const () as _))
}

unsafe extern "C" fn specialhi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rslash_charge_spd = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));

    MotionModule::set_rate(fighter.module_accessor, rslash_charge_spd);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
            GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            fighter.fastshift(L2CValue::Ptr(sub_specialhi_Main as *const () as _))
        }
        else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD) {
            if MotionModule::is_end(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD),
                    L2CValue::Bool(true)
                );
                return 1.into()
            }
            else {
                return 0.into()
            }
        }
        else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END),
                    L2CValue::Bool(false)
                );
                return 1.into()
            }
            else {
                return 0.into()
            }
        }
        else {
            return 0.into()
        }
    }
    else if !link_situation_helper(fighter).get_bool() || (link_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND) {
        if !link_situation_helper(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
                return 0.into()
            }
            return 0.into()
        }
        else if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
                return 0.into()
            }
            return 0.into()
        }
        else if StatusModule::is_changing(fighter.module_accessor) {
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
            fighter.fastshift(L2CValue::Ptr(sub_specialhi_Main as *const () as _))
        }
        else {
            return 0.into()
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
        return 0.into()
    }
}

unsafe extern "C" fn sub_specialhi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
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
    }
    return 1.into()
}

unsafe extern "C" fn link_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
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


// FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END //


#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_end_Main as *const () as _))
}

unsafe extern "C" fn specialhi_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if frame < 46.0 {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f { x: (motion_value * stick_x.signum()), y: 0.0, z: 0.0});
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


// FIGHTER_STATUS_KIND_SPECIAL_N //


#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_START, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_SHOOT_NUM);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_DOUBLE_COUNT);
    let max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    if max_hold_frame < 0 {
        WorkModule::set_int(fighter.module_accessor, -100, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_KIRBY {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW) {
            let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
            if copy_chara == *FIGHTER_KIND_LINK {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
            }
        }
    }
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_LINK {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_LINKARROW {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, -1);
            }
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_Main as *const () as _))
}

unsafe extern "C" fn special_n_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        let cat2 = fighter.global_table[0x21].get_int() as i32;
        let is_not_fastfall = cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0;
        if is_not_fastfall && fighter.global_table[STICK_Y].get_f32() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if !MotionModule::is_end(fighter.module_accessor) || (MotionModule::is_end(fighter.module_accessor) && fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND) {
                if MotionModule::is_end(fighter.module_accessor) {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(false)
                        );
                        return 1.into()
                    }
                }
                else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(false)
                        );
                        return 1.into()
                    }
                }
                else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING),
                            L2CValue::Bool(false)
                        );
                        return 1.into()
                    }
                }
                //goto
                else if !link_situation_helper(fighter).get_bool() || (link_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND) {
                    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                        if link_situation_helper(fighter).get_bool() {
                            sub_special_air_n_Main(fighter);
                            sub_special_n_Main_uniq(fighter);
                            return 0.into()
                        }
                    }
                }
                else {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                        sub_special_n_Main(fighter);
                    }
                }
                sub_special_n_Main_uniq(fighter);
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL),
                        L2CValue::Bool(false)
                    );
                    return 1.into()
                }
            }
        }
        else {
            if !link_situation_helper(fighter).get_bool() || (link_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND) {
                if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                    if link_situation_helper(fighter).get_bool() {
                        sub_special_air_n_Main(fighter);
                        sub_special_n_Main_uniq(fighter);
                        return 0.into()
                    }
                }
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                    sub_special_n_Main(fighter);
                }
            }
            sub_special_n_Main_uniq(fighter);
            return 0.into()
        }
    }
    return 0.into()
}

unsafe extern "C" fn sub_special_air_n_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);

    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        let cat2 = fighter.global_table[0x21].get_int() as i32;
        let is_not_fastfall = cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0;
        if is_not_fastfall && fighter.global_table[STICK_Y].get_f32() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::I32(1)
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_START {
            if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_END  {
                    return L2CValue::I32(0)
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END) {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x91355f0c9), true, -1.0);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
                        special_n_helper(fighter);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x91355f0c9), false, -1.0);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END);
                    }
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                    }
                    fighter.fastshift(L2CValue::Ptr(sub_special_air_n_end as *const () as _))
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x5306f402c), true, -1.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x5306f402c), false, -1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
                }
                fighter.fastshift(L2CValue::Ptr(sub_special_air_n as *const () as _))
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xb7af226d2), true, -1.0);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xb7af226d2), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START);
            }
            fighter.fastshift(L2CValue::Ptr(sub_special_air_n_start as *const () as _))
        }
    }
    else {
        return L2CValue::I32(0)
    }
}

unsafe extern "C" fn sub_special_air_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        let cat2 = fighter.global_table[0x21].get_int() as i32;
        let is_not_fastfall = cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0;
        if is_not_fastfall && fighter.global_table[STICK_Y].get_f32() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        sub_special_n_end_uniq(fighter);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
                else {
                    return 0.into()
                }
            }
        }
    }
    else {
        return 0.into()
    }
}

unsafe extern "C" fn sub_special_n_end_uniq(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
        let mut second_bowarrow_interval_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("second_bowarrow_interval_frame"));
        let mut bow_double_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_DOUBLE_COUNT);
        if bow_double_count == second_bowarrow_interval_frame {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), true);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
            ItemModule::remove_item(fighter.module_accessor, 0);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
            app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(0);
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            }
            else {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            }
        }
    }
}

unsafe extern "C" fn sub_special_air_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        let cat2 = fighter.global_table[0x21].get_int() as i32;
        let is_not_fastfall = cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0;
        if is_not_fastfall && fighter.global_table[STICK_Y].get_f32() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    let mut bow_max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    let mut max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) || max_hold_frame <= bow_max_hold_count {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                    return 0.into()
                }
                else {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
            }
        }
    }
    else {
        fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
    }
}

unsafe extern "C" fn sub_special_air_n_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        let cat2 = fighter.global_table[0x21].get_int() as i32;
        let is_not_fastfall = cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0;
        if is_not_fastfall && fighter.global_table[STICK_Y].get_f32() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
        if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
            }
            else {
                if !link_situation_helper(fighter).get_bool() {
                    return 0.into()
                }
                else {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                        fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                    }
                    else {
                        return 0.into()
                    }
                }
            }
        }
        else {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
                else {
                    return 0.into()
                }
            }
        }
    }
}

unsafe extern "C" fn sub_special_n_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);

    if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
        }
        else if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_HOLD, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
            if fighter.global_table[FIGHTER_KIND].get_i32() != *FIGHTER_KIND_KIRBY {
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new_raw(0x298585bf83));
                app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                fighter.pop_lua_stack(0);
            }
            else {
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new_raw(0x2ff4ab9a31));
                app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                fighter.pop_lua_stack(0);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX);
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
        }
        else {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
        }
    }
    else if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
        if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_END  {
            fighter.fastshift(L2CValue::Ptr(special_n_Main as *const () as _))
        }
        else {
            fighter.fastshift(L2CValue::Ptr(special_n_Main as *const () as _))
        }
    }
    else {
        let mut bow_max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
        let mut max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
        if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if max_hold_frame <= bow_max_hold_count {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
                fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
            }
            else {
                fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common_uniq as *const () as _))
        }
    }
}

unsafe extern "C" fn sub_special_n_common_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
            sub_special_n_Main_uniq(fighter);
            return 0.into()
        }
        else {
            sub_special_air_n_Main(fighter);
        }
    }
    else {
        sub_special_n_Main(fighter);
    }
    sub_special_n_Main_uniq(fighter);
    return 0.into()
}

unsafe extern "C" fn sub_special_n_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::I32(1)
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_START {
            if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                if bow_step != *FIGHTER_LINK_STATUS_BOW_STEP_END {
                    return L2CValue::I32(0)
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END) {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x58cf3cb66), true, -1.0);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
                        special_n_helper(fighter);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x58cf3cb66), false, -1.0);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END);
                    }
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
                    }
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_end as *const () as _))
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x17808a3d2), true, -1.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x17808a3d2), false, -1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
                }
                fighter.fastshift(L2CValue::Ptr(sub_special_n as *const () as _))
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x7e266f076), true, -1.0);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x7e266f076), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START);
            }
            fighter.fastshift(L2CValue::Ptr(sub_special_n_start as *const () as _))
        }
    }
    else {
        return L2CValue::I32(0)
    }
}

unsafe extern "C" fn sub_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        sub_special_n_end_uniq(fighter);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
                else {
                    return 0.into()
                }
            }
        }
    }
    else {
        return 0.into()
    }
}

unsafe extern "C" fn sub_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bow_max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    let mut max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) || max_hold_frame <= bow_max_hold_count {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_AIR {
                    return 0.into()
                }
                else {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
            }
        }
    }
    else {
        fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
    }
}

unsafe extern "C" fn sub_special_n_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
        if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
            }
            else {
                if !link_situation_helper(fighter).get_bool() {
                    return 0.into()
                }
                else {
                    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                        fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                    }
                    else {
                        return 0.into()
                    }
                }
            }
        }
        else {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
        }
        else {
            if !link_situation_helper(fighter).get_bool() {
                return 0.into()
            }
            else {
                if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                    fighter.fastshift(L2CValue::Ptr(sub_special_n_common as *const () as _))
                }
                else {
                    return 0.into()
                }
            }
        }
    }
}

unsafe extern "C" fn sub_special_n_Main_uniq(fighter: &mut L2CFighterCommon) {
    let mut bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
        let mut bow_charge_spd_div = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("bow_charge_spd_div"));
        let mut bow_bend_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("bow_bend_rate"));
        MotionModule::set_rate(fighter.module_accessor, (bow_bend_rate / bow_charge_spd_div));
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
    }
}

unsafe extern "C" fn special_n_helper(fighter: &mut L2CFighterCommon) {
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_FIRST), true);
}
