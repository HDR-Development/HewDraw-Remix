// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


#[no_mangle]
pub unsafe extern "Rust" fn pits_common(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    power_of_flight_cancel(boma, status_kind);
    upperdash_arm_whiff_freefall(fighter);
    fastfall_specials(fighter);
}


// Pits Power of Flight cancel
unsafe fn power_of_flight_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
        }
    }
}
 
unsafe fn upperdash_arm_jump_and_aerial_cancel(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) || (boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END) && boma.status_frame() > 6) {
        if (boma.is_situation(*SITUATION_KIND_GROUND) || WorkModule::is_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF))
        && boma.status_frame() > 28 {
            boma.check_jump_cancel(true, false);
        }
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        let start_frame = if boma.is_situation(*SITUATION_KIND_GROUND) { 16 } else { 19 };
        if (start_frame..35).contains(&boma.status_frame()) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT);
        }
    }
}

unsafe fn upperdash_arm_whiff_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END])
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && MotionModule::frame(fighter.module_accessor) >= MotionModule::end_frame(fighter.module_accessor) - 1.0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END
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
    upperdash_arm_jump_and_aerial_cancel(boma);
    pits_common(fighter, boma, status_kind);
}


pub extern "C" fn pit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pit_frame(fighter)
    }
}

pub unsafe fn pit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install() {
    smashline::Agent::new("pit")
        .on_line(Main, pit_frame_wrapper)
        .install();
}
