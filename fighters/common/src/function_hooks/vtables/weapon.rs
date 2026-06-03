use super::*;

// Used for when generic weapons hit something else.
#[skyline::hook(offset = 0x33a8010)]
unsafe extern "C" fn weapon_attack_callback(weapon: *mut BattleObject, arg: u64) {
    if (*weapon).kind == *WEAPON_KIND_PICKEL_FISHINGROD as u32 {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(weapon, arg)
}

// Resets projectile lifetime on parry
#[skyline::hook(offset = 0x33bdd88, inline)]
unsafe extern "C" fn force_reflect_full_lifetime(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[8].set_x(0);
}

pub unsafe extern "C" fn tornadoshot_can_pocket(_vtable: u64, weapon: &mut smash::app::Weapon) -> bool {
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    status == 0
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51ed978).data(tornadoshot_can_pocket as *const () as u64);

    skyline::install_hooks!(
        weapon_attack_callback,
        force_reflect_full_lifetime
    );
}