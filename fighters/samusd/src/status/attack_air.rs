use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter, statuses::reflet::FLOAT.into())
}

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_main(fighter, statuses::reflet::FLOAT.into())
}

pub fn install() {
    smashline::Agent::new("samusd")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main)
        .install();
}
