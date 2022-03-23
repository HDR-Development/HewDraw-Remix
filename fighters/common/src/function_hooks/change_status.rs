use super::*;
use globals::*;


#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter()
    && boma.kind() == *FIGHTER_KIND_TRAIL
    && StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
    && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
    && ((!VarModule::is_flag(boma.object(), vars::trail::IS_SIDE_SPECIAL_INPUT)
    && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
        || VarModule::is_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL)) { 
        next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
    }
    
    if StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
        next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
    }

    original!()(boma, next_status, arg3)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_request_from_script_hook
    );
}