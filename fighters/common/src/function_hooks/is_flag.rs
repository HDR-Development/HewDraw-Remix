use super::*;
use globals::*;

#[skyline::hook(replace=WorkModule::is_flag)]
unsafe fn is_flag_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET {
        return false;
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        //is_flag_hook
    );
}