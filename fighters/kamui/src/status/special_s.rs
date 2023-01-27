use super::*;


pub fn install() {
    smashline::install_status_scripts!(
        special_s_attack
    );
}

#[status_script(agent = "kamui", status = FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}