use super::*;

unsafe extern "C" fn item_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_ItemThrow()
} // remove vanilla rob up b -> attack mechanics

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ITEM_THROW, item_throw_pre);
}
