use super::*;
use globals::*;

// #[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
// unsafe fn gamewatch_special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     original!(fighter);
//     0.into()
// }

pub fn install() {
    install_status_scripts!(
        //gamewatch_special_s_init
    );
}