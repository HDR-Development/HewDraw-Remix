// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn dancing_blade_vertical_momentum(boma: &mut BattleObjectModuleAccessor){
    let fighter_gravity = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut FighterKineticEnergyGravity;
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2]) && boma.is_situation(*SITUATION_KIND_AIR) {
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.06);
        smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -1.5);
    }

    if boma.is_situation(*SITUATION_KIND_GROUND) && VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }
}

// Fixes weird vanilla behavior where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
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
    dancing_blade_vertical_momentum(boma);
    up_special_proper_landing(fighter);
    fastfall_specials(fighter);
}


// symbol-based call for the fe characters' common opff
extern "Rust" {
    fn fe_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[utils::macros::opff(FIGHTER_KIND_MARTH )]
pub fn marth_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		marth_frame(fighter);
        fe_common(fighter);
    }
}

pub unsafe fn marth_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}