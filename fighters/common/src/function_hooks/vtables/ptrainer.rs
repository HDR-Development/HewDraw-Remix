use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0x34ce694, inline)]
unsafe fn ptrainer_swap_backwards_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let object = ctx.registers[20].x() as *mut BattleObject;
    if VarModule::is_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH) {
        let new = match ctx.registers[8].x() {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => unreachable!()
        };

        ctx.registers[8].set_x(new);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH)
    }
}

// #[skyline::hook(offset = 0xf96330)]
// unsafe fn ptrainer_stub_death_switch() {}

#[skyline::from_offset(0x33bd9c0)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64;

#[skyline::hook(offset = 0x34d0a20)]
unsafe fn pzenigame_water_on_hit(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64 {
    let boma = (&mut *(weapon)).battle_object.boma();
    if !boma.is_status(*WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH) {
        if collision_mask as i32 & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) != 0 {
            boma.change_status_req(*WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH, false);
            return 0;
        }
    }
    else {
        return 0;
    }
    
    return normal_weapon_hit_handler(vtable, weapon, collision_mask);
}

#[skyline::hook(offset = 0x348d630)]
unsafe fn pfushigisou_seed_init(vtable: u64, weapon: *mut app::Weapon) -> u64 {
    // nothing
    return 0;
}

#[skyline::hook(offset = 0x348d6a0)]
unsafe fn pfushigisou_seed_on_hit(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64 {
    let boma = (&mut *(weapon)).battle_object.boma();
    if !boma.is_status_one_of(&[*WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND]) {
        if collision_mask as i32 & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) != 0 {
            boma.change_status_req(*WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, false);
            return 0;
        }
    }
    else {
        return 0;
    }
    
    return normal_weapon_hit_handler(vtable, weapon, collision_mask);
}

pub fn install() {
    // enables Bullet Seed to be reflected
    unsafe {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u64;
        let _ = skyline::patching::Patch::in_text(0x519f628).data(text.add(0x33b89d0 / 0x8));
    }

    skyline::install_hooks!(
        ptrainer_swap_backwards_hook,
        //ptrainer_stub_death_switch,
        pzenigame_water_on_hit,
        pfushigisou_seed_init,
        pfushigisou_seed_on_hit,
    );
}