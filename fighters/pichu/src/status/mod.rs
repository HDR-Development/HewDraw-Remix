use super::*;
use globals::*;
// status script import
mod special_hi;
mod special_s;

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::pichu::instance::SPECIAL_LW_DISCHARGE_AIR_START);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);

    special_hi::install(agent);
    special_s::install(agent);
}