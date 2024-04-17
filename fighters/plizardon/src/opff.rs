// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    let prev_situation = StatusModule::prev_situation_kind(boma);
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N || situation_kind != *SITUATION_KIND_GROUND || prev_situation != *SITUATION_KIND_AIR {
        return;
    }
    if StatusModule::is_changing(boma) {
        return;
    }
    if frame < 19.0 {
        MotionModule::set_frame(boma, 18.0, true);
    }
}

unsafe fn flare_blitz_edge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END) {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END,
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
    flame_cancel(boma, status_kind, situation_kind, frame);
    flare_blitz_edge_cancel(fighter);
    fastfall_specials(fighter);
    special_lw_track(boma);

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if motion_kind == hash40("attack_air_n") {
            if frame < 8.0 {
                MotionModule::set_rate(boma, 1.75);
            }
            if frame >= 8.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

pub extern "C" fn plizardon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		plizardon_frame(fighter)
    }
}

pub unsafe fn plizardon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, plizardon_frame_wrapper);
}
