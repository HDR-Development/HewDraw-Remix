// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn peanut_popgun_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT && frame > 5.0 {
        boma.check_airdodge_cancel();
    }
}

unsafe fn nspecial_cancels(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_cat_flag(Cat2::StickEscape) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeF) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_F);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_B);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_GROUND_JUMP);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            if fighter.sub_check_command_guard().get_bool() {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_GUARD);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
        }
        else {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump)))
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
            {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL_JUMP, true, false);
            }
        }
    }
}

// Allows Diddy to be actionable once hitstun is over, after being knocked out of upB
// rather than having to wait until the end of the knockback animation
unsafe fn up_special_knockback_canceling(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE]) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) && MotionModule::rate(fighter.module_accessor) != 0.0 {
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
            let hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            if hitstun <= 0.0 {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
            }
            else if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
                fighter.FighterStatusDamage__check_smoke_effect();
            }
        }
    }
}

unsafe fn fastfall_dashattack(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_DASH])

        && fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.sub_air_check_dive();
            if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    
                    
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    app::sv_kinetic_energy::enable(fighter.lua_state_agent);
    
                    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
                }
            }
        }
    }


unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_DANGER,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_FALL,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP2,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL,
        *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_HIT_CEIL,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}
unsafe fn dash_attack_jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if StatusModule::is_changing(boma) {
        return;
    }

    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && situation_kind == *SITUATION_KIND_AIR
    && MotionModule::frame(boma) >= 21.0 {
        fighter.check_jump_cancel(false, false);
    }
}

unsafe fn dashattack_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH) {
        if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && boma.is_situation (*SITUATION_KIND_GROUND) {
                // Current FAF in motion list is 43, frame is 0 indexed so subtract a frame
          let dash_attack_cancel_frame_ground  = 42.0;
          // 8F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
          let landing_lag = 9.0;
          if MotionModule::frame(boma) < (dash_attack_cancel_frame_ground - landing_lag) {
              MotionModule::set_frame_sync_anim_cmd(boma, dash_attack_cancel_frame_ground - landing_lag, true, true, true);
          }
        }
    }
}


pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    peanut_popgun_ac(boma, status_kind, situation_kind, cat[0], frame);
    nspecial_cancels(fighter, boma, status_kind);
    up_special_knockback_canceling(fighter);
    dash_attack_jump_cancels(fighter, boma, status_kind, situation_kind);
    dashattack_land_cancel(boma);
    fastfall_specials(fighter);
    fastfall_dashattack(fighter);
}


#[utils::macros::opff(FIGHTER_KIND_DIDDY )]
pub fn diddy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		diddy_frame(fighter)
    }
}

pub unsafe fn diddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

