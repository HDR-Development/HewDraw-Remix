// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn peanut_popgun_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT && frame > 5.0 {
        boma.check_airdodge_cancel();
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N || status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Allows Diddy to be actionable once hitstun is over, after being knocked out of upB
// rather than having to wait until the end of the knockback animation
unsafe fn up_special_knockback_canceling(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE]) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) && MotionModule::rate(fighter.module_accessor) != 0.0 {
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
            let hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            if hitstun <= 0.0 {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
            }
            else if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
                fighter.FighterStatusDamage__check_smoke_effect();
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    peanut_popgun_ac(boma, status_kind, situation_kind, cat[0], frame);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    up_special_knockback_canceling(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_DIDDY )]
pub fn diddy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		diddy_frame(fighter)
    }
}

pub unsafe fn diddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}