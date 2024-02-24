use super::*;
use globals::*;


// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK

#[status_script(agent = "inkling", status = FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN

#[status_script(agent = "inkling", status = FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        special_s_walk,
        special_s_run,
    );
}