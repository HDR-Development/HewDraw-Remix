use super::*;
mod fighter_status_guard;
mod guard;
mod guard_damage;
mod guard_off;
mod guard_on;
mod misc;

pub fn install() {
    //fighter_status_guard::install();
    //guard::install();
    guard_damage::install();
    guard_off::install();
    //guard_on::install();
    misc::install();
}