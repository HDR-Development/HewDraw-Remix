use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 

// FIGHTER_STATUS_KIND_ATTACK_AIR //

pub unsafe extern "C" fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

// FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL //

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        if fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
        } // jump cancel
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    let status = fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
    if status == *STATUS_KIND_NONE || status == *FIGHTER_STATUS_KIND_GUARD_ON {fighter.set_int(*FIGHTER_STATUS_KIND_WAIT,*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS); }
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_cancel"), -1.0, 1.0, 0.0, false, false);
    }
    else if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.set_int(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_cancel"), -1.0, 1.0, 0.0, false, false);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let status = fighter.get_int(*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            fighter.change_status(L2CValue::I32(status).into(), false.into());
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }    
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
        else {fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into()); }
    }
    return 0.into();
}

// FIGHTER_STATUS_KIND_SPECIAL_S //

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
        fighter.on_flag(*FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY);
    }
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

pub fn install() {
    smashline::Agent::new("mewtwo")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air)
        .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
        .install();
}