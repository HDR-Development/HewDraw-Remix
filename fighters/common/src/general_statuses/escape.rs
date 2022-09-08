use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_end_EscaleFB_hook,
        );
    }
}

// this runs as you leave hitlag
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_status_end_EscaleFB)]
pub unsafe fn sub_status_end_EscaleFB_hook(fighter: &mut L2CFighterCommon) {
}