// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, backdash_energy});
use super::*;
use globals::*;

 
unsafe fn slaughter_high_kick_devastator(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) && motion_kind == hash40("attack_hi3") {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP){
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4){
               if boma.is_stick_backward() && !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::SLAUGHTER_HIGH_KICK);
                    boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5, false);
               }
               if boma.is_stick_forward() && !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::DEVASTATOR);
                    boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, false);
               }
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::SLAUGHTER_HIGH_KICK);
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::DEVASTATOR);
    }
}

unsafe fn jaw_breaker(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if [*FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind)
        && frame > 17.0 {
        if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N){
            VarModule::on_flag(boma.object(), vars::demon::instance::JAW_BREAKER);
            boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        }
    }
    if ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::JAW_BREAKER);
    }
}
unsafe fn lightning_screw_uppercut(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_stand_21") {
        if frame < 18.0{
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON) {
                VarModule::on_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT);
            }
        }
        else{
            if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_22"), 0.0, 1.2, 0.0);
            }
        }
    }
    if motion_kind == hash40("attack_stand_22") && frame > 15.0 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_23"), 0.0, 1.15, 0.0);
        }
    }
    if motion_kind == hash40("attack_stand_23") && frame > 15.0 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
            boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, false);
        }
    }
    if ![hash40("attack_stand_21"), hash40("attack_stand_22"), hash40("attack_stand_23"), hash40("attack_step_2l")].contains(&motion_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT);
    }
}

unsafe fn spinning_demon(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_step_2s") {
        if frame > 15.0 && frame < 17.0{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(boma.object(), vars::demon::instance::SPINNING_DEMON);
            }
        }
        else if frame >= 17.0{
            if VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON){
                boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, false);
            }
        }
    }
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2) && VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON) && motion_kind == hash40("attack_stand_21"){
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_24"), 0.0, 1.0, 0.0);
    }
    if ![hash40("attack_stand_21"), hash40("attack_stand_24"), hash40("attack_step_2s")].contains(&motion_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::SPINNING_DEMON);
    }
}

unsafe fn korean_back_dash(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, stick_y: f32) {
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    && boma.left_stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"))
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
    ])
    && compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH)
    {
        boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK, false);
    }
}

unsafe fn enable_both_recovery_specials(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_LW_FALL]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    }
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) && !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    slaughter_high_kick_devastator(boma, cat[0], status_kind, situation_kind, motion_kind);
    jaw_breaker(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    korean_back_dash(boma, cat[0], status_kind, stick_y);
    lightning_screw_uppercut(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    spinning_demon(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    enable_both_recovery_specials(boma);
    common::opff::backdash_energy(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_DEMON )]
pub fn demon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		demon_frame(fighter)
    }
}

pub unsafe fn demon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}