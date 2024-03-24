// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// knife land cancel
unsafe fn knife_lc(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    && VarModule::is_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL)
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        // remove the unthrown knife from richter's hand
        if (2.0..13.0).contains(&boma.motion_frame())
        && ArticleModule::is_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE){
            ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        let landing_lag = 10.0; // amount of frames until richter can act when landing
        let rate = 27.0 / landing_lag;
        MotionModule::change_motion(boma, Hash40::new("landing_fall_special"), 0.0, rate, false, 0.0, false, false);
        VarModule::off_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL);
        EffectModule::kill_kind(boma, Hash40::new("sys_sp_flash"), true, true);
    }
}

// allow fair and bair to transition into their angled variants when the stick is angled up/down
unsafe fn whip_angling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32, stick_y: f32) {
    let stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        ControlModule::get_sub_stick_y(fighter.module_accessor)
    }
    else {
        ControlModule::get_stick_y(fighter.module_accessor)
    };
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
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2
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
    knife_lc(boma);
    whip_angling(fighter, boma, frame, stick_y);
    fastfall_specials(fighter);
}

pub extern "C" fn richter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		richter_frame(fighter)
    }
}

pub unsafe fn richter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install() {
    smashline::Agent::new("richter")
        .on_line(Main, richter_frame_wrapper)
        .install();
}
