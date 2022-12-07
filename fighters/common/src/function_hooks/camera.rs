use super::*;
use utils::ext::*;

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
        cameramanager__update_frame,
        cameramanager__update
    );
}