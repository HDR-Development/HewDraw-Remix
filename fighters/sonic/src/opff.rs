// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Jump during Spin Turn
unsafe fn sonic_spindash_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32){
if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN && boma.is_input_jump() {
    StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP, true);
  }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP,
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    sonic_spindash_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    //sonic_moveset(boma, situation_kind, status_kind, motion_kind, frame, cat[0], id);
    //sonic_lightspeed_dash(boma, status_kind, motion_kind, situation_kind, cat[0], id);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_SONIC )]
pub fn sonic_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		sonic_frame(fighter)
    }
}

pub unsafe fn sonic_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

