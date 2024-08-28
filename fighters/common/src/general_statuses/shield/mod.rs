use super::*;
mod fighter_status_guard;
mod furafura;
mod guard;
mod guard_damage;
mod guard_off;
mod guard_on;
mod shield_break_fly;
pub mod misc;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        fighter_status_guard::install();
        furafura::install();
        guard::install();
        guard_damage::install();
        guard_off::install();
        guard_on::install();
        shield_break_fly::install();
        misc::install();
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}
