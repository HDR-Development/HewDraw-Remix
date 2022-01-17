
#[skyline::hook(offset=0x6a70e0)]
pub fn stub_kill_screen() {}

pub fn install() {
    unsafe {
        // linear hitstun patch
        skyline::patching::patch_data(0x62ba54, &0xD2800000u32);
    }
    skyline::install_hooks!(
        //stub_kill_screen,
    );
}