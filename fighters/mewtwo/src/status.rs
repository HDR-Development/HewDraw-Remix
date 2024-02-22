use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 

// FIGHTER_STATUS_KIND_ATTACK_AIR //

pub unsafe extern "C" fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

pub fn install() {
    smashline::Agent::new("mewtwo")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air)
        .install();
}