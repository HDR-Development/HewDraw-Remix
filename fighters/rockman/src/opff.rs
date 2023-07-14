// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// unsafe fn utilt_command_input(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
//     if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND) {
//         WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
//     }
//     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
//         VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
//     }
    
//     if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3)
//     || [*FIGHTER_STATUS_KIND_WAIT,
//         *FIGHTER_STATUS_KIND_DASH,
//         *FIGHTER_STATUS_KIND_TURN,
//         *FIGHTER_STATUS_KIND_TURN_DASH,
//         *FIGHTER_STATUS_KIND_RUN,
//         *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) {
//         if boma.is_cat_flag(Cat4::Command623A) {
//             StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
//             VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
//         }
//     }
// }

// unsafe fn light_utilt_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
//     if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && frame >= 32.0 {
//         if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
//             // StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
//         } else {
//             CancelModule::enable_cancel(boma);
//         }
//     }
// }

// Jump cancel dtilt on hit
unsafe fn jc_dtilt_hit(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if boma.is_motion(Hash40::new("attack_lw3")) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) && frame > 12.0 {
            boma.check_jump_cancel(false);
        }
    }
}

// Mega Man Metal Blad Toss Airdodge Cancel
unsafe fn blade_toss_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if boma.status_frame() > 16 {
            boma.check_airdodge_cancel();
        }
    }
}

unsafe fn side_special_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && situation_kind == *SITUATION_KIND_AIR {
        if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // light_utilt_cancel(boma, id, status_kind, situation_kind, cat[0], frame);
    // utilt_command_input(boma, id, status_kind, situation_kind, frame);
    jc_dtilt_hit(boma, status_kind, situation_kind, cat[0], frame);
    blade_toss_ac(boma, status_kind, situation_kind, cat[0], frame);
    side_special_drift(boma, status_kind, situation_kind, cat[1], stick_y);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_ROCKMAN )]
pub fn rockman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		rockman_frame(fighter)
    }
}

pub unsafe fn rockman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}