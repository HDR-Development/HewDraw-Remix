// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn holy_water_ac(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 20.0 {
            boma.check_airdodge_cancel();
        }
    }
}

unsafe fn axe_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Simon Cross Land Cancel
unsafe fn cross_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat2: i32, stick_y: f32) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2]) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR)
        && fighter.status_frame() >= 19 {
            // Current FAF in motion list is 42, frame is 0 indexed so subtract a frame
            let special_s1_cancel_frame_ground = 41.0;
            // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
            let landing_lag = 11.0;
            if MotionModule::frame(fighter.module_accessor) < (special_s1_cancel_frame_ground - landing_lag) {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, special_s1_cancel_frame_ground - landing_lag, true, true, false);
            }
            LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// allow fair and bair to transition into their angled variants when the stick is angled up/down
unsafe fn whip_angling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32, stick_y: f32) {
    if fighter.is_motion_one_of(&[Hash40::new("attack_air_f"), Hash40::new("attack_air_f_hi"), Hash40::new("attack_air_f_lw")])
    && (11.0..12.0).contains(&frame) {
        if stick_y > 0.5 { // stick is held up
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_f_hi"), -1.0, 1.0, 0.0, false, false);
        } else if stick_y < -0.5 { // stick is held down
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_f_lw"), -1.0, 1.0, 0.0, false, false);
        }
    } 
    else if fighter.is_motion_one_of(&[Hash40::new("attack_air_b"), Hash40::new("attack_air_b_hi"), Hash40::new("attack_air_b_lw")])
    && (11.0..12.0).contains(&frame) {
        if stick_y > 0.5 { // stick is held up
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_b_hi"), -1.0, 1.0, 0.0, false, false);
        } else if stick_y < -0.5 { // stick is held down
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_b_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
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
    holy_water_ac(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    axe_drift(boma, status_kind, situation_kind, cat[1], stick_y);
    cross_land_cancel(fighter, boma, cat[2], stick_y);
    whip_angling(fighter, boma, frame, stick_y);
    fastfall_specials(fighter);
}

pub extern "C" fn simon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		simon_frame(fighter)
    }
}

pub unsafe fn simon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install() {
    smashline::Agent::new("simon")
        .on_line(Main, simon_frame_wrapper)
        .install();
}
