use super::*;

// FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn heavy_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("item_heavy_throw_b") {
        PostureModule::reverse_lr(fighter.module_accessor);
    }

    return original!(fighter);
}

pub fn install() {
    install_status_scripts!(
        heavy_throw_end,
    );
}