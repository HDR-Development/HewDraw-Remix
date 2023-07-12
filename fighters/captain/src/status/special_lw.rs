use super::*;
use globals::*;


#[status_script(agent = "captain", status = FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_lw_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "captain", status = FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    }

    ret
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_end_init,
        special_lw_end_main
    );
}
