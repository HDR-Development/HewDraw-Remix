use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END

unsafe extern "C" fn special_lw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) != *SITUATION_KIND_GROUND {
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw3_end_air"),0.0,1.0,false,0.0,false,false);
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
                let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw3_end_air_to_ground_rate"));
                MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw3_end"),0.0,rate,false,0.0,false,false);
                fighter.sub_shift_status_main(L2CValue::Ptr(some5 as *const () as _));
            }
        }
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_AIR {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw3_end_air"),0.0,1.0,false,0.0,false,false);
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
        let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw3_end_ground_to_ground_rate"));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw3_end"),0.0,rate,false,0.0,false,false);
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
                            let landing_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_lw"),hash40("lw3_landing_frame"));
                            WorkModule::set_float(fighter.module_accessor,landing_frame as f32,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
                        }
                        some4(fighter);
                        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
                        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw3_end"),0.0,1.0,false,0.0,false,false);
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, special_lw3_end);
}