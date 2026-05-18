// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn heros_bow_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let mut long_sword_scale = Vector3f{x: 1.175, y: 1.0, z: 1.0};
    if boma.is_motion_one_of(&[
        Hash40::new("attack_air_hi"),
        //Hash40::new("attack_s4"),
        Hash40::new("special_air_hi"),
        Hash40::new("special_hi"),
        Hash40::new("special_hi_start")])
    { // match sword size better w/o ton of extra range
        long_sword_scale.x = 1.1375;
    }
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &long_sword_scale);
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
        || (fighter.is_motion(Hash40::new("special_air_hi")) && fighter.motion_frame() > 63.0) )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    heros_bow_drift(fighter);
	sword_length(boma);
    triple_jump_lockout(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn toonlink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		toonlink_frame(fighter);
    }
}

pub unsafe fn toonlink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, toonlink_frame_wrapper);
}