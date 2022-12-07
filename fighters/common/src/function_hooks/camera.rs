use super::*;
use utils::ext::*;


static mut IS_STOP: bool = false;

// Keeps track of hitlag
#[skyline::hook(offset = 0x3a7f74, inline)]
unsafe fn battle_object__process_begin_sub(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor;
    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    IS_STOP = *(stop_module as *const bool).add(0x3c);
}

// Doubles camera speed while not in hitlag
#[skyline::hook(offset = 0x4f0a80)]
unsafe fn cameramanager__update(camera_manager: *mut *mut u64) {
    call_original!(camera_manager);
    if !IS_STOP {
        call_original!(camera_manager);
    }
}

pub fn install() {
    skyline::install_hooks!(
        battle_object__process_begin_sub,
        cameramanager__update
    );
}