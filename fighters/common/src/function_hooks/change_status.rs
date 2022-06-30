use super::*;
use globals::*;

#[skyline::hook(replace=StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND
        && (next_status == *FIGHTER_STATUS_KIND_CLIFF_CATCH || next_status == *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE || next_status == *FIGHTER_STATUS_KIND_CLIFF_WAIT)
        &&  VarModule::is_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER) {
            VarModule::off_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER);
            next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
        }
    }
    original!()(boma, next_status, arg3)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if boma.is_fighter() {
        // Clears buffer when sliding off an edge in a damaged state, to prevent accidental buffered aerials/airdodges (common on missed techs)
        if [*FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_SLIP_WAIT,
            *FIGHTER_STATUS_KIND_DAMAGE].contains(&StatusModule::status_kind(boma))
        && next_status == *FIGHTER_STATUS_KIND_FALL {
            clear_buffer = true;
        }

        if boma.kind() == *FIGHTER_KIND_TRAIL
        && StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
        && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
        && ((!VarModule::is_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT)
        && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
            || VarModule::is_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL)) { 
            next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
        }
        if boma.kind() == *FIGHTER_KIND_KOOPAJR
        && StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        }
    }
    original!()(boma, next_status, clear_buffer)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_request_hook,
        change_status_request_from_script_hook
    );
}