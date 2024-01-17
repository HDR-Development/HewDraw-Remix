
#[skyline::hook(offset=0x6a70e0)]
pub fn stub_kill_screen() {}

pub fn install() {
    skyline::install_hooks!(
        //stub_kill_screen,
    );
}