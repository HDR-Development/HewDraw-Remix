use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn jab_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK)
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }

    if fighter.is_cat_flag(Cat1::AttackS3) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::AttackHi3) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        return;
    }
  
    if fighter.is_cat_flag(Cat1::AttackLw3) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        return;
    }

    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialN) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialS) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_HI, false);
    }
}

unsafe fn dash_attack_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialS) {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_HI, false);
    }
}

unsafe fn tilt_cancels(fighter: &mut L2CFighterCommon) {
    // Level 2: Tilt Cancels
    if !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3
    ])
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    {
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialN) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialS) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        return;
    }
    
    // The tilt -> smash attacks is only valid for non-dtilt
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_LW3) {
        return;
    }
    
    if fighter.is_cat_flag(Cat1::AttackS4) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
        return;
    }
    
    if fighter.is_cat_flag(Cat1::AttackHi4) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
        return;
    }

    if fighter.is_cat_flag(Cat1::AttackLw4) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
        return;
    }
}

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F])
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    {
        return;
    }

    if fighter.is_input_jump()
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
    {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialN) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);    
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialS) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
        return;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        return;
    }

    if !fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F) {
        return;
    }

    match fighter.get_aerial() {
        Some(AerialKind::Fair) | None => return,
        _ => {}
    }

    fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
}

unsafe fn special_cancels(fighter: &mut L2CFighterCommon) {
    // Special Cancels
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.motion_frame() > 33.0
    {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_cat_flag(Cat1::AirEscape)
    {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA )]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    jab_cancels(fighter);
    dash_attack_cancels(fighter);
    tilt_cancels(fighter);
    aerial_cancels(fighter);
    special_cancels(fighter);
    nspecial_cancels(fighter);
}