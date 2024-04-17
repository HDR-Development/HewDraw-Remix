use super::*;

// FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY

unsafe extern "C" fn item_throw_heavy_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("item_heavy_throw_b") {
        PostureModule::reverse_lr(fighter.module_accessor);
    }

    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, item_throw_heavy_end);
}
