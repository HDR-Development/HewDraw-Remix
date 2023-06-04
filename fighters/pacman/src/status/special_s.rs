use super::*;
use globals::*;


// FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH

#[status_script(agent = "pacman", status = FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    // Disables super armor on sideB Power Pellet consumption
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    ret
}

// FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN

#[status_script(agent = "pacman", status = FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_return_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    ret
}

pub fn install() {
    install_status_scripts!(
        special_s_dash_main,
        special_s_return_main
    );
}