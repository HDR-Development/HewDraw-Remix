use super::*;
use globals::*;
use smashline::*;

pub fn install() {
    install_status_scripts!(
        init_special_s,
        init_special_s_command,
        init_special_s_loop
    );
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND //

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP //

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
    }
    original!(fighter)
}