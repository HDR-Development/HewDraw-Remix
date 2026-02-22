use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like down-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn persist_rng(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT) {
        let index = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, index);
    }
}

unsafe fn psych_up_crit(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE) {
        if VarModule::countdown_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0) {
            EFFECT_OFF_KIND(fighter, Hash40::new("brave_charge_hold"), false, false);
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 18, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
        }
        // turn off crits after landing an attack
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && fighter.is_motion_one_of(&[
            Hash40::new("attack_13"),
            Hash40::new("attack_s3_s2"),
            Hash40::new("attack_hi3"),
            Hash40::new("attack_dash"),
            Hash40::new("attack_s4_s"),
            Hash40::new("attack_hi4"),
            Hash40::new("attack_lw4"),
            Hash40::new("attack_air_n"),
            Hash40::new("attack_air_f"),
            Hash40::new("attack_air_b"),
            Hash40::new("attack_air_lw")
        ]) {
            EFFECT_OFF_KIND(fighter, Hash40::new("brave_charge_hold"), false, false);
            VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
}

unsafe fn kaclang_jc(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag() {
            fighter.check_jump_cancel(false, false);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK1,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK2,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK3,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK3_APPEND,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_HOLD,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL,
        *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe extern "C" fn brave_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    persist_rng(fighter);
    psych_up_crit(fighter);
    dspecial_cancels(fighter);
    kaclang_jc(fighter);
    fastfall_specials(fighter);

    // Extend sword length
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f::new(1.1, 1.05, 1.045));
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, brave_frame_wrapper);
}