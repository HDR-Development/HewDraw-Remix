use super::*;

#[no_mangle]
unsafe extern "C" fn demon_on_attack_inner(_vtable: u64, fighter: &mut Fighter, log: u64) {
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
}
