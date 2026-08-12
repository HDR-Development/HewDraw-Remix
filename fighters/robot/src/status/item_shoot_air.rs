use super::*;

unsafe extern "C" fn item_shoot_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_ItemShootAir_New()
} // remove vanilla rob up b -> attack mechanics

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR, item_shoot_air_pre);
}
