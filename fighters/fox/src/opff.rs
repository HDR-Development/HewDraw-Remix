// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn laser_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        fighter.check_land_cancel(None);
    }
}

// Fox Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    // disables jump cancels when parried between statuses
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT
    ])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        VarModule::on_flag(fighter.battle_object, vars::fox::instance::SPECIAL_LW_DISABLE_JC);
        if !fighter.is_status(*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END)
        && !fighter.is_in_hitlag() {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        }
    }

    if fighter.is_status_one_of(&[
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END
    ])
    && !fighter.is_in_hitlag()
    && !VarModule::is_flag(fighter.battle_object, vars::fox::instance::SPECIAL_LW_DISABLE_JC) {
        fighter.check_jump_cancel(false, false, false);
    }
}   

// Utaunt cancel into Fire Fox
unsafe fn utaunt_cancel_fire_fox(boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_motion_one_of(&[Hash40::new("appeal_hi_l"), Hash40::new("appeal_hi_r")]) 
    && (41.0..44.0).contains(&frame) 
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
    }
}

unsafe fn firefox_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firefox startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    laser_land_cancel(fighter);
    shine_jump_cancel(fighter);
    utaunt_cancel_fire_fox(boma, frame);
    firefox_startup_ledgegrab(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn fox_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		fox_frame(fighter)
    }
}

pub unsafe fn fox_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.motion_kind.hash, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, fox_frame_wrapper);
}
