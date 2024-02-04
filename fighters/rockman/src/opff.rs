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
// unsafe fn jc_dtilt_hit(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
//     if boma.is_motion(Hash40::new("attack_lw3")) {
//         if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) && frame > 12.0 {
//             boma.check_jump_cancel(false, false);
//         }
//     }
// }

// Mega Man Metal Blad Toss Airdodge Cancel
unsafe fn blade_toss_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if boma.status_frame() > 16 {
            boma.check_airdodge_cancel();
        }
    }
}

// upB freefalls after one use per airtime
// unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
//     if StatusModule::is_changing(fighter.module_accessor)
//     && (fighter.is_situation(*SITUATION_KIND_GROUND)
//         || fighter.is_situation(*SITUATION_KIND_CLIFF)
//         || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]))
//     {
//         VarModule::off_flag(fighter.battle_object, vars::rockman::instance::UP_SPECIAL_FREEFALL);
//     }
//     if fighter.is_prev_status(*FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP) {
//         if StatusModule::is_changing(fighter.module_accessor) {
//             VarModule::on_flag(fighter.battle_object, vars::rockman::instance::UP_SPECIAL_FREEFALL);
//         }
//     }
//     if fighter.is_status(*FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP) {
//         if fighter.is_situation(*SITUATION_KIND_AIR)
//         && !StatusModule::is_changing(fighter.module_accessor)
//         && VarModule::is_flag(fighter.battle_object, vars::rockman::instance::UP_SPECIAL_FREEFALL) {
//             if CancelModule::is_enable_cancel(fighter.module_accessor) {
//                 fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
//                 let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
//                 *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
//             }
//         }
//     }
// }

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // light_utilt_cancel(boma, id, status_kind, situation_kind, cat[0], frame);
    // utilt_command_input(boma, id, status_kind, situation_kind, frame);
    // jc_dtilt_hit(boma, status_kind, situation_kind, cat[0], frame);
    blade_toss_ac(boma, status_kind, situation_kind, cat[0], frame);
    // up_special_freefall(fighter);
    fastfall_specials(fighter);
}


pub extern "C" fn rockman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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
pub fn install() {
    smashline::Agent::new("rockman")
        .on_line(Main, rockman_frame_wrapper)
        .install();
}
