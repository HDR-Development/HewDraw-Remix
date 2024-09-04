
// #[skyline::hook(offset=0x6a7100)] // unadjusted for 13.0.2?
// pub fn stub_kill_screen() {}

pub fn install() {
    skyline::install_hooks!(
        //stub_kill_screen,
    );
}