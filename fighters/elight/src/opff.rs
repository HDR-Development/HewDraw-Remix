// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


 
unsafe fn hit_cancel_blade_switch(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if (fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP
    ]) || fighter.is_motion(Hash40::new("attack_13")))
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.is_cat_flag(Cat1::SpecialLw)
    && !fighter.is_in_hitlag() {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

unsafe fn photon_edge_actionability(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD) 
       && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
       && VarModule::get_int(fighter.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID) == 0 {
        VarModule::on_flag(fighter.battle_object, vars::elight::instance::ENABLE_SPECIAL_S_ACTIONABILITY);
    }
    if fighter.is_status(*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD){
        // Allow canceling before reappearing, but still after teleporting forward
        // frame 25 (more with hitlag) felt the best both visualy and for semi combo utility
        // If this cancel check isnt here, the cancel itself wouldnt line up with the visual and would just feel cumbersome
        if fighter.status_frame() >= 15 && VarModule::is_flag(fighter.battle_object, vars::elight::instance::ENABLE_SPECIAL_S_ACTIONABILITY){
            CancelModule::enable_cancel(fighter.boma());
        }
    }
    if fighter.is_status(*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END) {
        // Allow cancel afterwards if ur bad and delay your jump input for the rest of the end animation
        if VarModule::is_flag(fighter.battle_object, vars::elight::instance::ENABLE_SPECIAL_S_ACTIONABILITY){
            CancelModule::enable_cancel(fighter.boma());
        }
        // Disables drifting when stick is held back back
        if PostureModule::lr(fighter.boma()) * ControlModule::get_stick_x(fighter.boma()) < 0.0 {
            KineticModule::unable_energy(fighter.boma(), *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
	}
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END,
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

#[utils::macros::opff(FIGHTER_KIND_ELIGHT )]
pub unsafe fn elight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    hit_cancel_blade_switch(fighter);
    photon_edge_actionability(fighter);
    fastfall_specials(fighter);
}
