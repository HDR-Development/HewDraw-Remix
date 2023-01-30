// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Squirtle Withdraw Actionability On-Hit
unsafe fn withdraw_jc(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    /*
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END].contains(&status_kind)
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame > 10.0 {
    */
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT].contains(&status_kind) && frame >= 13.0 && !boma.is_in_hitlag() {
        //boma.check_jump_cancel(true);
        CancelModule::enable_cancel(boma);
    }

}