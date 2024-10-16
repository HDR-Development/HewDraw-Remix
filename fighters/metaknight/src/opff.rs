// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn dimensional_cape_early_attack(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
    && boma.status_frame() > 11
    && compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, false);
    }
}

/// this cancels side special early if you hit the opponent
unsafe fn drill_rush_on_hit_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH)
    && AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        // Allows MK to rebound off of shield
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
        // Transition to rebound
        fighter.change_status_req(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, false);
    }
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END)
    && AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.object(), vars::metaknight::instance::SPECIAL_S_HIT);
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 0.85, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}				 

unsafe fn fspecial_once_per_airtime(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
    }
}

unsafe fn transition_fall(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL)
    && (boma.is_prev_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END)
    && VarModule::is_flag(boma.object(), vars::metaknight::instance::SPECIAL_S_HIT)) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
}

unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP])
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    && fighter.status_frame() > 20 {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }
}

unsafe fn special_lw_landing_lag(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END)
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && !boma.is_prev_situation(*SITUATION_KIND_GROUND) {
        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    dimensional_cape_early_attack(boma);
    special_lw_landing_lag(boma);
    sword_length(boma);
    transition_fall(boma);
    up_special_proper_landing(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn metaknight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		metaknight_frame(fighter);
    }
}

pub unsafe fn metaknight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
        drill_rush_on_hit_cancel(fighter);
        fspecial_once_per_airtime(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, metaknight_frame_wrapper);
}