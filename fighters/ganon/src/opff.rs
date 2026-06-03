// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Dtaunt Counter
// unsafe fn dtaunt_counter(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
//     if [hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&motion_kind)
//         && frame >= 29.0 && frame <= 59.0 {
//         if FighterStopModuleImpl::is_damage_stop(boma) {
//             if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
//                 DamageModule::add_damage(boma, 100.0, 0);
//                 WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
//                 StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
//             }
//         }
//     }
// }

unsafe fn special_hi_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.motion_frame() >= 14.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
}

unsafe fn wizards_foot_jump_refresh(fighter: &mut L2CFighterCommon) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&fighter.status())
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        let jump_count_max = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        if fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == jump_count_max {
            fighter.set_int(jump_count_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    // dtaunt_counter(boma, motion_kind, frame);
    special_hi_landing(fighter);
    wizards_foot_jump_refresh(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn ganon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ganon_frame(fighter)
    }
}

pub unsafe fn ganon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ganon_frame_wrapper);
}