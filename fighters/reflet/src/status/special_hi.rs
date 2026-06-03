use super::*;

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::reflet::instance::SPECIAL_HI_ENABLE_FREEFALL);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(End, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL, special_hi_end);
}
