use super::*;
use utils::ext::*;

static mut IS_STOP: bool = false;

#[skyline::hook(offset = 0x3a7f90, inline)]
unsafe fn battle_object__process_begin_sub(ctx: &skyline::hooks::InlineCtx) {
    IS_STOP = *ctx.registers[19].w.as_ref() & 1 != 0;
}

#[skyline::hook(offset = 0x2606270)]
unsafe fn cameramanager__update_frame(camera_manager: *mut *mut u64) {
    call_original!(camera_manager);
    if !IS_STOP {
        call_original!(camera_manager);
    }
}

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
        cameramanager__update_frame,
        cameramanager__update
    );
}