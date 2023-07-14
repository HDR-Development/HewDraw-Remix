// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn aerial_wiz_foot_jump_reset_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == jump_count_max {
                WorkModule::set_int(boma, jump_count_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
}

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

// Repeated Warlock Punch turnaround
// unsafe fn repeated_warlock_punch_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
//     if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN {
//         if frame > 30.0 && frame < 45.0 {
//             if stick_x * facing < 0.0 {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
//             }
//         }
//     }
// }

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    aerial_wiz_foot_jump_reset_bounce(boma, status_kind, situation_kind);
    // dtaunt_counter(boma, motion_kind, frame);
    // repeated_warlock_punch_turnaround(boma, status_kind, stick_x, facing, frame);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_GANON )]
pub fn ganon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ganon_frame(fighter)
    }
}

pub unsafe fn ganon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}