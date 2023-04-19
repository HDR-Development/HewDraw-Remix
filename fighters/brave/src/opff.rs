use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like down-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn persist_rng(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT) {
        let index = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, index);
    }
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)
    || fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START)
    || fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    }
}

unsafe fn psych_up_crit(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START) {
        if fighter.is_flag(0x200000ea) && fighter.status_frame() == 6 {
            VarModule::on_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 600);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE) {
        if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) <= 0 {
            EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x11be25bbf2), false, false);
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 18, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
        }
        else {
            VarModule::dec_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
        }
        // turn off crits after landing an attack
        if (fighter.is_motion_one_of(&[
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
        ]) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
            EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x11be25bbf2), false, false);
            VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
    }
}

// Hero dash cancel Frizz
unsafe fn dash_cancel_frizz(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_motion(Hash40::new("special_n1"))
    && fighter.motion_frame() > 20.0 && fighter.motion_frame() < 44.0 // after F20 and before the FAF
    && (WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) > 12.0)
    {
        if fighter.check_dash_cancel() {
            let mut brave_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
            FighterSpecializer_Brave::add_sp(&mut brave_fighter, -12.0);
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 15, -2, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// Hero woosh cancel
unsafe fn woosh_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1"), Hash40::new("special_hi_empty"), Hash40::new("special_air_hi_empty")]){
        if MotionModule::frame(fighter.module_accessor) >= 41.0 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }

}

unsafe fn kaclang_jc(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag() {
            fighter.check_jump_cancel(false);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_BRAVE )]
pub unsafe fn brave_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    persist_rng(fighter);
    psych_up_crit(fighter);
    nspecial_cancels(fighter);
    dspecial_cancels(fighter);
    dash_cancel_frizz(fighter);
    woosh_cancel(fighter);
    kaclang_jc(fighter);

    // Extend sword length
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f::new(1.1, 1.05, 1.045));
}