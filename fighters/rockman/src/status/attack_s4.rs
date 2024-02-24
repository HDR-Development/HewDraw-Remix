use super::*;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rockman_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

pub fn install() {
    install_status_scripts!(
        rockman_attack_s4_main
    );
}