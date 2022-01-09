use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;

use crate::utils::hdr;

use crate::vars::popo_jc_grab;

//=================================================================
//== SHIELD STOPS
//=================================================================
unsafe fn shield_stop(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    let player_number = hdr::get_player_number(boma);

    if (status_kind == *FIGHTER_STATUS_KIND_DASH) || (status_kind == *FIGHTER_STATUS_KIND_TURN_DASH) {
        if ( ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) ) {
            // If Nana and Popo is JC grabbing, don't shield stop
            if !(fighter_kind == *FIGHTER_KIND_NANA && popo_jc_grab[player_number]) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                ControlModule::clear_command(boma, true);
            }
        }
    }
}

//=================================================================
//== SHIELD DROPS
//=================================================================
unsafe fn shield_drop(boma: &mut BattleObjectModuleAccessor, cat2: i32, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
        // Check if shield drop input has been taken
        let shield_drop_inputs = [
            *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS,
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI,
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW,
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L,
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R
        ];
        if shield_drop_inputs.iter().any(|x| hdr::compare_cat(cat2, *x)) {
            if GroundModule::is_passable_ground(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    // shield_stop(boma, cat[0], status_kind, fighter_kind);
    shield_drop(boma, cat[1], status_kind);
}
