use super::*;
use globals::*;

#[skyline::hook(replace=WorkModule::is_flag)]
unsafe fn is_flag_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if boma.is_fighter() {
        if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN {
            return true;
        }
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        is_flag_hook
    );
}