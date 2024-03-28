use super::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_N

pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
                        special_n_helper(fighter);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), false, -1.0);
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
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), false, -1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
                }
                fighter.fastshift(L2CValue::Ptr(sub_special_air_n as *const () as _))
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), false, -1.0);
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
            fighter.fastshift(L2CValue::Ptr(special_n_main_loop as *const () as _))
        }
        else {
            fighter.fastshift(L2CValue::Ptr(special_n_main_loop as *const () as _))
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
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
                        special_n_helper(fighter);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), false, -1.0);
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
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), false, -1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
                }
                fighter.fastshift(L2CValue::Ptr(sub_special_n as *const () as _))
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), false, -1.0);
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
}