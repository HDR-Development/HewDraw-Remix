use super::*;

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_LandingAttackAir()
}

pub fn install() {
    smashline::install_status_scripts!(
        landing_attack_air_pre
    );
}
