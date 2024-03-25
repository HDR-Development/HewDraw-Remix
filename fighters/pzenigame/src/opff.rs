// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Squirtle Withdraw JC
unsafe fn withdraw_jc(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    /*
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END].contains(&status_kind)
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame > 11.0 {
    */
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT].contains(&status_kind)
    && !StatusModule::is_changing(boma)
    && frame >= 13.0
    && !boma.is_in_hitlag() {
        //boma.check_jump_cancel(true, false);
        CancelModule::enable_cancel(boma);
    }
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) && boma.status_frame() > 15 && !boma.is_in_hitlag() {
        boma.check_jump_cancel(true, false);
    }

    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && boma.status_frame() < 10 && !boma.is_in_hitlag() {
        boma.check_jump_cancel(true, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END,
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

unsafe fn special_lw_track(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && !boma.is_button_on(Buttons::SpecialAll) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::off_flag(object, vars::ptrainer::instance::IS_SWITCH_BACKWARDS);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    withdraw_jc(boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    fastfall_specials(fighter);
    special_lw_track(boma);
}

pub extern "C" fn pzenigame_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pzenigame_frame(fighter)
    }
}
pub unsafe fn pzenigame_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pzenigame_frame_wrapper);
}
