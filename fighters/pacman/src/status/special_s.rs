use super::*;
use globals::*;


#[status_script(agent = "pacman", status = FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    // Disables super armor on sideB Power Pellet consumption
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    ret
}

pub fn install() {
    install_status_scripts!(
        special_s_main
    );
}