use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn soaring_slash_drift(fighter: &mut L2CFighterCommon) {
    let stick_x = fighter.stick_x();
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2])
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && stick_x != 0.0
    {
        KineticModule::add_speed_outside(
            fighter.module_accessor,
            *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION,
            &Vector3f::new(0.2 * stick_x.signum(), 0.0, 0.0)
        );
    }
}

// Chrome Soaring Slash Cancel
unsafe fn soaring_slash_cancel(fighter: &mut L2CFighterCommon) {
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2)
    && 27.0 < frame && frame < 30.0
    && fighter.is_button_on(Buttons::Guard)
    {
        if VarModule::is_flag(fighter.battle_object, vars::common::SOARING_SLASH_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::common::UP_SPECIAL_CANCEL);
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        } else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        }
    }
}

// Side Special Cancels
unsafe fn side_special_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4])
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    {
        return;
    }

    let transition_air = match MotionModule::motion_kind(fighter.module_accessor) {
        utils::hash40!("special_s3_hi") | utils::hash40!("special_air_s3_hi") if fighter.is_cat_flag(Cat1::AttackHi3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_hi") | utils::hash40!("special_air_s3_hi") if fighter.is_cat_flag(Cat1::AttackHi4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_s") | utils::hash40!("special_air_s3_s") if fighter.is_cat_flag(Cat1::AttackS3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_s") | utils::hash40!("special_air_s3_s") if fighter.is_cat_flag(Cat1::AttackS4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_lw") | utils::hash40!("special_air_s3_lw") if fighter.is_cat_flag(Cat1::AttackLw3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_lw") | utils::hash40!("special_air_s3_lw") if fighter.is_cat_flag(Cat1::AttackLw4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                return;
            }

            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s4_hi") | utils::hash40!("special_air_s4_hi") if fighter.is_input_jump() => {
            if fighter.is_situation(*SITUATION_KIND_AIR)
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
            {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                return;
            } 

            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                return;
            }

            false
        }
        _ => false
    };

    if transition_air {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
    }
}

// Soaring Slash Hit
unsafe fn soaring_slash(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3
    ])
    {
        VarModule::off_flag(fighter.battle_object, vars::common::SOARING_SLASH_HIT);
    }

    if fighter.is_status(*FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3) {
        return;
    }

    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, vars::common::SOARING_SLASH_HIT);
    }
}

// symbol-based call for the fe characters' common opff
extern "Rust" {
    fn fe_common(fighter: &mut L2CFighterCommon);
}

#[utils::macros::opff(FIGHTER_KIND_CHROM )]
pub unsafe fn chrom_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    
    soaring_slash_drift(fighter);
    soaring_slash_cancel(fighter);
    side_special_cancels(fighter);
    soaring_slash(fighter);
}