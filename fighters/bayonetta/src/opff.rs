use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn dash_attack_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || fighter.is_in_hitlag()
    || VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS) {
        return;
    }

    let mut new_status = 0;
    let mut is_input_cancel = false;

    if fighter.is_cat_flag(Cat1::SpecialS) {
        is_input_cancel = true;
        new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        is_input_cancel = true;
        new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
    }

    if is_input_cancel {
        if !fighter.is_in_hitlag(){
            if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S {
                StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
            }
            fighter.change_status_req(new_status, false);
        }
    }
}

unsafe fn tilt_cancels(fighter: &mut L2CFighterCommon) {
    // Level 2: Tilt Cancels
    if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S3)
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS)
    {
        return;
    }
    if !fighter.is_motion(Hash40::new("attack_s3_s3")) {
        return;
    }
    let mut new_status = 0;
    let mut is_input_cancel = false;

    if fighter.is_cat_flag(Cat1::SpecialN) {
        is_input_cancel = true;
        new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
    }

    if fighter.is_cat_flag(Cat1::SpecialS) {
        is_input_cancel = true;
        new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
    }

    if fighter.is_cat_flag(Cat1::SpecialHi) {
        is_input_cancel = true;
        new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
    }

    if is_input_cancel {
        if !fighter.is_in_hitlag(){
            if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S {
                StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
            }
            fighter.change_status_req(new_status, false);
        }
    }
}

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME, 0);
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME, 0);
    }

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !fighter.is_motion_one_of(&[Hash40::new("attack_air_n_hold"), Hash40::new("attack_air_f"), Hash40::new("attack_air_f_hold"), Hash40::new("attack_air_f2_hold"), Hash40::new("attack_air_f3_hold"), Hash40::new("attack_air_hi_hold"), Hash40::new("attack_air_lw_hold")])
    && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS)
    {
        let mut new_status = 0;
        let mut is_input_cancel = false;
        if fighter.is_input_jump()
        && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
        {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_JUMP_AERIAL;
        }

        if fighter.is_cat_flag(Cat1::SpecialN) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        }

        if fighter.is_cat_flag(Cat1::SpecialS) {
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME) <= 2 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        }

        if fighter.is_cat_flag(Cat1::SpecialHi) {
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME) <= 0 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        }

        if is_input_cancel {
            if !fighter.is_in_hitlag(){
                //disable fair1 special/attack cancel
                // if fighter.is_motion(Hash40::new("attack_air_f")) {
                //     if new_status != *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                //         return;
                //     }
                // }
                // disable dair jump cancel
                if fighter.is_motion(Hash40::new("attack_air_lw")) {
                    if new_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                        return;
                    }
                }

                if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S {
                    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME);
                }
                else if new_status == *FIGHTER_STATUS_KIND_SPECIAL_HI  {
                    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
                }
                VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
                fighter.change_status_req(new_status, false);
                return;
            }
        }
        
        if fighter.is_motion(Hash40::new("attack_air_f2")) {
            match fighter.get_aerial() {
                Some(AerialKind::Fair) | None => return,
                _ => {
                    if !fighter.is_in_hitlag() {
                        VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
                        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                        return;
                    }
                }
            }
            return;
        }
    }
}

unsafe fn special_cancels(fighter: &mut L2CFighterCommon) {
    // Special Cancels
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.motion_frame() > 31.0
    && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS)
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

unsafe fn recovery_resource_management(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
    }
    else{
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL]){
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
            }
        }
        else{
            VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
    }
    
}

unsafe fn clear_proration(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if (fighter.is_prev_situation(*SITUATION_KIND_AIR) && fighter.is_situation(*SITUATION_KIND_GROUND)) || 
        (fighter.is_prev_situation(*SITUATION_KIND_AIR) && fighter.is_situation(*SITUATION_KIND_CLIFF)) ||
        fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL]) && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL) {
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
        }
}

unsafe fn abk_flight_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) && fighter.motion_frame() < 25.0 && !StopModule::is_stop(fighter.module_accessor){
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        if stick_y != 0.0 && !fighter.is_in_hitlag(){
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new(0.0, 1.0 * stick_y, 0.0));
        }
    }
    
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA )]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    if let Some(info) = FrameInfo::update_and_get(fighter) {
        //dash_attack_cancels(fighter);
        tilt_cancels(fighter);
        aerial_cancels(fighter);
        special_cancels(fighter);
        nspecial_cancels(fighter);
        recovery_resource_management(fighter);
        abk_flight_drift(fighter);
        clear_proration(fighter, info.boma);
    }

    
}