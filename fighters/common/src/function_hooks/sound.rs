use super::*;
use globals::*;

#[skyline::hook(replace=smash::app::sv_animcmd::PLAY_LANDING_SE)]
unsafe fn PLAY_LANDING_SE_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if boma.is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        return;
    }

    original!()(lua_state);
}

#[skyline::hook(replace=smash::app::sv_animcmd::PLAY_SE)]
unsafe fn PLAY_SE_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if boma.is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        return;
    }

    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        PLAY_LANDING_SE_hook,
        PLAY_SE_hook,
    );
}