use super::*;
use globals::*;


#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    println!("change status from: {}", StatusModule::status_kind(boma));
    
    println!("next status: {}", next_status);
    original!()(boma, next_status, arg3)
}

pub fn install() {
    skyline::install_hooks!(
        //change_status_request_from_script_hook
    );
}