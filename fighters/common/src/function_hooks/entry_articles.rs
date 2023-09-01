use smash::lib::{*, lua_const::*};

#[skyline::hook(offset = 0x3a6650)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    let barrel_kind = *WEAPON_KIND_DONKEY_DKBARREL;
    if weapon_kind == barrel_kind{
        return 1;
    }
    call_original!(weapon_kind, entry_id)
}

pub fn install() {
    skyline::install_hooks!(
        get_article_use_type_mask,
    );
}