use super::*;

// Used for when generic weapons hit something else.
#[skyline::hook(offset = 0x33a8260)]
unsafe extern "C" fn weapon_attack_callback(weapon: *mut BattleObject, arg: u64) {
    if (*weapon).kind == *WEAPON_KIND_PICKEL_FISHINGROD as u32 {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(weapon, arg)
}

pub fn install() {
    skyline::install_hooks!(
        weapon_attack_callback
    );
}