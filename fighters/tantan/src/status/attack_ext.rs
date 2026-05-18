use super::*;

// 71000186b0
unsafe extern "C" fn attack_ext_exec_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if let Some(func_ptr) = smashline::api::get_target_function("lua2cpp_tantan.nrs", 0x186b0) {
        let exec_inner: fn(&mut L2CValue, &mut L2CFighterCommon) -> L2CValue = std::mem::transmute(func_ptr);
        exec_inner(&mut L2CValue::U64(0), fighter);
    }

    return 0.into();
}

unsafe fn set_transition_terms(fighter: &mut L2CFighterCommon, some_bool: bool) {
    if some_bool {
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
    else {
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
}

unsafe fn check_recoil_cancel(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL) { return; }
    if fighter.is_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_IS_CANCEL) {
        VarModule::off_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL);
        return;
    }

    let mut new_status = 0;
    if fighter.is_cat_flag(Cat1::AttackS4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
    } else if fighter.is_cat_flag(Cat1::AttackHi4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
    } else if fighter.is_cat_flag(Cat1::AttackLw4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
    } else if fighter.is_cat_flag(Cat1::AttackS3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
    } else if fighter.is_cat_flag(Cat1::AttackHi3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
    } else if fighter.is_cat_flag(Cat1::AttackLw3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
    } else if fighter.is_cat_flag(Cat1::AttackN) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK;
    }

    if (new_status > 0) {
        //DamageModule::add_damage(fighter.module_accessor, new_status as f32, 0);
        VarModule::off_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_pad_flag(PadFlag::JumpTrigger) {
                new_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
            }
        }
        else {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_AIR;
        }
        VarModule::set_int(fighter.battle_object, vars::tantan::status::RECOIL_CANCEL_STATUS, new_status);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
}

unsafe extern "C" fn attack_ext_default_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_ext_exec_inner(fighter);
    check_recoil_cancel(fighter);

    return 0.into();
}

unsafe extern "C" fn attackfall_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_ext_exec_inner(fighter);
    fighter.sub_fall_uniq_process_exec();
    check_recoil_cancel(fighter);

    return 0.into();
}

unsafe extern "C" fn attackjumpsquat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

unsafe extern "C" fn attackwalk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

unsafe extern "C" fn attackwalkback_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BACK)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

unsafe extern "C" fn attacklanding_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

unsafe extern "C" fn attacklandinglight_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

unsafe extern "C" fn attacksquat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    check_recoil_cancel(fighter);

    return 0.into();
}

unsafe extern "C" fn attacksquatwait_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT)(fighter);
    check_recoil_cancel(fighter);

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL, attackfall_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL_AERIAL, attackfall_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT, attackjumpsquat_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, attacklanding_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT, attacklandinglight_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, attacksquat_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, attacksquatwait_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, attackwalk_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BACK, attackwalkback_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE, attack_ext_default_exec);
}