use super::*;

unsafe extern "C" fn check_special_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
    ];
    let is_enables = terms.map(|term| WorkModule::is_enable_transition_term(fighter.module_accessor, term));
    fighter.enable_transition_term_many(&terms);
    let ret = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.sub_transition_group_check_ground_special()
    } else {
        fighter.sub_transition_group_check_air_special()
    };
    for i in 0..terms.len() {
        if !is_enables[i] {
            fighter.unable_transition_term(terms[i]);
        }
    }
    return ret;
}

unsafe extern "C" fn check_final_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL,
    ];
    let is_enables = terms.map(|term| WorkModule::is_enable_transition_term(fighter.module_accessor, term));
    fighter.enable_transition_term_many(&terms);
    let ret = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.sub_transition_group_check_ground_special()
    } else {
        fighter.sub_transition_group_check_air_special()
    };
    for i in 0..terms.len() {
        if !is_enables[i] {
            fighter.unable_transition_term(terms[i]);
        }
    }
    return ret;
}

// FIGHTER_STATUS_KIND_ATTACK_LW4

unsafe extern "C" fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
    fighter.set_int(0, *FIGHTER_RYU_STATUS_ATTACK_INT_FRAME);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);

    fighter.status_AttackLw4_common();
    fighter.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_lw4_main_loop as *const () as _))
}

unsafe extern "C" fn attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) 
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) 
        && check_final_cancel(fighter).get_bool() {
            return true.into();
        }
        if fighter.is_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) 
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) 
        && check_special_cancel(fighter).get_bool() {
            return true.into();
        }
        let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_private"),
            hash40("attack_start_cancel_frame"),
        );
        if current_frame <= attack_start_cancel_frame
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) 
        && ryu_kara_cancel(fighter).get_bool() {
            return true.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let lw4_combo_max = fighter.get_param_int("lw4_combo_max", "");
        if ComboModule::count(fighter.module_accessor) < lw4_combo_max as u64
        && fighter.is_cat_flag(Cat1::AttackN)
        && fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_lw4_mtrans();
        }
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return false.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return false.into();
    }

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main);
}