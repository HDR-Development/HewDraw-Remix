use super::*;

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter, statuses::samusd::FLOAT.into())
}

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_main(fighter, statuses::samusd::FLOAT.into())
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}
