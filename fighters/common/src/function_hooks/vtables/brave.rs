use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave23special_lw_open_commandERNS_7FighterE"]
    fn special_lw_open_command();
}

extern "C" {
    #[link_name = "hero_rng_hook_impl"]
    fn hero_rng_hook_impl(fighter: *mut BattleObject);
}

#[skyline::hook(replace = special_lw_open_command)]
pub unsafe fn hero_rng_hook(fighter: *mut BattleObject) {
    hero_rng_hook_impl(fighter);
}

#[skyline::hook(offset = 0x853e10)]
pub unsafe fn psych_up_hit() {
    // do nothing
}

pub fn install() {
    skyline::install_hooks!(
        hero_rng_hook,
        psych_up_hit
    );
}