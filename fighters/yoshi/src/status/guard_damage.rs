use super::*;

// FIGHTER_STATUS_KIND_GUARD_DAMAGE

pub unsafe extern "C" fn guard_damage_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}

pub unsafe extern "C" fn guard_damage_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_exitStatus_common();
    smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}

pub unsafe extern "C" fn guard_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardDamage_common(L2CValue::Bool(false));
    fighter.sub_shift_status_main(L2CValue::Ptr(
        L2CFighterCommon_status_GuardDamage_Main as *const () as _,
    ))
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_init);
    agent.status(Exit, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_exit);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_main);
}
