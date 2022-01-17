use super::*;
use globals::*;


#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, next_status: i32, arg3: bool) -> u64 {
    if boma.is_fighter() {
        if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&StatusModule::status_kind(boma)) && !CancelModule::is_enable_cancel(boma) {
            if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&next_status) {
                return 0;
            }
        }
    }
    original!()(boma, next_status, arg3)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_request_from_script_hook
    );
}