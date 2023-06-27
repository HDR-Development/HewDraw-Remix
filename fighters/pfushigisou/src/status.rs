use super::*;
use globals::*;

#[skyline::hook(replace=request_change_pokemon)]
unsafe fn request_change_pokemon() -> bool {
    return false;
}

#[skyline::hook(offset = 0xf96310)]
unsafe fn stub_death_switch() {}

pub fn install() {
    install_status_scripts!(
    );
    skyline::install_hooks!(
        stub_death_switch,
    );
}
