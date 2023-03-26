use super::*;

// FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn heavy_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("item_heavy_throw_b") {
        PostureModule::reverse_lr(fighter.module_accessor);
    }

    if MotionModule::motion_kind(fighter.boma()) == hash40("item_heavy_throw_lw") 
        && fighter.motion_frame() < 3.0 
    {
        let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        fighter.set_speed(Vector2f::new(speed_x, 10.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    return original!(fighter);
}

pub fn install() {
    install_status_scripts!(
        heavy_throw_end
    );
}