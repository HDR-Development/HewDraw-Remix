use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0x831850)]
unsafe extern "C" fn bayonetta_set_bullet_arts_acmd(motion: u64, limb: i32, on_off: i32) -> u64 {
    let ret = original!()(motion, limb, on_off);
    let on = match on_off {
        1 => true,
        2 => false,
        _ => unreachable!()
    };
    let limb_str = match limb {
        0 => "arml",
        1 => "armr",
        2 => "legl",
        3 => "legr",
        _ => unreachable!()
    };
    // clear hitboxes
    if !on {
        return hash40("game_shootingoff_generic");
    }
    // run script for limb
    let acmd = format!("game_shootingon_generic_{}", limb_str);
    return hash40(acmd.as_str());
}

pub fn install() {
    skyline::install_hooks!(
        bayonetta_set_bullet_arts_acmd,
    );
}