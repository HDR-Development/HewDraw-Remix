use super::*;

// FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY


unsafe extern "C" fn heavy_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("item_heavy_throw_b") {
        PostureModule::reverse_lr(fighter.module_accessor);
    }

    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY)(fighter);
}


pub fn install() {
    smashline::Agent::new("donkey")
        .status(End, *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, heavy_throw_end)
        .install();
}
