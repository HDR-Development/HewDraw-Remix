// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn heros_bow_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.2, y: 1.0, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword2"), &long_sword_scale);
}

// Prevents Toon Link from being able to use both aerial jumps immediately after one another
unsafe fn triple_jump_lockout(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        let triple_jump_lockout_frame = ParamModule::get_int(fighter.object(), ParamType::Agent, "triple_jump_lockout_frame");
        if fighter.global_table[CURRENT_FRAME].get_i32() < triple_jump_lockout_frame {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        }
    }
}

// symbol-based call for the links' common opff
extern "Rust" {
    fn links_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && ( fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST
        ])
        || (fighter.is_motion(Hash40::new("special_air_hi")) && fighter.motion_frame() > 49.0) )
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
    heros_bow_drift(boma, status_kind, situation_kind, cat[1], stick_y);
	sword_length(boma);
    triple_jump_lockout(fighter);
    fastfall_specials(fighter);
    

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if motion_kind == hash40("attack_11") {
            if frame < 6.0 {
                MotionModule::set_rate(boma, 1.667);
            }
            if frame >= 6.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_TOONLINK )]
pub fn toonlink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		toonlink_frame(fighter);
        links_common(fighter);
    }
}

pub unsafe fn toonlink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}