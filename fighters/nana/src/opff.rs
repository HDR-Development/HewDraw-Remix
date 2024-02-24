// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Ice Climbers Cheer Cancel (Techy)
unsafe fn cheer_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_POPO_STATUS_KIND_THROW_NANA) {
        MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}
 
// symbol-based call for the shotos' common opff
extern "Rust" {
    fn ice_climbers_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // nothing lol
}

pub fn cheer_cancel_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        cheer_cancel(fighter);
    }
}

pub extern "C" fn nana_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		nana_frame(fighter);
        ice_climbers_common(fighter);
        cheer_cancel_wrapper(fighter);
    }
}

pub unsafe fn nana_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install() {
    smashline::Agent::new("nana")
        .on_line(Main, nana_frame_wrapper)
        .install();
}