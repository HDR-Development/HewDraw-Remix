// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


pub unsafe fn morphball_crawl(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) {
        if frame >= 31.0 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
            }
        }
    }
}

// pub unsafe fn manual_detonate(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
//     if fighter.is_cat_flag(Cat1::SpecialLw) && VarModule::is_flag(boma, vars::samusd::MANUAL_DETONATE_READY) {
        
//     }
// }

pub unsafe fn remove_super_missiles(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, false);
    }
    else if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, false);
    }
}

pub unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}
 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if motion_kind == hash40("attack_air_b") {
            if frame >= 11.0 && frame < 15.0 {
                MotionModule::set_rate(boma, 0.4);
            }
            if frame >= 15.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

pub unsafe extern "Rust" fn common_samusd(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        morphball_crawl(&mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
        //remove_super_missiles(&mut *info.boma, info.status_kind);
    }
}

#[utils::macros::opff(FIGHTER_KIND_SAMUSD )]
pub fn samusd_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		samusd_frame(fighter);
        common_samusd(fighter);
    }
}

pub unsafe fn samusd_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}