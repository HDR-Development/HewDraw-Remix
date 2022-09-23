// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn hit_cancel_blade_switch(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if (fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP
    ]) || fighter.is_motion(Hash40::new("attack_13")))
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.is_cat_flag(Cat1::SpecialLw)
    && !fighter.is_in_hitlag() {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

unsafe fn lightning_buster_cancel(boma: &mut BattleObjectModuleAccessor,  motion_kind: u64, frame: f32) {
    
    if (motion_kind == hash40("special_n") || motion_kind == hash40("special_air_n"))
        && frame > 10.0 && frame < 29.0 
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
    {
        if !boma.is_in_hitlag() {
            MotionModule::set_frame_sync_anim_cmd(boma, 29.0, true, true, false);   
        }
    }
}

unsafe fn photon_edge_shorten(boma: &mut BattleObjectModuleAccessor,  status_kind: i32) {

    if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD
        && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    {
        if !boma.is_in_hitlag() {
            boma.change_status_req(*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, false);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    hit_cancel_blade_switch(boma, cat[0], status_kind, situation_kind, motion_kind);
    lightning_buster_cancel(boma, motion_kind, frame);
    photon_edge_shorten(boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_ELIGHT )]
pub fn elight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		elight_frame(fighter);
    }
}

pub unsafe fn elight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
