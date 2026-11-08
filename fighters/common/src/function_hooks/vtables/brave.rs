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

// #[skyline::hook(offset = 0x853e10)]
// pub unsafe fn psyche_up_hit() {
//     // do nothing
// }

#[skyline::from_offset(0x853e10)]
extern "C" fn remove_psyche_up(fighter: &mut Fighter);

#[skyline::hook(offset = 0x854520)]
pub unsafe extern "C" fn brave_on_attack(vtable: u64, battleObject: *mut BattleObject, collisionLog: CollisionLog) -> u64 {
    let boma = &mut (*(&mut *(battleObject)).module_accessor);
    if boma.is_motion_one_of(&[Hash40::new_raw(0xc1a0567e3), Hash40::new_raw(0x10a24f50e9)]) {
        if [0x3, 0x2d, 0x31].contains(&(&mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id))).kind()) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && sv_math::rand(hash40("fighter"), 100) == 1 {
                StatusModule::change_status_request(&mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id)), 0xb5, false);
            }
        }
    }

    return call_original!(vtable, battleObject, collisionLog);
}

pub fn install() {
    // Removes a Psyche Up check
    //skyline::patching::Patch::in_text(0x8542ec).data(0x14000010u32);

    skyline::install_hooks!(
        hero_rng_hook,
        brave_on_attack,
        //psyche_up_hit
    );
}