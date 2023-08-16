// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


pub unsafe fn morphball_crawl(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) {
        if frame >= 32.0 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
            }
        }
    }
}

pub unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}
 
unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
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

pub unsafe fn remove_super_missiles(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, false);
    }
    else if boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, false);
    }
}
 
pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    remove_super_missiles(boma);
    fastfall_specials(fighter);
}

pub unsafe extern "Rust" fn common_samusd(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        morphball_crawl(&mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
    }
}

#[utils::macros::opff(FIGHTER_KIND_SAMUSD )]
pub fn samusd_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		samusd_frame(fighter);
        common_samusd(fighter);
    }
}

pub unsafe fn samusd_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

// #[smashline::weapon_frame(agent = WEAPON_KIND_SAMUSD_BOMB, main)]
// pub fn samusd_bomb_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
//     unsafe {
//         let boma = weapon.boma();
//         let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
//         // Ensure the boma's owner is Dark Samus.
//         if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUSD {
//             let dsamus = utils::util::get_battle_object_from_id(owner_id);
//             let dsamus_boma = &mut *(*dsamus).module_accessor;
//             if StatusModule::status_kind(boma) == *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL
//             && dsamus_boma.is_cat_flag(Cat1::SpecialLw)
//             && VarModule::is_flag(dsamus, vars::samusd::instance::MANUAL_DETONATE_READY) {
//                 if WorkModule::is_enable_transition_term_group(dsamus_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK)
//                     || WorkModule::is_enable_transition_term_group(dsamus_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK)
//                     || WorkModule::is_enable_transition_term_group(dsamus_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL)
//                     || WorkModule::is_enable_transition_term_group(dsamus_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
//                     StatusModule::change_status_request_from_script(boma, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK, false);
//                     VarModule::off_flag(dsamus, vars::samusd::instance::MANUAL_DETONATE_READY);
//                     dsamus_boma.clear_commands(Cat1::SpecialLw); // Clear down b command so Dark Samus doesn't immediately drop another bomb
//                 }
//             }
//         }
//     }
// }
