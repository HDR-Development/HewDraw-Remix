use super::*;

#[skyline::from_offset(0x932f50)]
unsafe extern "C" fn demon_on_attack_original(vtable: u64, fighter: &mut Fighter, log: u64);

// FighterVtableSlot::OnAttack
unsafe extern "C" fn demon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) {
    let battle_object = &mut fighter.battle_object;
    let module_accessor = battle_object.module_accessor;
    let kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let status = StatusModule::status_kind(module_accessor);
    if [
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3,
    ].contains(&status)
    && VarModule::is_flag(battle_object, vars::demon::status::CHECK_STEP_CANCEL) {
        let collision_log: &mut CollisionLog = std::mem::transmute(log as *mut u64);
        if [
            *COLLISION_KIND_ATTACK as u8,
            *COLLISION_KIND_HIT as u8,
            *COLLISION_KIND_SHIELD as u8,
        ].contains(&collision_log.collision_kind) {
            VarModule::on_flag(battle_object, vars::demon::status::ENABLE_STEP_CANCEL);
        }
    }

    demon_on_attack_original(vtable, fighter, log)
}

// FighterVtableSlot::OnGrab
// Returns the event untouched so the game's own handling is skipped.
unsafe extern "C" fn demon_on_grab(_vtable: u64, _fighter: &mut Fighter, event: u64) -> u64 {
    event
}

pub fn install() {
    // The following disables the reversed stick values when autoturn runs
    let _ = skyline::patching::Patch::in_text(0x934a6c).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7d1c).nop();

    // Disables Reverse Special Command calls
    let _ = skyline::patching::Patch::in_text(0x934a4c).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7cfc).nop();

    replace_fighter_vtable(
        *FIGHTER_KIND_DEMON,
        FighterVtableSlot::OnAttack,
        demon_on_attack as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_DEMON,
        FighterVtableSlot::OnGrab,
        demon_on_grab as *const () as _
    );
}
