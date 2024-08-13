use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "krool_belly_damage_hook_impl"]
    fn krool_belly_damage_hook_impl(damage: f32, fighter: *mut Fighter, unk: bool);
}

// #[skyline::hook(offset = 0xc050d8, inline)]
// pub unsafe fn krool_belly_toggle_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     krool_belly_toggle_hook_impl(ctx);
// }

#[skyline::hook(offset = 0xc055f0)]
pub unsafe fn krool_belly_damage_hook(damage: f32, fighter: *mut Fighter, unk: bool) {
    krool_belly_damage_hook_impl(damage, fighter, unk);
}

pub fn install() {
    skyline::install_hooks!(
        krool_belly_damage_hook
    );
}