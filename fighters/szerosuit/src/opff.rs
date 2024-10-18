// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// ZSS Flip Jump - Jump Cancel and Flipstool Handling
unsafe fn flip_jump_jc_flipstool(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
    || boma.is_motion_one_of(&[Hash40::new("special_lw_start"), Hash40::new("special_air_lw_start")]) {
        if boma.status_frame() > 21 && !boma.is_in_hitlag() {
            boma.check_jump_cancel(false, false);
        }
        // Turn on the vanilla flip jump footstool-enable flag if you're holding the special button and you're in the window to be able to flipstool manually
        if VarModule::is_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
            }
            else {
                WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
            }
        }
    }
}

// Transitions ZSS into Flip Jump's footstool rebound animation upon connecting with dair
unsafe fn dair_rebound(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_lw"))
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.is_in_hitlag() {
        VarModule::on_flag(fighter.battle_object, vars::szerosuit::status::ATTACK_AIR_LW_REBOUND);
        fighter.change_status_req(*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, true);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::on_flag(fighter.battle_object, vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_N_SHOOT_H
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
    flip_jump_jc_flipstool(boma);
    dair_rebound(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn szerosuit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		szerosuit_frame(fighter)
    }
}

pub unsafe fn szerosuit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, szerosuit_frame_wrapper);
}