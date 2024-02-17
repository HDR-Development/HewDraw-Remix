
#[skyline::hook(offset=0x6a70e0)]
pub fn stub_kill_screen() {}

pub fn install() {
    unsafe {
        // linear hitstun patch
        skyline::patching::Patch::in_text(0x62BA74).data(0xD2800000u32);
    }
    skyline::install_hooks!(
        //stub_kill_screen,
    );
}