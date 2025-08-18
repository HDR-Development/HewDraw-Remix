use super::*;

pub unsafe extern "C" fn special_n1_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.change_motion_by_situation("special_n1_hold", "special_air_n1_hold", 0.0, 1.0, false, 0.0, false, false);
    fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n1_hold_main_loop)
}

pub unsafe extern "C" fn special_n1_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_pad_flag(PadFlag::SpecialTrigger)
    || fighter.is_pad_flag(PadFlag::AttackTrigger) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE.into(), false.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS).into()).get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
    }
    else {
        if fighter.is_cat_flag(Cat1::AirEscape) {
            fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS).into()).get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), true.into());
            return 0.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.change_motion_inherit_frame_by_situation("special_n1_hold", "special_air_n1_hold", -1.0, 1.0, 0.0, false, false);
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.inc_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
        if charge_count as f32 >= n1_cshot_charge_frame {
            fighter.set_int(n1_cshot_charge_frame as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
            fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL.into(), false.into());
            return 1.into();
        }
        else {
            if fighter.is_pad_flag(PadFlag::SpecialTrigger)
            || fighter.is_pad_flag(PadFlag::AttackTrigger) {
                fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE.into(), false.into());
                return 1.into();
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x322bb60c04), charge_count as f32 / n1_cshot_charge_frame);
        }
    }
    else {
        let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x322bb60c04), charge_count as f32 / n1_cshot_charge_frame);
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n1_fire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_count = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_COUNT);
    let n1_cshot_charge_frame = fighter.get_param_float("param_special_n", "n1_cshot_charge_frame");
    if (charge_count as f32) < n1_cshot_charge_frame {
        let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            Hash40::new("special_n1_fire")
        }
        else {
            Hash40::new("special_air_n1_fire")
        };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let motion = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                Hash40::new("special_n1_neon")
            }
            else {
                Hash40::new("special_air_n1_neon")
            }
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                Hash40::new("special_n1_fire_max")
            }
            else {
                Hash40::new("special_air_n1_fire_max")
            }
        };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    fighter.set_int(0, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 as i32 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    fighter.main_shift(special_n1_fire_main_loop)
}

pub unsafe extern "C" fn special_n1_fire_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, *GROUND_CORRECT_KIND_AIR);
        if (fighter.is_motion(Hash40::new("special_n1_fire")) || fighter.is_motion(Hash40::new("special_air_n1_fire"))) {
            fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_fire", "special_air_n1_fire", -1.0, 1.0, 0.0);
        }
        else if (fighter.is_motion(Hash40::new("special_n1_fire_max")) || fighter.is_motion(Hash40::new("special_air_n1_fire_max"))) {
            fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_fire_max", "special_air_n1_fire_max", -1.0, 1.0, 0.0);
        }
        else {
            fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_neon", "special_air_n1_neon", -1.0, 1.0, 0.0);
        }
    }

    return 0.into();
}

// pub unsafe extern "C" fn special_n1_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let cancel_status = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
//     if fighter.is_situation(*SITUATION_KIND_GROUND) {
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n1_cancel"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     else {
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//         let motion = if cancel_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
//             Hash40::new("special_air_n1_jump_cancel")
//         } else if cancel_status == *FIGHTER_STATUS_KIND_FLY {
//             Hash40::new("special_air_n1_jump_cancel")
//         } else {
//             Hash40::new("special_air_n1_cancel")
//         };
//         MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
//     }
//     if cancel_status == *STATUS_KIND_NONE {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32)
//     }
//     ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    
//     fighter.main_shift(special_n1_cancel_main_loop)
// }

// pub unsafe extern "C" fn special_n1_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let cancel_status = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
//     fighter.sub_check_charge_cancel_jump_mini_attack();
//     if fighter.is_situation(*SITUATION_KIND_GROUND) {
//         if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
//         && cancel_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
//             FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
//         }
//     }
//     if cancel_status != *STATUS_KIND_NONE {
//         // if !CancelModule::is_enable_cancel(fighter.module_accessor) {
//         //     if !MotionModule::is_end(fighter.module_accessor) {
//         //         // goto
//         //     }
//         // }
//         if cancel_status == *FIGHTER_STATUS_KIND_GUARD_ON {
//             //if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {}
//             fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
//             return 0.into();
//         }
//     }
//     if CancelModule::is_enable_cancel(fighter.module_accessor) {
//         if fighter.sub_wait_ground_check_common(false.into()).get_bool()
//         || !fighter.sub_air_check_fall_common().get_bool() {
//             return 0.into();
//         }
//     }
//     if MotionModule::is_end(fighter.module_accessor) {
//         fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
//         return 0.into();
//     }
//     if StatusModule::is_situation_changed(fighter.module_accessor) {
//         if fighter.is_situation(*SITUATION_KIND_GROUND) {
//             let motion = if fighter.kind == *FIGHTER_KIND_KIRBY { Hash40::new("miigunner_special_air_n1_jump_cancel") } else { Hash40::new("special_air_n1_jump_cancel") };
//             if MotionModule::motion_kind(fighter.module_accessor) == motion {
//                 fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
//                 return 0.into();
//             } 
//             GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1_cancel"), -1.0, 1.0, 0.0, false, false);
//             fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
//         }
//         else {
//             GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1_cancel"), -1.0, 1.0, 0.0, false, false);
//             fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
//         }
//     }

//     return 0.into();
// }

pub unsafe extern "C" fn special_n3_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_n3_loop(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n3_loop as *const () as _));
    fighter.main_shift(special_n3_loop_main_loop)
}

unsafe extern "C" fn sub_special_n3_loop(fighter: &mut L2CFighterCommon, param: L2CValue) -> L2CValue {
    if param.get_bool() {
        fighter.inc_int(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_WORK_INT_HOLD_COUNT);
    }
    // else {
    //     if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //         fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
    //         fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    //     }
    //     else {
    //         let hold_count = fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_WORK_INT_HOLD_COUNT);
    //         let max_hold = fighter.get_param_int("param_special_n", "n3_hold_max_frame");
    //         if hold_count > max_hold {
    //             fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_MAX);
    //             fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
    //             fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    //         }
    //     }
    // }

    return 0.into();
}

pub unsafe extern "C" fn special_n3_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3_loop"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST);
            };
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n3_loop"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_FIRST);
            };
        }
    }
    //special_n3_charge(fighter);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //|| fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_GRENADE_LAUNCHER_FLAG_MAX) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END.into(), true.into());
        return 1.into();
    }
    
    return 0.into();
}

// unsafe extern "C" fn special_n3_charge(fighter: &mut L2CFighterCommon) {
//     let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
//     let mut charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_n3_charge_start");
//     let mut charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_n3_charge_end");
//     let mut max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.max_charge_frames");
//     if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < max_charge_frames
//     && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//         let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
//         MotionModule::set_rate(fighter.module_accessor, motion_rate);
//         VarModule::set_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE, charge + 1.0);
//     }
//     else {
//         VarModule::set_float(fighter.battle_object, vars::miigunner::instance::SPECIAL_N3_CHARGE, charge);
//         MotionModule::set_rate(fighter.module_accessor, 1.0);
//     }
// }

pub unsafe extern "C" fn special_n3_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD, special_n1_hold_main);
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, special_n1_fire_main);

    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_LOOP, special_n3_loop_main);
    agent.status(Exec, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END, special_n3_end_exec);
}