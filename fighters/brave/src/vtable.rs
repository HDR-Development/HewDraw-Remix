use super::*;

#[skyline::from_offset(0x854520)]
unsafe extern "C" fn brave_on_attack_original(vtable: u64, fighter: *mut BattleObject, log: &mut CollisionLog) -> u64;

// FighterVtableSlot::OnAttack
unsafe extern "C" fn brave_on_attack(vtable: u64, fighter: *mut BattleObject, log: &mut CollisionLog) -> u64 {
    let boma = &mut *(*fighter).module_accessor;
    if boma.is_motion_one_of(&[Hash40::new_raw(0xc1a0567e3), Hash40::new_raw(0x10a24f50e9)]) {
        let opponent_boma = &mut *sv_battle_object::module_accessor(log.opponent_battle_object_id);
        if [0x3, 0x2d, 0x31].contains(&opponent_boma.kind()) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && sv_math::rand(hash40("fighter"), 100) == 1 {
                StatusModule::change_status_request(opponent_boma, 0xb5, false);
            }
        }
    }

    brave_on_attack_original(vtable, fighter, log)
}

pub fn install() {
    // Removes a Psyche Up check
    //skyline::patching::Patch::in_text(0x8542ec).data(0x14000010u32);

    replace_fighter_vtable(
        *FIGHTER_KIND_BRAVE,
        FighterVtableSlot::OnAttack,
        brave_on_attack as *const () as _
    );
}
