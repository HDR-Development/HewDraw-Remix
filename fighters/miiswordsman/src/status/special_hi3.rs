use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD

pub unsafe extern "C" fn exec_special_hi3_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub unsafe extern "C" fn pre_special_hi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
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

unsafe extern "C" fn special_hi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi3_end_Main as *const () as _))
}

unsafe extern "C" fn special_hi3_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut motion_value = 0.7;

    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
            if miisword_situation_helper(fighter).get_bool() {
                if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
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

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD, exec_special_hi3_hold);
    
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END, pre_special_hi3_end);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END, special_hi3_end);
}