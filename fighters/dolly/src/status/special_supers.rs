use super::*;

pub unsafe extern "C" fn special_super_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::drain(fighter.battle_object, 2);
    smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL)(fighter)
}

pub unsafe extern "C" fn special_super2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::drain(fighter.battle_object, 2);
    smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, special_super_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, special_super2_main);
}