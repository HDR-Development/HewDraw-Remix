use super::*;

mod special_hi;
mod special_lw;
mod special_n;

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);

    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
}