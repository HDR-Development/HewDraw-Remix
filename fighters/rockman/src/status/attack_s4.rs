use super::*;

unsafe extern "C" fn rockman_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

pub fn install() {
    smashline::Agent::new("rockman")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, rockman_attack_s4_main)
        .install();
}
