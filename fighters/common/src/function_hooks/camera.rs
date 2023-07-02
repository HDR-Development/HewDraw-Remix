use super::*;
use utils::ext::*;


// Doubles camera speed
#[skyline::hook(offset = 0x4fdbf0)]
unsafe fn normal_camera(ptr: u64, float: f32) {
    call_original!(ptr, float);
    call_original!(ptr, float);
}

pub fn install() {
    skyline::install_hooks!(
        normal_camera
    );
}