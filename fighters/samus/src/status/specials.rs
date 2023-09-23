use super::*;

#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s1g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s1a_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s2g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s2a_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_s1g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_s1a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_s2g_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_s2a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_color(fighter)
}


unsafe extern "C" fn special_s_color(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION) {
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
    }
    else if is_ice(fighter)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
        suit_effect(fighter.module_accessor,fighter.battle_object);
    }
    0.into()
}


pub fn install() {
    install_status_scripts!(
        special_s1g_end,
        special_s1a_end,
        special_s2g_end,
        special_s2a_end,

        special_s1g_exec,
        special_s1a_exec,
        special_s2g_exec,
        special_s2a_exec,
    );
}