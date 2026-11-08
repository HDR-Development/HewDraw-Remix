use super::*;

#[skyline::from_offset(0xc077e0)]
unsafe extern "C" fn krool_on_damage_original(vtable: u64, fighter: *mut Fighter, log: u64) -> u64;

// FighterVtableSlot::OnDamage
unsafe extern "C" fn krool_on_damage(vtable: u64, fighter: *mut Fighter, log: u64) -> u64 {
    let boma = (*fighter).battle_object.boma();
    let opponent_id = *(*(log as *mut u64).add(0x10 / 0x8) as *mut u32).add(0x44 / 0x4);
    let opponent_boma = &mut *sv_battle_object::module_accessor(opponent_id);

    let ret = krool_on_damage_original(vtable, fighter, log);

    if opponent_boma.is_item() && opponent_boma.kind() == *ITEM_KIND_KROOLCROWN
    && boma.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
        StatusModule::change_status_request(opponent_boma, *ITEM_STATUS_KIND_DEAD, false);
        EffectModule::req_follow(boma, Hash40::new("sys_item_get"), Hash40::new("crown1"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
        boma.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN);
    }

    ret
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_KROOL,
        FighterVtableSlot::OnDamage,
        krool_on_damage as *const () as _
    );
}
