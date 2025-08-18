use super::*;

// Used for when generic weapons hit something else.
#[skyline::hook(offset = 0x33a8280)]
unsafe extern "C" fn weapon_attack_callback(weapon: *mut BattleObject, arg: u64) {
    if (*weapon).kind == *WEAPON_KIND_PICKEL_FISHINGROD as u32 {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(weapon, arg)
}

// Resets projectile lifetime on parry
#[skyline::hook(offset = 0x33bdff8, inline)]
unsafe extern "C" fn force_reflect_full_lifetime(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

pub fn install() {
    skyline::install_hooks!(
        weapon_attack_callback,
        force_reflect_full_lifetime
    );
}