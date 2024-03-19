use super::*;

#[skyline::hook(replace = FighterUtil::get_just_shield_se)]
unsafe extern "C" fn get_just_shield_se(fighter_kind: i32) -> u64 {
    match fighter_kind {
        0x3c => hash40("se_ryu_guard_just"),
        0x3d => hash40("se_ken_guard_just"),
        0x40 => hash40("se_bayonetta_final03"),
        0x5b => hash40("se_elight_escapeforesight01"),
        _ => hash40("se_common_justshield")
    }
}

pub fn install() {
    skyline::install_hooks!(
        get_just_shield_se
    );
}