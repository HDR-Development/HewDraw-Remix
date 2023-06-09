use super::*;
use globals::*;


pub fn install() {
    install_status_scripts!(
        mario_special_lw_shoot_main,
    );
}


#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn mario_special_lw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    return 0.into();

}
