use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    if !fighter.global_table[SUB_STATUS].get_bool() {
        // allow fast fall during float release aerials
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_fall_common_uniq(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_fall_common_uniq as *const () as _));
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}