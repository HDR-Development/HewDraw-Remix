// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

// Fox Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END])
    && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false, false);
        }
}   

// Utaunt cancel into Fire Fox
unsafe fn utaunt_cancel_fire_fox(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_hi_l") {
        if frame > 41.0 && frame < 44.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
            }
        }
    }
}

unsafe fn firefox_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firefox startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND,

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

    laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    shine_jump_cancel(fighter);
    utaunt_cancel_fire_fox(boma, motion_kind, frame);
    firefox_startup_ledgegrab(fighter);
    fastfall_specials(fighter);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("throw_hi") {
        if frame >= 10.0 {
            MotionModule::set_rate(boma, 1.8);
        }
    }
}

pub extern "C" fn fox_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		fox_frame(fighter)
    }
}

pub unsafe fn fox_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install() {
    smashline::Agent::new("fox")
        .on_line(Main, fox_frame_wrapper)
        .install();
}
