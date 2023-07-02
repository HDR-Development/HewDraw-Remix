// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn paralyzer_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_N_SHOOT {
        if frame > 7.0 {
            boma.check_dash_cancel();
        }
    }
}

// ZSS Flip Jump - Jump Cancel and Flipstool Handilng
unsafe fn flip_jump_jc_flipstool(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
        || motion_kind == hash40("special_lw_start")
        || motion_kind == hash40("special_air_lw_start") {
        if frame > 20.0 {
            if !boma.is_in_hitlag() {
                boma.check_jump_cancel(false);
            }
        }
        // Turn on the vanilla flip jump footstool-enable flag if you're holding the special button and you're in the window to be able to flipstool manually
        if VarModule::is_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE){
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
                WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
            }
            else{
                WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
            }
        }
    }
}

// Transitions ZSS into Flip Jump's footstool rebund animation upon connecting with dair
unsafe fn dair_rebound(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) && fighter.is_motion(Hash40::new("attack_air_lw")) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !fighter.is_in_hitlag(){
            VarModule::on_flag(fighter.battle_object, vars::szerosuit::status::ATTACK_AIR_LW_REBOUND);
            fighter.change_status_req(*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, true);
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    //paralyzer_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    flip_jump_jc_flipstool(boma, status_kind, motion_kind, cat[0], frame);
    dair_rebound(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_SZEROSUIT )]
pub fn szerosuit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		szerosuit_frame(fighter)
    }
}

pub unsafe fn szerosuit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}