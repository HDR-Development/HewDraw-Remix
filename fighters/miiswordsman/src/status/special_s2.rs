use super::*;

pub unsafe extern "C" fn special_s2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

pub unsafe extern "C" fn special_s2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05) + -1);
    sub_special_s2_main(fighter);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);

    fighter.main_shift(special_s2_main_loop)
}

unsafe extern "C" fn special_s2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        VarModule::set_float(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_SPEED_Y, sum_speed_y);
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH.into(), true.into());
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        sub_special_s2_main(fighter);
    }
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        if GroundModule::is_status_cliff(fighter.module_accessor) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
            return 1.into();
        }
    }

    return 0.into();
}

unsafe fn sub_special_s2_main(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_start"), 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_start"), 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
}

pub unsafe extern "C" fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_miiswordsman_special_s01"), 0);
    return 0.into();
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH

unsafe extern "C" fn special_s2_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
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

unsafe extern "C" fn special_s2_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_s2_dash_substatus(fighter, false.into());
    }
    VarModule::set_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_GROUND_START, fighter.is_situation(*SITUATION_KIND_GROUND));
    special_s2_dash_change_motion(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s2_dash_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_dash_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_dash_change_motion(fighter: &mut L2CFighterCommon) {
    let mot = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        Hash40::new("special_s2_dash")
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let sum_speed_y = VarModule::get_float(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_SPEED_Y);
        let stable_speed_y = fighter.get_param_float("air_speed_y_stable", "");
        let accel_y = fighter.get_param_float("air_accel_y", "");
        let min_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_s2.min_speed_y_mul");
        let max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_s2.max_speed_y_mul");
        let speed_y = sum_speed_y.clamp(-(sum_speed_y.abs()) * min_mul, stable_speed_y * max_mul);
        let mul_str = if speed_y >= 0.0 { "special_s2.accel_y_mul_up" } else { "special_s2.accel_y_mul_down" };
        let accel_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, mul_str);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y * accel_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        Hash40::new("special_air_s2_dash")
    };
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
    }
}

unsafe extern "C" fn special_s2_dash_substatus(fighter: &mut L2CFighterCommon, unk: L2CValue) -> L2CValue {
    if unk.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    }
    return 0.into();
}

unsafe extern "C" fn special_s2_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool()
    || fighter.sub_ground_check_stop_wall().get_bool() {
        return 0.into();
    }
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if dash_count <= 0 {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s2_dash_change_motion(fighter);
    }
    special_s2_main_loop_helper(fighter);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
        let shield_hit_end_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_s2.shield_hit_end_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, shield_hit_end_speed_x * lr, 0.0);
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK.into(), false.into());
        return 0.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_s2_main_loop_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        let cont = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && GroundModule::is_status_cliff(fighter.module_accessor) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
            true
        }
        else {
            false
        };
        if cont {
            return 0.into();
        }
    }
    1.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK

pub unsafe extern "C" fn special_s2_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
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
    
    return 0.into();
}

unsafe extern "C" fn special_s2_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_attack_mot_change(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_attack_main_loop as *const () as _))
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
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_GROUND_START) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_attack"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s2_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool()) {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("miiswordsman_counter_arc"), true, true);
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                let landing_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "special_s2.landing_lag");
                fighter.check_land_cancel(Some(landing_lag));
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
    return 0.into();
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END

unsafe extern "C" fn special_s2_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END)(fighter)
}

unsafe extern "C" fn special_s2_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    special_s2_end_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP));
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
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
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
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
                    }
                }
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP));
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
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
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
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP));
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
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP));
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

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, special_s2_dash_pre);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, special_s2_dash_main);

    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, special_s2_attack_pre);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, special_s2_attack_main);
    
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, special_s2_end_pre);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, special_s2_end_main);
}